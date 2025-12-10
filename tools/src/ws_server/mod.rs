//! WebSocket server for browser extension communication
//!
//! Architecture:
//! - Server listens on localhost:9222 (configurable)
//! - Chrome extension connects via WebSocket
//! - CLI sends commands to server, server forwards to extension
//! - Extension executes in browser, returns response

pub mod protocol;

use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message};

pub use protocol::*;

/// Server state shared across connections
pub struct ServerState {
    /// Connected extension (only one allowed)
    extension_tx: RwLock<Option<mpsc::Sender<String>>>,

    /// Pending requests waiting for response
    pending_requests: RwLock<HashMap<u64, tokio::sync::oneshot::Sender<Response>>>,

    /// Request ID counter
    next_id: RwLock<u64>,

    /// Broadcast channel for server shutdown
    #[allow(dead_code)]
    shutdown_tx: broadcast::Sender<()>,
}

impl ServerState {
    pub fn new() -> (Arc<Self>, broadcast::Receiver<()>) {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        (
            Arc::new(Self {
                extension_tx: RwLock::new(None),
                pending_requests: RwLock::new(HashMap::new()),
                next_id: RwLock::new(1),
                shutdown_tx,
            }),
            shutdown_rx,
        )
    }

    /// Send command to extension and wait for response
    pub async fn send_command(&self, command: Command) -> Result<Response, String> {
        let extension_tx = self.extension_tx.read().await;
        let tx = extension_tx
            .as_ref()
            .ok_or_else(|| "No extension connected".to_string())?
            .clone();
        drop(extension_tx);

        // Get request ID
        let id = {
            let mut next_id = self.next_id.write().await;
            let id = *next_id;
            *next_id += 1;
            id
        };

        // Create oneshot channel for response
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();

        // Store pending request
        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(id, resp_tx);
        }

        // Send request
        let request = Request { id, command };
        let json = serde_json::to_string(&request).map_err(|e| e.to_string())?;
        tx.send(json)
            .await
            .map_err(|_| "Failed to send to extension".to_string())?;

        // Wait for response with timeout
        let response = tokio::time::timeout(std::time::Duration::from_secs(30), resp_rx)
            .await
            .map_err(|_| "Extension response timeout".to_string())?
            .map_err(|_| "Response channel closed".to_string())?;

        Ok(response)
    }

    /// Handle incoming response from extension
    pub async fn handle_response(&self, msg: ResponseMessage) {
        let mut pending = self.pending_requests.write().await;
        if let Some(tx) = pending.remove(&msg.id) {
            let _ = tx.send(msg.response);
        }
    }

    /// Check if extension is connected
    #[allow(dead_code)]
    pub async fn is_extension_connected(&self) -> bool {
        self.extension_tx.read().await.is_some()
    }
}

/// Start the WebSocket server
pub async fn start_server(port: u16) -> Result<(), String> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    println!("WebSocket server listening on ws://{}", addr);
    println!("Waiting for Chrome extension to connect...");

    let (state, mut shutdown_rx) = ServerState::new();

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((stream, addr)) => {
                        println!("New connection from: {}", addr);
                        let state_clone = state.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_connection(stream, state_clone).await {
                                eprintln!("Connection error: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Accept error: {}", e);
                    }
                }
            }
            _ = shutdown_rx.recv() => {
                println!("Server shutting down...");
                break;
            }
        }
    }

    Ok(())
}

/// Client type for identification
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientIdentification {
    client_type: String,
}

/// Handle a single WebSocket connection
async fn handle_connection(stream: TcpStream, state: Arc<ServerState>) -> Result<(), String> {
    let ws_stream = accept_async(stream)
        .await
        .map_err(|e| format!("WebSocket handshake failed: {}", e))?;

    let (ws_tx, mut ws_rx) = ws_stream.split();

    // Wait for first message to determine client type
    let first_msg = match ws_rx.next().await {
        Some(Ok(Message::Text(text))) => text,
        Some(Err(e)) => {
            return Err(format!("Error receiving first message: {}", e));
        }
        _ => {
            return Err("Connection closed before identification".to_string());
        }
    };

    // Check if this is an extension or CLI connection
    if let Ok(ident) = serde_json::from_str::<ClientIdentification>(&first_msg) {
        if ident.client_type == "extension" {
            return handle_extension_connection(ws_tx, ws_rx, state).await;
        }
    }

    // Not an extension - treat as CLI request
    handle_cli_connection(first_msg, ws_tx, state).await
}

/// Handle extension connection (long-lived, bidirectional)
async fn handle_extension_connection(
    mut ws_tx: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<TcpStream>,
        Message,
    >,
    mut ws_rx: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>,
    state: Arc<ServerState>,
) -> Result<(), String> {
    // Create channel for sending messages to this connection
    let (msg_tx, mut msg_rx) = mpsc::channel::<String>(32);

    // Register as extension connection
    {
        let mut extension_tx = state.extension_tx.write().await;
        if extension_tx.is_some() {
            println!("Warning: Replacing existing extension connection");
        }
        *extension_tx = Some(msg_tx.clone());
    }

    println!("Extension connected!");

    // Spawn task to forward messages to WebSocket
    let forward_task = tokio::spawn(async move {
        while let Some(msg) = msg_rx.recv().await {
            if ws_tx.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Process incoming messages from extension
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Check for keep-alive messages
                if text.contains("\"type\":\"keepAlive\"") {
                    continue;
                }

                // Try to parse as response
                match serde_json::from_str::<ResponseMessage>(&text) {
                    Ok(response) => {
                        state.handle_response(response).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse extension message: {} - {}", text, e);
                    }
                }
            }
            Ok(Message::Close(_)) => {
                println!("Extension disconnected");
                break;
            }
            Ok(Message::Ping(data)) => {
                let _ = msg_tx
                    .send(format!("pong:{}", String::from_utf8_lossy(&data)))
                    .await;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Cleanup
    {
        let mut extension_tx = state.extension_tx.write().await;
        *extension_tx = None;
    }

    forward_task.abort();
    println!("Extension connection closed");

    Ok(())
}

/// Handle CLI connection (short-lived, request-response)
async fn handle_cli_connection(
    first_msg: String,
    mut ws_tx: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<TcpStream>,
        Message,
    >,
    state: Arc<ServerState>,
) -> Result<(), String> {
    // Parse the request
    let request: Request = serde_json::from_str(&first_msg)
        .map_err(|e| format!("Failed to parse CLI request: {} - {}", first_msg, e))?;

    println!("CLI request: {:?}", request.command);

    // Forward command to extension and wait for response
    let response = state.send_command(request.command).await;

    // Send response back to CLI
    let response_msg = match response {
        Ok(resp) => ResponseMessage {
            id: request.id,
            response: resp,
        },
        Err(e) => ResponseMessage {
            id: request.id,
            response: Response::Error { message: e },
        },
    };

    let json = serde_json::to_string(&response_msg).map_err(|e| e.to_string())?;
    ws_tx
        .send(Message::Text(json))
        .await
        .map_err(|e| e.to_string())?;

    println!("CLI connection closed");
    Ok(())
}

/// CLI client to connect to the server and send commands
pub async fn send_command_to_server(port: u16, command: Command) -> Result<Response, String> {
    use tokio_tungstenite::connect_async;

    let url = format!("ws://127.0.0.1:{}", port);
    let (ws_stream, _) = connect_async(&url)
        .await
        .map_err(|e| format!("Failed to connect to server at {}: {}", url, e))?;

    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    // Send command
    let request = Request { id: 1, command };
    let json = serde_json::to_string(&request).map_err(|e| e.to_string())?;
    ws_tx
        .send(Message::Text(json))
        .await
        .map_err(|e| e.to_string())?;

    // Wait for response
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let response: ResponseMessage =
                    serde_json::from_str(&text).map_err(|e| e.to_string())?;
                return Ok(response.response);
            }
            Ok(Message::Close(_)) => {
                return Err("Connection closed before response".to_string());
            }
            Err(e) => {
                return Err(format!("WebSocket error: {}", e));
            }
            _ => {}
        }
    }

    Err("No response received".to_string())
}

//! MCP (Model Context Protocol) server implementation
//!
//! Provides browser automation tools to Claude Code via stdio JSON-RPC.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

use crate::ws_server::{self, Command, Response};

/// MCP JSON-RPC request
#[derive(Debug, Deserialize)]
struct McpRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

/// MCP JSON-RPC response
#[derive(Debug, Serialize)]
struct McpResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
}

#[derive(Debug, Serialize)]
struct McpError {
    code: i32,
    message: String,
}

/// Tool definition for MCP
#[derive(Debug, Serialize)]
struct Tool {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

/// Run the MCP server (stdio-based)
pub async fn run_mcp_server(ws_port: u16) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    eprintln!("[MCP] Server starting...");

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[MCP] Read error: {}", e);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        eprintln!("[MCP] Received: {}", line);

        let request: McpRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[MCP] Parse error: {}", e);
                continue;
            }
        };

        let response = handle_request(request, ws_port).await;

        let response_json = serde_json::to_string(&response).unwrap();
        eprintln!("[MCP] Sending: {}", response_json);

        writeln!(stdout, "{}", response_json).unwrap();
        stdout.flush().unwrap();
    }
}

async fn handle_request(request: McpRequest, ws_port: u16) -> McpResponse {
    let id = request.id.unwrap_or(Value::Null);

    match request.method.as_str() {
        "initialize" => McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "kavik-browser",
                    "version": "0.1.0"
                }
            })),
            error: None,
        },

        "tools/list" => McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "tools": get_tools()
            })),
            error: None,
        },

        "tools/call" => {
            let tool_name = request.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let arguments = request.params.get("arguments").cloned().unwrap_or(json!({}));

            match call_tool(tool_name, arguments, ws_port).await {
                Ok(result) => McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(json!({
                        "content": [{
                            "type": "text",
                            "text": result
                        }]
                    })),
                    error: None,
                },
                Err(e) => McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(McpError {
                        code: -32000,
                        message: e,
                    }),
                },
            }
        }

        "notifications/initialized" => {
            // No response needed for notifications
            McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!(null)),
                error: None,
            }
        }

        _ => McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(McpError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
            }),
        },
    }
}

fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "browser_screenshot".to_string(),
            description: "Take a screenshot of the current browser tab".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        Tool {
            name: "browser_resize".to_string(),
            description: "Resize the browser viewport. Use preset names (mobile, tablet, desktop, large) or custom width/height.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "preset": {
                        "type": "string",
                        "description": "Preset name: mobile (375x667), tablet (768x1024), desktop (1024x768), large (1440x900)",
                        "enum": ["mobile", "tablet", "desktop", "large"]
                    },
                    "width": {
                        "type": "integer",
                        "description": "Custom viewport width in pixels"
                    },
                    "height": {
                        "type": "integer",
                        "description": "Custom viewport height in pixels"
                    }
                },
                "required": []
            }),
        },
        Tool {
            name: "browser_navigate".to_string(),
            description: "Navigate to a URL".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "URL to navigate to"
                    }
                },
                "required": ["url"]
            }),
        },
        Tool {
            name: "browser_click".to_string(),
            description: "Click on an element by CSS selector".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "selector": {
                        "type": "string",
                        "description": "CSS selector of the element to click"
                    }
                },
                "required": ["selector"]
            }),
        },
        Tool {
            name: "browser_status".to_string(),
            description: "Check browser extension connection status".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        Tool {
            name: "browser_reload".to_string(),
            description: "Reload the current page".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
    ]
}

/// Viewport presets
const PRESETS: &[(&str, u32, u32)] = &[
    ("mobile", 375, 667),
    ("tablet", 768, 1024),
    ("desktop", 1024, 768),
    ("large", 1440, 900),
];

fn get_preset(name: &str) -> Option<(u32, u32)> {
    PRESETS
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, w, h)| (*w, *h))
}

async fn call_tool(name: &str, args: Value, ws_port: u16) -> Result<String, String> {
    let command = match name {
        "browser_screenshot" => Command::Screenshot,

        "browser_resize" => {
            let (width, height) = if let Some(preset) = args.get("preset").and_then(|v| v.as_str())
            {
                get_preset(preset).ok_or_else(|| format!("Unknown preset: {}", preset))?
            } else {
                let width = args
                    .get("width")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u32)
                    .ok_or("width required if no preset")?;
                let height = args
                    .get("height")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u32)
                    .ok_or("height required if no preset")?;
                (width, height)
            };
            Command::Resize { width, height }
        }

        "browser_navigate" => {
            let url = args
                .get("url")
                .and_then(|v| v.as_str())
                .ok_or("url required")?
                .to_string();
            Command::Navigate { url }
        }

        "browser_click" => {
            let selector = args
                .get("selector")
                .and_then(|v| v.as_str())
                .ok_or("selector required")?
                .to_string();
            Command::Click { selector }
        }

        "browser_status" => Command::GetStatus,

        "browser_reload" => Command::Reload,

        _ => return Err(format!("Unknown tool: {}", name)),
    };

    let response = ws_server::send_command_to_server(ws_port, command).await?;

    match response {
        Response::Screenshot { base64 } => {
            // Return as base64 image that Claude can interpret
            Ok(format!(
                "Screenshot captured successfully. Base64 PNG data ({} bytes):\n{}",
                base64.len(),
                base64
            ))
        }
        Response::Status {
            connected,
            page_url,
            viewport_width,
            viewport_height,
        } => {
            let mut status = format!("Connected: {}", connected);
            if let Some(url) = page_url {
                status.push_str(&format!("\nPage URL: {}", url));
            }
            if let (Some(w), Some(h)) = (viewport_width, viewport_height) {
                status.push_str(&format!("\nViewport: {}x{}", w, h));
            }
            Ok(status)
        }
        Response::Success { data } => {
            if let Some(d) = data {
                Ok(format!("Success: {}", serde_json::to_string_pretty(&d).unwrap_or_default()))
            } else {
                Ok("Success".to_string())
            }
        }
        Response::Html { html } => Ok(format!("HTML:\n{}", html)),
        Response::Pong => Ok("Pong!".to_string()),
        Response::Error { message } => Err(message),
    }
}

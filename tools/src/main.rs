//! kavik-tools: Browser automation CLI for kavik.cz development
//!
//! Provides WebSocket server and Chrome extension coordination for:
//! - Screenshots
//! - Viewport resizing
//! - Navigation
//! - Visual debugging

mod commands;
mod mcp;
mod ws_server;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "kavik-tools")]
#[command(about = "Browser automation tools for kavik.cz", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the WebSocket server
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },
    /// Run as MCP server (for Claude Code integration)
    Mcp {
        /// WebSocket server port
        #[arg(long, default_value = "9222")]
        port: u16,
    },
    /// Browser management
    Browser {
        #[command(subcommand)]
        action: BrowserAction,
    },
    /// Execute browser commands
    Exec {
        /// WebSocket server port
        #[arg(long, default_value = "9222")]
        port: u16,

        #[command(subcommand)]
        action: ExecAction,
    },
}

#[derive(Subcommand)]
enum ServerAction {
    /// Start the WebSocket server
    Start {
        /// Port to listen on
        #[arg(long, default_value = "9222")]
        port: u16,
    },
}

#[derive(Subcommand)]
enum BrowserAction {
    /// Launch Chromium with the extension
    Launch {
        /// URL to open
        #[arg(long, default_value = "http://localhost:4321")]
        url: String,

        /// WebSocket server port
        #[arg(long, default_value = "9222")]
        ws_port: u16,

        /// Run headless
        #[arg(long)]
        headless: bool,

        /// Path to Chromium binary
        #[arg(long)]
        browser: Option<PathBuf>,
    },
    /// Kill all automation browser instances
    Kill,
    /// Check if Chromium is available
    Check,
}

#[derive(Subcommand)]
enum ExecAction {
    /// Check extension connection status
    Status,
    /// Ping the extension
    Ping,
    /// Take a screenshot
    Screenshot {
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Resize the viewport
    Resize {
        /// Width in pixels
        #[arg(long)]
        width: Option<u32>,

        /// Height in pixels
        #[arg(long)]
        height: Option<u32>,

        /// Preset: mobile, tablet, desktop, large, astro-breakpoint
        #[arg(long)]
        preset: Option<String>,
    },
    /// Navigate to a URL
    Navigate {
        /// URL to navigate to
        url: String,
    },
    /// Click an element
    Click {
        /// CSS selector
        selector: String,
    },
    /// Click at coordinates
    ClickAt {
        /// X coordinate
        x: i32,
        /// Y coordinate
        y: i32,
    },
    /// Scroll the page
    Scroll {
        /// Absolute Y position
        #[arg(long)]
        y: Option<i32>,

        /// Delta to scroll
        #[arg(long)]
        delta: Option<i32>,

        /// Scroll to bottom
        #[arg(long)]
        to_bottom: bool,
    },
    /// Get page HTML
    Html {
        /// CSS selector (default: whole page)
        #[arg(long)]
        selector: Option<String>,
    },
    /// Reload the page
    Reload,
    /// Detach CDP debugger
    Detach,
}

/// Viewport presets
const PRESETS: &[(&str, u32, u32)] = &[
    ("mobile", 375, 667),
    ("tablet", 768, 1024),
    ("desktop", 1024, 768),
    ("large", 1440, 900),
    ("astro-breakpoint", 800, 600), // 50em breakpoint
];

fn get_preset(name: &str) -> Option<(u32, u32)> {
    PRESETS
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, w, h)| (*w, *h))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Server { action } => match action {
            ServerAction::Start { port } => ws_server::start_server(port).await,
        },
        Commands::Mcp { port } => {
            mcp::run_mcp_server(port).await;
            Ok(())
        }
        Commands::Browser { action } => match action {
            BrowserAction::Launch {
                url,
                ws_port,
                headless,
                browser,
            } => {
                let opts = commands::browser::LaunchOptions {
                    url,
                    ws_port,
                    headless,
                    browser_path: browser,
                };

                match commands::browser::launch_browser(opts) {
                    Ok(mut child) => {
                        // Wait for extension to connect
                        match commands::browser::wait_for_extension_connection(
                            ws_port,
                            Duration::from_secs(30),
                        )
                        .await
                        {
                            Ok(()) => {
                                println!("Browser ready! Press Ctrl+C to exit.");
                                // Wait for browser to exit
                                let _ = child.wait();
                                Ok(())
                            }
                            Err(e) => {
                                let _ = child.kill();
                                Err(e)
                            }
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            BrowserAction::Kill => commands::browser::kill_browser_instances(),
            BrowserAction::Check => {
                match commands::browser::check_chromium_available() {
                    Ok(path) => {
                        println!("Chromium found: {}", path.display());
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
        },
        Commands::Exec { port, action } => handle_exec(port, action).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn handle_exec(port: u16, action: ExecAction) -> Result<(), String> {
    use ws_server::{send_command_to_server, Command, Response};

    let command = match action {
        ExecAction::Status => Command::GetStatus,
        ExecAction::Ping => Command::Ping,
        ExecAction::Screenshot { output } => {
            let response = send_command_to_server(port, Command::Screenshot).await?;
            match response {
                Response::Screenshot { base64 } => {
                    let data = base64::Engine::decode(
                        &base64::engine::general_purpose::STANDARD,
                        &base64,
                    )
                    .map_err(|e| format!("Failed to decode base64: {}", e))?;

                    let path = output.unwrap_or_else(|| PathBuf::from("screenshot.png"));
                    std::fs::write(&path, data)
                        .map_err(|e| format!("Failed to write screenshot: {}", e))?;
                    println!("Screenshot saved to: {}", path.display());
                    return Ok(());
                }
                Response::Error { message } => return Err(message),
                _ => return Err("Unexpected response".to_string()),
            }
        }
        ExecAction::Resize {
            width,
            height,
            preset,
        } => {
            let (w, h) = if let Some(preset_name) = preset {
                get_preset(&preset_name).ok_or_else(|| {
                    format!(
                        "Unknown preset: {}. Available: {}",
                        preset_name,
                        PRESETS
                            .iter()
                            .map(|(n, _, _)| *n)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })?
            } else {
                match (width, height) {
                    (Some(w), Some(h)) => (w, h),
                    _ => return Err("Specify --width and --height, or --preset".to_string()),
                }
            };
            Command::Resize { width: w, height: h }
        }
        ExecAction::Navigate { url } => Command::Navigate { url },
        ExecAction::Click { selector } => Command::Click { selector },
        ExecAction::ClickAt { x, y } => Command::ClickAt { x, y },
        ExecAction::Scroll { y, delta, to_bottom } => Command::Scroll { y, delta, to_bottom },
        ExecAction::Html { selector } => Command::GetHtml { selector },
        ExecAction::Reload => Command::Reload,
        ExecAction::Detach => Command::Detach,
    };

    let response = send_command_to_server(port, command).await?;

    match response {
        Response::Pong => println!("Pong!"),
        Response::Status {
            connected,
            page_url,
            viewport_width,
            viewport_height,
        } => {
            println!("Connected: {}", connected);
            if let Some(url) = page_url {
                println!("Page URL: {}", url);
            }
            if let (Some(w), Some(h)) = (viewport_width, viewport_height) {
                println!("Viewport: {}x{}", w, h);
            }
        }
        Response::Success { data } => {
            println!("Success");
            if let Some(d) = data {
                println!("{}", serde_json::to_string_pretty(&d).unwrap_or_default());
            }
        }
        Response::Html { html } => {
            println!("{}", html);
        }
        Response::Error { message } => {
            return Err(message);
        }
        _ => {
            println!("Response: {:?}", response);
        }
    }

    Ok(())
}

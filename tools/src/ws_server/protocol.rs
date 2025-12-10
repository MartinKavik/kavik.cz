//! WebSocket protocol types for CLI <-> Server <-> Extension communication

use serde::{Deserialize, Serialize};

/// Commands sent from CLI to Extension via Server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Command {
    /// Check if extension is connected
    Ping,

    /// Get extension status
    GetStatus,

    /// Take a screenshot
    Screenshot,

    /// Resize viewport
    Resize { width: u32, height: u32 },

    /// Navigate to URL
    Navigate { url: String },

    /// Click an element by selector
    Click { selector: String },

    /// Click at absolute coordinates
    ClickAt { x: i32, y: i32 },

    /// Scroll the page
    Scroll {
        #[serde(skip_serializing_if = "Option::is_none")]
        y: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        delta: Option<i32>,
        #[serde(default)]
        to_bottom: bool,
    },

    /// Get page HTML
    GetHtml {
        #[serde(skip_serializing_if = "Option::is_none")]
        selector: Option<String>,
    },

    /// Reload the page
    Reload,

    /// Detach CDP debugger
    Detach,
}

/// Response from Extension to CLI via Server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Response {
    /// Command succeeded
    Success {
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<serde_json::Value>,
    },

    /// Command failed
    Error { message: String },

    /// Screenshot data
    Screenshot { base64: String },

    /// Pong response
    Pong,

    /// Extension status
    Status {
        connected: bool,
        #[serde(rename = "pageUrl")]
        page_url: Option<String>,
        #[serde(rename = "viewportWidth")]
        viewport_width: Option<u32>,
        #[serde(rename = "viewportHeight")]
        viewport_height: Option<u32>,
    },

    /// HTML content
    Html { html: String },
}

/// Request wrapper with ID for request/response matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: u64,
    pub command: Command,
}

/// Response wrapper with ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub id: u64,
    pub response: Response,
}

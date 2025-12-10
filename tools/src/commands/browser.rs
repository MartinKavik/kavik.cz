//! Browser launching and management
//!
//! Uses Chromium (not Chrome) because:
//! - `--load-extension` flag is deprecated in Chrome 137+ branded builds
//! - Chromium keeps all developer flags permanently

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// Find the extension directory relative to the kavik-tools binary
fn find_extension_path() -> Result<PathBuf, String> {
    // Try relative to current executable
    if let Ok(exe_path) = std::env::current_exe() {
        // Binary is in target/release or target/debug
        // Extension is in tools/extension
        if let Some(parent) = exe_path.parent() {
            // Check if we're in target/release or target/debug
            if parent.ends_with("release") || parent.ends_with("debug") {
                // Go up to tools: target/release -> target -> tools
                if let Some(target_dir) = parent.parent() {
                    if let Some(tools_dir) = target_dir.parent() {
                        let ext_path = tools_dir.join("extension");
                        if ext_path.exists() {
                            return Ok(ext_path);
                        }
                    }
                }
            }
        }
    }

    // Try relative to current directory
    let candidates = [
        PathBuf::from("tools/extension"),
        PathBuf::from("extension"),
        PathBuf::from("../tools/extension"),
    ];

    for path in &candidates {
        if path.exists() {
            return path
                .canonicalize()
                .map_err(|e| format!("Failed to canonicalize path: {}", e));
        }
    }

    Err("Extension directory not found. Run from kavik.cz repo root or tools directory.".to_string())
}

/// Find Chromium binary in PATH
fn find_chromium_binary() -> Result<PathBuf, String> {
    let candidates = [
        "chromium-browser", // Debian/Ubuntu
        "chromium",         // Arch/Fedora
    ];

    for name in candidates {
        if let Ok(path) = which::which(name) {
            return Ok(path);
        }
    }

    Err(
        "Chromium not found in PATH.\n\
        Install with: apt install chromium-browser (Debian/Ubuntu)\n\
        or: pacman -S chromium (Arch)\n\
        or: dnf install chromium (Fedora)\n\n\
        Note: Chrome is not supported because --load-extension was deprecated in Chrome 137+"
            .to_string(),
    )
}

/// Create a temporary user data directory for isolated browser profile
fn create_temp_profile() -> Result<PathBuf, String> {
    let temp_dir = std::env::temp_dir().join(format!("kavik-chromium-{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp profile: {}", e))?;
    Ok(temp_dir)
}

/// Options for launching the browser
pub struct LaunchOptions {
    pub url: String,
    #[allow(dead_code)]
    pub ws_port: u16,
    pub headless: bool,
    pub browser_path: Option<PathBuf>,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            url: "http://localhost:4321".to_string(),
            ws_port: 9222,
            headless: false,
            browser_path: None,
        }
    }
}

/// Launch Chromium with the extension pre-loaded
pub fn launch_browser(opts: LaunchOptions) -> Result<Child, String> {
    let extension_path = find_extension_path()?;
    let user_data_dir = create_temp_profile()?;
    let browser = opts
        .browser_path
        .map(Ok)
        .unwrap_or_else(find_chromium_binary)?;

    println!("Launching Chromium from: {}", browser.display());
    println!("Extension path: {}", extension_path.display());
    println!("User data dir: {}", user_data_dir.display());

    let mut cmd = Command::new(&browser);

    // Core flags for automation
    cmd.args([
        &format!("--load-extension={}", extension_path.display()),
        &format!("--user-data-dir={}", user_data_dir.display()),
        "--no-first-run",
        "--no-default-browser-check",
        "--disable-default-apps",
        "--disable-popup-blocking",
        "--disable-translate",
        "--disable-sync",
        // Disable background throttling so extension stays responsive
        "--disable-background-timer-throttling",
        "--disable-backgrounding-occluded-windows",
        "--disable-renderer-backgrounding",
    ]);

    if opts.headless {
        // Use new headless mode that supports extensions
        cmd.arg("--headless=new");
    }

    // Open the target URL
    cmd.arg(&opts.url);

    // Suppress browser output
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    let child = cmd.spawn().map_err(|e| {
        format!(
            "Failed to launch Chromium: {}.\n\
            Binary: {}\n\
            Is Chromium installed?",
            e,
            browser.display()
        )
    })?;

    println!("Chromium launched with PID: {}", child.id());
    println!("Waiting for extension to connect...");

    Ok(child)
}

/// Wait for the extension to connect to the WebSocket server
pub async fn wait_for_extension_connection(port: u16, timeout: Duration) -> Result<(), String> {
    use tokio::time::{sleep, Instant};

    let start = Instant::now();
    let check_interval = Duration::from_millis(500);

    while start.elapsed() < timeout {
        // Try to get status from the server
        match crate::ws_server::send_command_to_server(port, crate::ws_server::Command::GetStatus)
            .await
        {
            Ok(crate::ws_server::Response::Status { connected, .. }) => {
                if connected {
                    println!("Extension connected!");
                    return Ok(());
                }
            }
            _ => {}
        }

        sleep(check_interval).await;
    }

    Err(format!(
        "Extension did not connect within {:?}.\n\
        Check that:\n\
        1. Chromium launched successfully\n\
        2. Extension loaded without errors (check chrome://extensions)\n\
        3. WebSocket server is running on port {}",
        timeout, port
    ))
}

/// Kill all kavik-related Chromium processes
pub fn kill_browser_instances() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        // Find and kill Chromium processes with our temp profile
        let output = Command::new("pkill")
            .args(["-f", "kavik-chromium-"])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                println!("Killed Chromium automation instances");
            }
            Ok(_) => {
                println!("No Chromium automation instances found");
            }
            Err(e) => {
                eprintln!("pkill failed: {}", e);
            }
        }

        // Also clean up temp directories
        let temp_dir = std::env::temp_dir();
        if let Ok(entries) = std::fs::read_dir(&temp_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                if name.to_string_lossy().starts_with("kavik-chromium-") {
                    if let Err(e) = std::fs::remove_dir_all(entry.path()) {
                        eprintln!("Failed to remove temp dir: {}", e);
                    }
                }
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        println!("Browser kill not implemented for this platform");
    }

    Ok(())
}

/// Check if Chromium is available
pub fn check_chromium_available() -> Result<PathBuf, String> {
    find_chromium_binary()
}

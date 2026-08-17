//! Capability detection — determines host surface and available engines.
//!
//! Copyright (c) 2026 Timothy Charles Holborn. All rights reserved.
//!
//! Ports the capability gating pattern from `webizen-studio::endpoints`.
//! Containers call these functions to fail closed when an engine is
//! unreachable (e.g. on public web vs desktop webview).

use web_sys::Window;

/// Host surface type — determines which engines are reachable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostSurface {
    /// Running inside the webizen-desktop Tauri webview.
    /// All daemon endpoints, GPU surface, and native commands are reachable.
    DesktopWebview,
    /// Running on the public web (no daemon).
    /// Containers must fail closed or use public-web fallbacks.
    PublicWeb,
}

/// Detect whether the app is running inside the Tauri desktop webview.
///
/// Tauri injects `window.__TAURI_INTERNALS__` (or `window.__TAURI__`)
/// when running inside the native host.
pub fn is_native_host() -> bool {
    let window = web_sys::window().unwrap();
    has_tauri_internals(&window)
}

/// Detect the current host surface.
pub fn current_host_surface() -> HostSurface {
    if is_native_host() {
        HostSurface::DesktopWebview
    } else {
        HostSurface::PublicWeb
    }
}

/// Browser panes are only available on the desktop host.
pub fn supports_browser_pane() -> bool {
    is_native_host()
}

/// GPU surface (wgpu) is only available on the desktop host.
pub fn supports_gpu_surface() -> bool {
    is_native_host()
}

/// WebSocket telemetry stream is only available when daemon is reachable.
pub fn supports_telemetry_stream() -> bool {
    is_native_host()
}

/// Construct the daemon base URL (if native host).
pub fn daemon_base_url() -> Option<String> {
    if !is_native_host() {
        return None;
    }
    // The daemon port is injected by the desktop host as a global.
    let window = web_sys::window().unwrap();
    let port = js_sys::Reflect::get(window.as_ref(), &"__QUALIA_DAEMON_PORT__".into())
        .ok()
        .and_then(|v| v.as_f64())
        .map(|p| p as u16)
        .unwrap_or(8080);
    Some(format!("http://127.0.0.1:{}", port))
}

/// Construct the telemetry WebSocket URL (if native host).
pub fn telemetry_ws_url() -> Option<String> {
    if !is_native_host() {
        return None;
    }
    let window = web_sys::window().unwrap();
    let port = js_sys::Reflect::get(window.as_ref(), &"__QUALIA_DAEMON_PORT__".into())
        .ok()
        .and_then(|v| v.as_f64())
        .map(|p| p as u16)
        .unwrap_or(8080);
    Some(format!("ws://127.0.0.1:{}/telemetry", port))
}

/// Construct the LLM handshake WebSocket URL (if native host).
pub fn llm_ws_url() -> Option<String> {
    if !is_native_host() {
        return None;
    }
    Some("ws://127.0.0.1:4242".into())
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn has_tauri_internals(window: &Window) -> bool {
    let tauri = js_sys::Reflect::get(window.as_ref(), &"__TAURI_INTERNALS__".into());
    if let Ok(val) = &tauri {
        if !val.is_undefined() && !val.is_null() {
            return true;
        }
    }
    let tauri2 = js_sys::Reflect::get(window.as_ref(), &"__TAURI__".into());
    if let Ok(val) = &tauri2 {
        return !val.is_undefined() && !val.is_null();
    }
    false
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_surface_equality() {
        assert_eq!(HostSurface::DesktopWebview, HostSurface::DesktopWebview);
        assert_ne!(HostSurface::DesktopWebview, HostSurface::PublicWeb);
    }
}

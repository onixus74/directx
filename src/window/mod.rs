/*!
Window helper module.

## Summary
Provides a pre-configured window for DirectX rendering. Intended only as a
minimal DXUT-like library for creating examples.

*/

// TODO:
//   * Documentation
//   * Configuration based on enabled cargo options
//   * Handling of DirectX errors, e.g. lost device

use std::sync::mpsc::Receiver;

use com_rs::ComPtr;
use directx_sys::d3d11::*;
use directx_sys::dxgi::IDXGISwapChain;
use user32::*;
use winapi::*;

mod builder;
pub use self::builder::WindowBuilder;

mod events;
pub use self::events::*;

mod wndproc;

/// Window
pub struct Window {
    hwnd: HWND,
    recv: Receiver<WindowEvent>,
    device: ComPtr<ID3D11Device>,
    device_context: ComPtr<ID3D11DeviceContext>,
    swapchain: ComPtr<IDXGISwapChain>,
    render_target: ComPtr<ID3D11RenderTargetView>
}

unsafe impl Send for Window { }

impl Window {
    /// Create a new `Window` with default options
    pub fn new() -> Result<Window, DWORD> {
        WindowBuilder::new().build()
    }

    /// Accessor for D3D11 device
    pub fn device(&self) -> ComPtr<ID3D11Device> {
        self.device.clone()
    }

    /// Accessor for D3D11 context
    pub fn context(&self) -> ComPtr<ID3D11DeviceContext> {
        self.device_context.clone()
    }

    /// Accessor for D3D11 render target
    pub fn render_target(&self) -> ComPtr<ID3D11RenderTargetView> {
        self.render_target.clone()
    }

    /// Accessor for DXGI swapchain
    pub fn swapchain(&self) -> ComPtr<IDXGISwapChain> {
        self.swapchain.clone()
    }

    /// Return a blocking iterator for window events
    pub fn wait_events(&self) -> WaitEventIterator {
        WaitEventIterator { window: &self }
    }

    /// Return a non-blocking iterator for window events
    pub fn poll_events(&self) -> PollEventIterator {
        PollEventIterator { window: &self }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { DestroyWindow(self.hwnd) };
    }
}

/// Non-blocking window event iterator.
pub struct PollEventIterator<'a> {
    window: &'a Window
}

impl<'a> Iterator for PollEventIterator<'a> {
    type Item = WindowEvent;
    fn next(&mut self) -> Option<WindowEvent> {
        self.window.recv.try_recv().ok()
    }
}

/// Blocking window event iterator.
pub struct WaitEventIterator<'a> {
    window: &'a Window
}

impl<'a> Iterator for WaitEventIterator<'a> {
    type Item = WindowEvent;
    fn next(&mut self) -> Option<WindowEvent> {
        self.window.recv.recv().ok()
    }
}

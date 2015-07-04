use std::cell::RefCell;
use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use com_rs::ComPtr;
use directx_sys::d3d11::*;
use directx_sys::dxgi::{self, IDXGISwapChain, DXGISwapChain};
use kernel32::*;
use user32::*;
use winapi::*;

use super::{Window, WindowEvent, WindowMode};
use super::wndproc::wndproc;

// TODO: - options for specifying back buffer/depth formats
//       - option to specify multiple supported d3d feature levels

#[link(name = "user32")]
extern "stdcall" {
    fn SetRect(lprc: *mut RECT, left: c_int, top: c_int, right: c_int,
               bottom: c_int) -> BOOL;
}

// Per-thread window data
pub struct WindowContext {
    pub hwnd: HWND,
    pub send: Sender<WindowEvent>
}

thread_local! {
    pub static CONTEXT: RefCell<Option<WindowContext>> = RefCell::new(None)
}


/// Window builder
#[derive(Debug)]
pub struct WindowBuilder {
    title: Vec<u16>,
    dimensions: (u32, u32),
    window_mode: WindowMode,
    feature_level: Option<FeatureLevel>
}

// Helper method to convert str to Windows-compatible UTF16 string
fn utf16_str(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide()
                 .chain(Some(0).into_iter())
                 .collect::<Vec<_>>()
}

impl WindowBuilder {
    /// Create a new WindowBuilder with default options
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            title: utf16_str("DirectX Window"),
            dimensions: (800, 600),
            window_mode: WindowMode::Windowed,
            feature_level: None
        }
    }

    /// Specify the title of the created window
    pub fn title(mut self, title: &str) -> WindowBuilder {
        self.title = utf16_str(title);
        self
    }

    /// Specify the dimensions of the created window
    pub fn dimensions(mut self, width: u32, height: u32) -> WindowBuilder {
        self.dimensions = (width, height);
        self
    }

    /// Set the desired Direct3D feature level
    pub fn feature_level(mut self, level: FeatureLevel) -> WindowBuilder {
        self.feature_level = Some(level);
        self
    }

    /// Specify the initial window mode
    pub fn window_mode(mut self, window_mode: WindowMode) -> WindowBuilder {
        self.window_mode = window_mode;
        self
    }

    /// Build the window
    pub fn build(self) -> Result<Window, DWORD> {
        // Channel for passing the Window out of the new thread
        let (tx, rx) = channel();

        // Spawn a new thread to create the window
        thread::spawn(move || {
            // Create new window with specified options
            let hwnd = self.create_window();
            if hwnd.is_null() {
                tx.send(Err(unsafe { GetLastError() })).ok();
                return;
            }

            // Make window visible
            unsafe {
                ShowWindow(hwnd, SW_SHOW);
                UpdateWindow(hwnd);
            }

            // Setup window event channel
            let (wnd_tx, wnd_rx) = channel();
            let context = WindowContext {
                hwnd: hwnd,
                send: wnd_tx
            };

            // Stash context in thread local storage
            CONTEXT.with(|cell| {
                *cell.borrow_mut() = Some(context);
            });

            // Initialise D3D (TODO: higher level stuff)
            let mut d3d_device = ComPtr::<ID3D11Device>::new();
            let mut d3d_context = ComPtr::<ID3D11DeviceContext>::new();
            let mut dxgi_swapchain = ComPtr::<IDXGISwapChain>::new();

            let mut usage = dxgi::Usage::default();
            usage.set_dxgi_usage(dxgi::DXGI_USAGE_RENDER_TARGET_OUTPUT);

            let windowed = match self.window_mode {
                WindowMode::Fullscreen => FALSE,
                _ => TRUE
            };

            let swapchain_desc = dxgi::SwapChainDesc {
                buffer_desc: dxgi::ModeDesc {
                    width: self.dimensions.0 as u32,
                    height: self.dimensions.1 as u32,
                    format: dxgi::Format::R8G8B8A8Unorm,
                    ..Default::default()
                },
                sample_desc: dxgi::SampleDesc {
                    count: 1,
                    quality: 0
                },
                buffer_count: 1,
                buffer_usage: usage,
                output_window: hwnd,
                windowed: windowed,
                ..Default::default()
            };

            let feature_level = self.feature_level.unwrap_or(
                                    FeatureLevel::Level_11_0);
            let mut actual_feature_level = unsafe { mem::zeroed() };

            let result = unsafe {
                D3D11CreateDeviceAndSwapChain(
                    ptr::null(),
                    DriverType::Hardware,
                    ptr::null_mut(),
                    D3D11_CREATE_DEVICE_DEBUG,
                    &feature_level,
                    1,
                    SDK_VERSION,
                    &swapchain_desc,
                    dxgi_swapchain.as_mut(),
                    d3d_device.as_mut(),
                    &mut actual_feature_level,
                    d3d_context.as_mut())
            };
            match result {
                0 => { }
                _ => {
                    tx.send(Err(unsafe { GetLastError() })).ok();
                    return;
                }
            }

            let mut d3d_backbuffer = ComPtr::<ID3D11Texture2D>::new();
            unsafe {
                dxgi_swapchain.get_buffer(
                        0, &d3d_backbuffer.iid(), d3d_backbuffer.as_mut())
            };

            let mut d3d_rtv = ComPtr::<ID3D11RenderTargetView>::new();
            unsafe {
                d3d_device.create_render_target_view(
                    d3d_backbuffer.as_ptr(), ptr::null(), d3d_rtv.as_mut())
            };

            unsafe {
                d3d_context.om_set_render_targets(
                    1, &d3d_rtv.as_ptr(), ptr::null())
            };

            let viewport = Viewport {
                width: self.dimensions.0 as f32,
                height: self.dimensions.1 as f32,
                ..Default::default()
            };
            unsafe { d3d_context.rs_set_viewports(1, &viewport) };

            // Send new window back to parent thread
            tx.send(Ok(Window {
                hwnd: hwnd,
                recv: wnd_rx,
                device: d3d_device,
                device_context: d3d_context.clone(),
                swapchain: dxgi_swapchain.clone(),
                render_target: d3d_rtv.clone()
            })).ok();

            // Start message pump
            let mut msg = unsafe { mem::uninitialized() };
            loop {
                unsafe {
                    while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0,
                                       PM_REMOVE) > 0 {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }
        });

        rx.recv().unwrap()
    }

    fn create_window(&self) -> HWND {
        let class_name = utf16_str("DirectXWindow");

        let wcex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: unsafe { GetModuleHandleW(ptr::null()) },
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: ptr::null_mut()
        };
        unsafe { RegisterClassExW(&wcex) };

        let window_style = match self.window_mode {
            WindowMode::Windowed => WS_OVERLAPPEDWINDOW,
            _ => WS_POPUP
        };

        let dimensions = match self.window_mode {
            WindowMode::FullscreenWindowed => unsafe { (
                GetSystemMetrics(SM_CXSCREEN) as u32,
                GetSystemMetrics(SM_CYSCREEN) as u32
            )},
            _ => self.dimensions
        };

        let rect = unsafe {
            let mut rect = ::std::mem::uninitialized();
            SetRect(&mut rect, 0, 0, dimensions.0 as i32, dimensions.1 as i32);
            AdjustWindowRect(&mut rect, WS_OVERLAPPEDWINDOW, FALSE);
            rect
        };

        unsafe { CreateWindowExW(
                WS_EX_APPWINDOW,
                class_name.as_ptr(),
                self.title.as_ptr() as LPCWSTR,
                window_style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                rect.right - rect.left,
                rect.bottom - rect.top,
                ptr::null_mut(),
                ptr::null_mut(),
                GetModuleHandleW(ptr::null()),
                ptr::null_mut())
        }
    }
}

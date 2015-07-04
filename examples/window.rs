extern crate directx;
extern crate directx_sys;

#[cfg(feature = "window")]
use directx::window::*;
use directx_sys::d3d11::{D3D11DeviceContext, FeatureLevel};
use directx_sys::dxgi::DXGISwapChain;

#[cfg(feature = "window")]
fn main() {
    let window = WindowBuilder::new()
        .title("Hello World")
        .dimensions(1280, 720)
        .feature_level(FeatureLevel::Level_11_0)
        .window_mode(WindowMode::Windowed)
        .build().unwrap();

    let mut color = [0.0, 0.2, 0.4, 1.0];

    'main: loop {
        for event in window.poll_events() {
            match event {
                WindowEvent::Closed => break 'main,
                WindowEvent::KeyboardInput(key, KeyboardEvent::Pressed) => {
                    match key {
                        KeyCode::Key1 => color = [1.0, 0.0, 0.0, 1.0],
                        KeyCode::Key2 => color = [0.0, 1.0, 0.0, 1.0],
                        KeyCode::Key3 => color = [0.0, 0.0, 1.0, 1.0],
                        _ => {}
                    }
                }
                _ => { }
            }
        }

        // TODO higher level stuff
        unsafe {
            window.context().clear_render_target_view(
                window.render_target().as_ptr(),
                &color);
            window.swapchain().present(0, Default::default());
        }
    }
}

#[cfg(not(feature = "window"))]
fn main() {
    println!("This example requires the 'window' feature");
}

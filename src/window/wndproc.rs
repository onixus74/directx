use user32::DefWindowProcW;
use winapi::*;

use super::{KeyCode, KeyboardEvent, MouseEvent, WindowEvent};
use super::builder::CONTEXT;

#[link(name = "user32")]
extern "stdcall" {
    fn MapVirtualKeyW(code: UINT, map_type: UINT) -> UINT;
}

const MAPVK_VSC_TO_VK_EX: UINT = 3;


// Message handler, forward events on to Window.
pub unsafe extern "system" fn wndproc(window: HWND, msg: UINT, wparam: WPARAM,
                                      lparam: LPARAM) -> LRESULT {
    // Helper for sending events to the current window
    fn send_event(event: WindowEvent) {
        CONTEXT.with(|cell| {
            if let Some(ref ctx) = *cell.borrow() {
                ctx.send.send(event).ok();
            }
        })
    }
    // Helper for sending mouse events with co-ordinates
    fn send_mouse_event(event: MouseEvent, lparam: LPARAM) {
        send_event(WindowEvent::MouseInput(
            event,
            lparam as i16,
            (lparam >> 16) as i16
        ))
    }

    // Map windows virtual key to left/right versions
    fn map_keycode(wparam: WPARAM, lparam: LPARAM) -> WPARAM {
        let scancode = ((lparam & 0x00ff0000) >> 16) as u32;
        let extended = (lparam & 0x01000000) != 0;

        match wparam {
            VK_SHIFT => unsafe {
                MapVirtualKeyW(scancode, MAPVK_VSC_TO_VK_EX) as WPARAM
            },
            VK_CONTROL => {
                if extended { VK_RCONTROL } else { VK_LCONTROL }
            },
            VK_MENU => {
                if extended { VK_RMENU } else { VK_LMENU }
            },
            _ => wparam
        }
    }

    match msg {
        // Standard window events
        WM_ACTIVATE => {
            send_event(WindowEvent::Focused(wparam != 0));
            0
        },
        WM_DESTROY => {
            send_event(WindowEvent::Closed);
            0
        },
        WM_SIZE => {
            send_event(WindowEvent::Resized(
                lparam as u16,
                (lparam >> 16) as u16
            ));
            0
        },
        WM_MOVE => {
            send_event(WindowEvent::Moved(
                lparam as i16,
                (lparam >> 16) as i16
            ));
            0
        }

        // Keyboard events
        WM_KEYDOWN => {
            send_event(WindowEvent::KeyboardInput(
                KeyCode::from_virtual_key(map_keycode(wparam, lparam)),
                if (lparam & (1 << 30)) == 0 {
                    KeyboardEvent::Pressed
                } else {
                    KeyboardEvent::Repeated
                }
            ));
            0
        },
        WM_KEYUP => {
            send_event(WindowEvent::KeyboardInput(
                KeyCode::from_virtual_key(map_keycode(wparam, lparam)),
                KeyboardEvent::Released
            ));
            0
        },

        // Mouse events
        WM_MOUSEMOVE => {
            send_mouse_event(MouseEvent::Move, lparam);
            0
        },
        WM_LBUTTONDOWN => {
            send_mouse_event(MouseEvent::LeftButtonDown, lparam);
            0
        },
        WM_LBUTTONUP => {
            send_mouse_event(MouseEvent::LeftButtonUp, lparam);
            0
        },
        WM_RBUTTONDOWN => {
            send_mouse_event(MouseEvent::RightButtonDown, lparam);
            0
        },
        WM_RBUTTONUP => {
            send_mouse_event(MouseEvent::RightButtonUp, lparam);
            0
        },
        WM_MBUTTONDOWN => {
            send_mouse_event(MouseEvent::MiddleButtonDown, lparam);
            0
        },
        WM_MBUTTONUP => {
            send_mouse_event(MouseEvent::MiddleButtonUp, lparam);
            0
        },

        // Use default message handler for all other messages
        _ => DefWindowProcW(window, msg, wparam, lparam)
    }
}

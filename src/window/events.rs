use winapi::*;

macro_rules! keycode {
    ($($key:ident => $name:ident),+) => (
        #[derive(Debug)]
        #[allow(missing_docs)]
        /// Friendly names for Windows VK_* constants
        pub enum KeyCode {
            /// Unknown key, contained value is the virtual key code
            Unknown(WPARAM),
            $($name),+
        }

        impl KeyCode {
            /// Convert from Windows virtual keycode.
            pub fn from_virtual_key(virtual_key: WPARAM) -> KeyCode {
                match virtual_key {
                    $($key => KeyCode::$name),+,
                    _ => KeyCode::Unknown(virtual_key)
                }
            }
        }
    )
}

keycode! {
    VK_LSHIFT => LeftShift,
    VK_RSHIFT => RightShift,
    VK_LCONTROL => LeftControl,
    VK_RCONTROL => RightControl,
    VK_LMENU => LeftAlt,
    VK_RMENU => RightAlt,
    VK_OEM_PLUS => Plus,
    VK_OEM_COMMA => Comma,
    VK_OEM_MINUS => Minus,
    VK_OEM_PERIOD => Period,
    VK_OEM_1 => SemiColon,
    VK_OEM_2 => Slash,
    VK_OEM_3 => Tilde,
    VK_OEM_4 => LeftBracket,
    VK_OEM_5 => BackSlash,
    VK_OEM_6 => RightBracket,
    VK_OEM_7 => Quote,
    VK_BACK => Backspace,
    VK_TAB => Tab,
    VK_RETURN => Return,
    VK_PAUSE => Pause,
    VK_ESCAPE => Escape,
    VK_SPACE => Space,
    VK_PRIOR => PageUp,
    VK_NEXT => PageDown,
    VK_END => End,
    VK_HOME => Home,
    VK_LEFT => Left,
    VK_UP => Up,
    VK_RIGHT => Right,
    VK_DOWN => Down,
    VK_SNAPSHOT => PrintScreen,
    VK_INSERT => Insert,
    VK_DELETE => Delete,
    VK_CAPITAL => CapsLock,
    VK_NUMLOCK => NumLock,
    VK_SCROLL => ScrollLock,
    VK_0 => Key0,
    VK_1 => Key1,
    VK_2 => Key2,
    VK_3 => Key3,
    VK_4 => Key4,
    VK_5 => Key5,
    VK_6 => Key6,
    VK_7 => Key7,
    VK_8 => Key8,
    VK_9 => Key9,
    VK_A => A,
    VK_B => B,
    VK_C => C,
    VK_D => D,
    VK_E => E,
    VK_F => F,
    VK_G => G,
    VK_H => H,
    VK_I => I,
    VK_J => J,
    VK_K => K,
    VK_L => L,
    VK_M => M,
    VK_N => N,
    VK_O => O,
    VK_P => P,
    VK_Q => Q,
    VK_R => R,
    VK_S => S,
    VK_T => T,
    VK_U => U,
    VK_V => V,
    VK_W => W,
    VK_X => X,
    VK_Y => Y,
    VK_Z => Z,
    VK_NUMPAD0 => Numpad0,
    VK_NUMPAD1 => Numpad1,
    VK_NUMPAD2 => Numpad2,
    VK_NUMPAD3 => Numpad3,
    VK_NUMPAD4 => Numpad4,
    VK_NUMPAD5 => Numpad5,
    VK_NUMPAD6 => Numpad6,
    VK_NUMPAD7 => Numpad7,
    VK_NUMPAD8 => Numpad8,
    VK_NUMPAD9 => Numpad9,
    VK_MULTIPLY => NumpadMultiply,
    VK_ADD => NumpadPlus,
    VK_SUBTRACT => NumpadMinus,
    VK_DECIMAL => NumpadPeriod,
    VK_DIVIDE => NumpadDivide,
    VK_F1 => F1,
    VK_F2 => F2,
    VK_F3 => F3,
    VK_F4 => F4,
    VK_F5 => F5,
    VK_F6 => F6,
    VK_F7 => F7,
    VK_F8 => F8,
    VK_F9 => F9,
    VK_F10 => F10,
    VK_F11 => F11,
    VK_F12 => F12,
    VK_F13 => F13,
    VK_F14 => F14,
    VK_F15 => F15,
    VK_F16 => F16,
    VK_F17 => F17,
    VK_F18 => F18,
    VK_F19 => F19,
    VK_F20 => F20,
    VK_F21 => F21,
    VK_F22 => F22,
    VK_F23 => F23,
    VK_F24 => F24
}

/// Type of keyboard event
#[derive(Debug)]
pub enum KeyboardEvent {
    /// Key pressed
    Pressed,
    /// Key held
    Repeated,
    /// Key released
    Released
}

/// Type of mouse event
#[derive(Debug)]
pub enum MouseEvent {
    /// Mouse moved
    Move,
    /// Left mouse button pressed
    LeftButtonDown,
    /// Left mouse button released
    LeftButtonUp,
    /// Right mouse button pressed
    RightButtonDown,
    /// Right mouse button released
    RightButtonUp,
    /// Middle mouse button pressed
    MiddleButtonDown,
    /// Middle mouse button released
    MiddleButtonUp
}

/// Window event
#[derive(Debug)]
pub enum WindowEvent {
    /// Keyboard input event
    KeyboardInput(KeyCode, KeyboardEvent),
    /// Mouse input event
    MouseInput(MouseEvent, i16, i16),
    /// Window closed
    Closed,
    /// Window resized
    Resized(u16, u16),
    /// Window moved
    Moved(i16, i16),
    /// Window focused/unfocused
    Focused(bool)
}

/// Window mode
#[derive(Debug)]
pub enum WindowMode {
    /// Regular window
    Windowed,
    /// Fullscreen (exclusive mode)
    Fullscreen,
    /// Fullscreen borderless window
    FullscreenWindowed
}

// Define alphanumeric virtual keys for the keycode macro above
const VK_0: WPARAM = 0x30;
const VK_1: WPARAM = 0x31;
const VK_2: WPARAM = 0x32;
const VK_3: WPARAM = 0x33;
const VK_4: WPARAM = 0x34;
const VK_5: WPARAM = 0x35;
const VK_6: WPARAM = 0x36;
const VK_7: WPARAM = 0x37;
const VK_8: WPARAM = 0x38;
const VK_9: WPARAM = 0x39;
const VK_A: WPARAM = 0x41;
const VK_B: WPARAM = 0x42;
const VK_C: WPARAM = 0x43;
const VK_D: WPARAM = 0x44;
const VK_E: WPARAM = 0x45;
const VK_F: WPARAM = 0x46;
const VK_G: WPARAM = 0x47;
const VK_H: WPARAM = 0x48;
const VK_I: WPARAM = 0x49;
const VK_J: WPARAM = 0x4A;
const VK_K: WPARAM = 0x4B;
const VK_L: WPARAM = 0x4C;
const VK_M: WPARAM = 0x4D;
const VK_N: WPARAM = 0x4E;
const VK_O: WPARAM = 0x4F;
const VK_P: WPARAM = 0x50;
const VK_Q: WPARAM = 0x51;
const VK_R: WPARAM = 0x52;
const VK_S: WPARAM = 0x53;
const VK_T: WPARAM = 0x54;
const VK_U: WPARAM = 0x55;
const VK_V: WPARAM = 0x56;
const VK_W: WPARAM = 0x57;
const VK_X: WPARAM = 0x58;
const VK_Y: WPARAM = 0x59;
const VK_Z: WPARAM = 0x5A;

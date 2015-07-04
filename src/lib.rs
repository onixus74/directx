#![deny(missing_docs)]

/*!
# directx 0.0.1
High-level Rust wrapper for the DirectX API.
*/

extern crate com_rs;
extern crate directx_sys;
#[cfg(feature = "window")]
extern crate kernel32;
extern crate libc;
#[cfg(feature = "window")]
extern crate user32;
extern crate winapi;

#[cfg(feature = "window")]
pub mod window;

//! highgui: high-level GUI
extern crate libc;
use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::mem;
use std::ptr;

extern "C" {
    fn cv_named_window(name: *const c_char, flags: c_int);
    fn cv_destroy_window(name: *const c_char);
    fn cv_set_mouse_callback(
        name: *const c_char,
        on_mouse: extern "C" fn(e: i32, x: i32, y: i32, f: i32, data: *mut c_void),
        userdata: *mut c_void,
    );
}


/// Creates a window that can be used as a placeholder for images and
/// trackbars. All created windows are referred to by their names. If a window
/// with the same name already exists, the function does nothing.
pub fn highgui_named_window(name: &str, flags: WindowFlags) {
    let s = CString::new(name).unwrap();
    unsafe {
        cv_named_window((&s).as_ptr(), flags.bits());
    }
}

/// Destroys the specified window with the given name.
pub fn highgui_destroy_window(name: &str) {
    let s = CString::new(name).unwrap();
    unsafe {
        cv_destroy_window((&s).as_ptr());
    }
}

/// Pointer referring to the data used in MouseCallback
pub type MouseCallbackData = *mut c_void;

/// Callback function for mouse events, primarily used in
/// [highgui_set_mouse_callback](fn.highgui_set_mouse_callback.html)
pub type MouseCallback = fn(i32, i32, i32, i32, MouseCallbackData);

/// Set mouse handler for the specified window (identified by name). A callback
/// handler should be provided and optional user_data can be passed around.
pub fn highgui_set_mouse_callback(name: &str, on_mouse: MouseCallback, user_data: *mut c_void) {
    struct CallbackWrapper {
        cb: Box<MouseCallback>,
        data: *mut c_void,
    }

    extern "C" fn _mouse_callback(e: i32, x: i32, y: i32, f: i32, ud: *mut c_void) {
        let cb_wrapper = unsafe { ptr::read(ud as *mut CallbackWrapper) };
        let true_callback = *(cb_wrapper.cb);
        true_callback(e, x, y, f, cb_wrapper.data);
        mem::forget(cb_wrapper.cb);
    }

    let box_wrapper: Box<CallbackWrapper> = Box::new(CallbackWrapper {
        cb: Box::new(on_mouse),
        data: user_data,
    });
    let box_wrapper_raw = Box::into_raw(box_wrapper) as *mut c_void;

    let s = CString::new(name).unwrap();
    unsafe {
        cv_set_mouse_callback((&s).as_ptr(), _mouse_callback, box_wrapper_raw);
    }
}

bitflags!{
    /// Flags for [highgui_named_window](fn.highgui_named_window.html)
    /// specifying the behavior of the window.
    pub struct WindowFlags: i32 {
        /// The user can resize the window (no constraint).
        /// Use also to switch a fullscreen window to a normal size.
        const WINDOW_NORMAL = 0x00000000;
        /// The user cannot resize the window, the size is constrainted by the image displayed.
        const WINDOW_AUTOSIZE = 0x00000001;
        /// Window with opengl support.
        const WINDOW_OPENGL = 0x00001000;
        /// Change the window to fullscreen.
        const WINDOW_FULLSCREEN = 0x00000001;
        /// The image expands as much as it can (no ratio constraint).
        const WINDOW_FREERATIO = 0x00000100;
        /// The ratio of the image is respected.
        const WINDOW_KEEPRATIO = 0x00000000;
        /// Show status bar and tool bar (if supported).
        /// **Note:** Only supported by the Qt window backend.
        const WINDOW_GUI_EXPANDED = 0x00000000;
        /// Old-fashioned way – no status bar, no tool bar.
        /// **Note:** Only supported by the Qt window backend.
        const WINDOW_GUI_NORMAL = 0x00000010;
    }
}

/// Mouse events
#[repr(i32)] // i32 to align it to the respective callback parameter
#[derive(Clone, Copy, Debug)]
pub enum MouseEventType {
    /// Indicates that the mouse has moved over the window.
    MouseMove = 0,
    /// Indicates that the left mouse button is pressed.
    LButtonDown = 1,
    /// Indicates that the right mouse button is pressed.
    RButtonDown = 2,
    /// Indicates that the middle mouse button is pressed.
    MButtonDown = 3,
    /// Indicates that left mouse button is released.
    LButtonUp = 4,
    /// Indicates that right mouse button is released.
    RButtonUp = 5,
    /// Indicates that middle mouse button is released.
    MButtonUp = 6,
    /// Indicates that left mouse button is double clicked.
    LButtonDblClick = 7,
    /// Indicates that right mouse button is double clicked.
    RButtonDblClick = 8,
    /// Indicates that middle mouse button is double clicked.
    MButtonDblClick = 9,
    /// Positive and negative values mean forward and backward scrolling, respectively.
    MouseWheel = 10,
    /// Positive and negative values mean right and left scrolling, respectively.
    MouseHWheel = 11,
}

//! highgui: high-level GUI
extern crate libc;
use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::{mem, ptr};

extern "C" {
    fn cv_named_window(name: *const c_char, flags: c_int);
    fn cv_destroy_window(name: *const c_char);
    fn cv_set_mouse_callback(
        name: *const c_char,
        on_mouse: Option<extern "C" fn(e: i32, x: i32, y: i32, f: i32, data: *mut c_void)>,
        userdata: *mut c_void,
    );
}

/// Creates a window that can be used as a placeholder for images and
/// trackbars. All created windows are referred to by their names. If a window
/// with the same name already exists, the function does nothing.
pub fn create_named_window(name: &str, flags: WindowFlags) {
    let s = CString::new(name).unwrap();
    unsafe {
        cv_named_window((&s).as_ptr(), flags.bits());
    }
}

/// Destroys the specified window with the given name.
pub fn destroy_window(name: &str) {
    let s = CString::new(name).unwrap();
    unsafe {
        cv_destroy_window((&s).as_ptr());
    }
}

/// Callback function for mouse events, primarily used in
/// [set_mouse_callback](fn.set_mouse_callback.html).
pub type MouseCallback<T> = fn(i32, i32, i32, i32, &T);

/// Represents a handle to the mouse callback and its data.
///
/// Dropping this handle also detaches the callback from the window. Use std::mem::forget to
/// deliberately leak it.
#[derive(Debug)]
pub struct MouseCallbackHandle<T: Sync + Send> {
    wrapper: *mut MouseCallbackWrapper<T>,
    name: CString,
}

impl<T: Sync + Send> Drop for MouseCallbackHandle<T> where {
    fn drop(&mut self) {
        unsafe {
            // set the new callback to NULL
            // if the window does not exist anymore, this does nothing
            cv_set_mouse_callback((&self.name).as_ptr(), None, ptr::null_mut());
            Box::from_raw(self.wrapper)
        };
    }
}

/// Sets the mouse callback for the specified window (identified by name).
///
/// Since the callback is called from another thread, the type specified for data must implement
/// Sync and Send. The returned MouseCallbackHandle manages the lifetime of supplied data.
pub fn set_mouse_callback<T: Sync + Send>(
    name: &str,
    callback: MouseCallback<T>,
    data: T,
) -> MouseCallbackHandle<T> {
    let boxed_wrapper = Box::new(MouseCallbackWrapper::<T> {
        callback: Box::new(callback),
        data,
    });
    let boxed_wrapper_raw = Box::into_raw(boxed_wrapper);

    let s = CString::new(name).unwrap();
    unsafe {
        cv_set_mouse_callback(
            (&s).as_ptr(),
            Some(MouseCallbackWrapper::<T>::extern_mouse_callback),
            boxed_wrapper_raw as *mut c_void,
        );
    }

    MouseCallbackHandle {
        wrapper: boxed_wrapper_raw,
        name: s,
    }
}

struct MouseCallbackWrapper<T: Sync + Send> {
    callback: Box<MouseCallback<T>>,
    data: T,
}

impl<T: Sync + Send> MouseCallbackWrapper<T> {
    extern "C" fn extern_mouse_callback(e: i32, x: i32, y: i32, f: i32, ud: *mut c_void) {
        let wrapper = unsafe { Box::from_raw(ud as *mut Self) };
        let true_callback = *(wrapper.callback);
        true_callback(e, x, y, f, &wrapper.data);

        // leak the callback here to let opencv use it for multiple callbacks.
        // its lifetime is managed by the corresponding MouseCallbackHandle
        mem::forget(wrapper);
    }
}

bitflags!{
    /// Flags for [named_window](fn.named_window.html)
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

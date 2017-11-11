//! highgui: high-level GUI
extern crate libc;

use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::{mem, ptr};

use core::CMat;
use Mat;

extern "C" {
    fn cv_named_window(name: *const c_char, flags: c_int);
    fn cv_destroy_window(name: *const c_char);
    fn cv_set_mouse_callback(
        name: *const c_char,
        on_mouse: Option<extern "C" fn(e: i32, x: i32, y: i32, f: i32, data: *mut c_void)>,
        userdata: *mut c_void,
    );
    fn cv_imshow(name: *const c_char, cmat: *mut CMat);
    fn cv_wait_key(delay_ms: c_int) -> c_int;
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

/// Displays the specified image in the window.
pub fn show_mat(name: &str, mat: &Mat) {
    let s = CString::new(name).unwrap();
    unsafe {
        cv_imshow((&s).as_ptr(), mat.inner);
    }
}

/// Waits for a pressed key.
///
/// The function waits for a key event infinitely in the case of `Delay::Forever` or
/// `Delay::Msec(msec <= 0)`, or for `msec` milliseconds otherwise. Since the OS has
/// a minimum time between switching threads, the function will not wait exactly `msec` ms, it will
/// wait at least `msec` ms, depending on what else is running on your computer at that time. It
/// returns the code of the pressed key or `None` if no key was pressed before the specified time had
/// elapsed.
///
/// **Note:** This function is the only method in HighGUI that can fetch and handle events, so it
/// needs to be called periodically for normal event processing unless HighGUI is used within an
/// environment that takes care of event processing.
///
/// **Note:** The function only works if there is at least one HighGUI window created and the window
/// is active. If there are several HighGUI windows, any of them can be active.
pub fn wait_key(delay: Delay) -> Option<c_int> {
    let delay_msec = match delay {
        Delay::Forever => 0,
        Delay::Msec(msec) => msec,
    };

    let result = unsafe { cv_wait_key(delay_msec) };

    if result >= 0 {
        Some(result)
    } else {
        None
    }
}

/// The duration wait_key should wait for a key press.
#[derive(Debug, Copy, Clone)]
pub enum Delay {
    /// Wait infinitely.
    Forever,
    /// Wait for the specified amount of milliseconds, or infinitely, if the specified value <= 0.
    Msec(c_int),
}

impl Default for Delay {
    fn default() -> Self {
        Delay::Forever
    }
}

/// Callback function for mouse events, primarily used in
/// [set_mouse_callback](fn.set_mouse_callback.html).
pub type MouseCallback<T> = fn(
    event_type: MouseEventType,
    x_pos: i32,
    y_pos: i32,
    event_flags: MouseEventFlags,
    user_data: &T,
);

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
    user_data: T,
) -> MouseCallbackHandle<T> {
    let boxed_wrapper = Box::new(MouseCallbackWrapper::<T> {
        callback: Box::new(callback),
        user_data,
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
    user_data: T,
}

impl<T: Sync + Send> MouseCallbackWrapper<T> {
    extern "C" fn extern_mouse_callback(
        event_type: i32,
        x_pos: i32,
        y_pos: i32,
        event_flags: i32,
        user_data: *mut c_void,
    ) {
        let wrapper = unsafe { Box::from_raw(user_data as *mut Self) };
        let true_callback: MouseCallback<T> = *(wrapper.callback);

        let event = unsafe { mem::transmute(event_type) };
        let flags = MouseEventFlags::from_bits_truncate(event_flags);

        true_callback(event, x_pos, y_pos, flags, &wrapper.user_data);

        // leak the callback here to let opencv use it for multiple callbacks.
        // its lifetime is managed by the corresponding MouseCallbackHandle
        mem::forget(wrapper);
    }
}

bitflags! {
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

bitflags! {
    /// Mouse event flags returned in the MouseCallback.
    pub struct MouseEventFlags: i32 {
        /// Indicates that the left mouse button is down.
        const CV_EVENT_FLAG_LBUTTON = 1;
        /// Indicates that the right mouse button is down.
        const CV_EVENT_FLAG_RBUTTON = 2;
        /// Indicates that the middle mouse button is down.
        const CV_EVENT_FLAG_MBUTTON = 4;
        /// Indicates that the CTRL key is pressed.
        const CV_EVENT_FLAG_CTRLKEY = 8;
        /// Indicates that the SHIFT key is pressed.
        const CV_EVENT_FLAG_SHIFTKEY = 16;
        /// Indicates that the ALT key is pressed.
        const CV_EVENT_FLAG_ALTKEY = 32;
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

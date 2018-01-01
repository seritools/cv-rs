use libc::{c_char, c_double, c_int};
use super::core::CMat;

#[link(name = "opencv-wrapper", kind = "static")]
extern "C" {
    pub fn cv_videocapture_new(index: c_int) -> *mut CvVideoCapture;
    pub fn cv_videocapture_from_file(path: *const c_char) -> *mut CvVideoCapture;
    pub fn cv_videocapture_is_opened(ccap: *const CvVideoCapture) -> bool;
    pub fn cv_videocapture_read(v: *mut CvVideoCapture, m: *mut CMat) -> bool;
    pub fn cv_videocapture_drop(cap: *mut CvVideoCapture);
    pub fn cv_videocapture_set(cap: *mut CvVideoCapture, property: c_int, value: c_double) -> bool;
    pub fn cv_videocapture_get(cap: *mut CvVideoCapture, property: c_int) -> c_double;

    pub fn cv_fourcc(c1: c_char, c2: c_char, c3: c_char, c4: c_char) -> c_int;
}

pub enum CvVideoCapture {}

unsafe impl Send for CvVideoCapture {}

use num;
use errors::*;

use super::*;

/// Opaque data struct for C bindings
#[derive(Clone, Copy, Debug)]
pub enum CMat {}
unsafe impl Send for CMat {}
impl CMat {
    pub fn new() -> *mut CMat {
        unsafe { cv_mat_new() }
    }
}

impl MatType {
    const CHANNEL_SHIFT: u8 = 3;
    const DEPTH_MASK: u8 = (1 << Self::CHANNEL_SHIFT) - 1;

    pub(crate) fn as_opencv_value(&self) -> c_int {
        (self.channels << Self::CHANNEL_SHIFT | self.depth as u16) as c_int
    }

    pub(crate) fn from_opencv_value(value: c_int) -> Result<MatType> {
        let depth = num::FromPrimitive::from_i32(value & (Self::DEPTH_MASK as c_int)).unwrap();
        let channels = (value >> Self::CHANNEL_SHIFT) as u16;

        MatType::new(depth, channels)
    }
}

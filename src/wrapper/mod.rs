use libc::{c_char, c_double, c_float, c_int, c_uchar, c_void, size_t, uint8_t};
use super::core::{Mat, MatType, Rect2f};

mod core;
mod functions;
mod videoio;
pub use self::core::*;
pub use self::functions::*;
pub use self::videoio::*;

/// A 4-element struct that is widely used to pass pixel values.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Scalar {
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
}

impl Scalar {
    /// Creates a new scalar object.
    pub fn new(v0: i32, v1: i32, v2: i32, v3: i32) -> Self {
        Scalar {
            v0: v0,
            v1: v1,
            v2: v2,
            v3: v3,
        }
    }
}

/// 2D integer points specified by its coordinates `x` and `y`.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2i {
    /// x coordinate
    pub x: i32,

    /// y coordinate
    pub y: i32,
}

/// 2D floating points specified by its coordinates `x` and `y`.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2f {
    /// x coordinate
    pub x: f32,

    /// y coordinate
    pub y: f32,
}

/// Represents the integral size (width and height) of an image or rectangle.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2i {
    /// The width.
    pub width: c_int,

    /// The height.
    pub height: c_int,
}

impl Size2i {
    /// Initializes a `Size2i` with the specified width and height.
    pub fn new(width: c_int, height: c_int) -> Self {
        Size2i {
            width: width,
            height: height,
        }
    }
}

/// `Size2f` struct is used for specifying the size (`width` and `height` as
/// `f32`) of an image or rectangle.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Size2f {
    /// width
    pub width: f32,

    /// height
    pub height: f32,
}

/// The `Rect` defines a rectangle in integer.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Rect {
    /// x coordinate of the left-top corner
    pub x: i32,
    /// y coordinate of the left-top corner
    pub y: i32,
    /// width of this rectangle
    pub width: i32,
    /// height of this rectangle
    pub height: i32,
}

impl Rect {
    /// Creates a new `Rect` with (x, y, width, height) parameters.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    /// Scales the rectangle by the specified ratio.
    pub fn scale(&self, ratio: f32) -> Rect {
        let new_x = ((1.0 - ratio) * (self.width as f32) / 2.0) as i32 + self.x;
        let new_y = ((1.0 - ratio) * (self.height as f32) / 2.0) as i32 + self.y;
        let new_w = ((self.width as f32) * ratio) as i32;
        let new_h = ((self.height as f32) * ratio) as i32;
        Rect {
            x: new_x,
            y: new_y,
            width: new_w,
            height: new_h,
        }
    }

    /// Normalize the rectangle according to the image (if the rectangle is
    /// inside the image, then the result should be all within (0, 1).
    pub fn normalize_to_mat(&self, mat: &Mat) -> Rect2f {
        let cols = mat.cols();
        let rows = mat.rows();
        Rect2f {
            x: (self.x as f32) / (cols as f32),
            y: (self.y as f32) / (rows as f32),
            width: (self.width as f32) / (cols as f32),
            height: (self.height as f32) / (rows as f32),
        }
    }
}

#[repr(C)]
pub struct CVecOfRect {
    pub array: *mut Rect,
    pub size: usize,
}

impl Default for CVecOfRect {
    fn default() -> Self {
        CVecOfRect {
            array: ::std::ptr::null_mut::<Rect>(),
            size: 0,
        }
    }
}

impl Drop for CVecOfRect {
    fn drop(&mut self) {
        extern "C" {
            fn cv_vec_of_rect_drop(_: *mut CVecOfRect);
        }
        unsafe {
            cv_vec_of_rect_drop(self);
        }
    }
}

impl CVecOfRect {
    pub fn rustify(self) -> Vec<Rect> {
        (0..self.size)
            .map(|i| unsafe { *(self.array.offset(i as isize)) })
            .collect::<Vec<_>>()
    }
}

#[repr(C)]
pub struct CVecDouble {
    array: *mut c_double,
    size: usize,
}

impl CVecDouble {
    pub fn rustify(self) -> Vec<f64> {
        (1..self.size)
            .map(|i| unsafe { *(self.array.offset(i as isize)) })
            .collect::<Vec<_>>()
    }
}

impl Default for CVecDouble {
    fn default() -> Self {
        CVecDouble {
            array: ::std::ptr::null_mut::<c_double>(),
            size: 0,
        }
    }
}

#[repr(C)]
pub struct ImencodeResult {
    pub status: bool,
    pub buf: *mut u8,
    pub size: usize,
}

pub enum CHogDescriptor {}

/// The opaque type for C
pub enum CCascadeClassifier {}

#[derive(Debug, Clone, Copy)]
/// Opaque type for C/C++ SvmDetector object
pub enum CSvmDetector {}

pub enum CTermCriteria {}

/// This struct represents a rotated (i.e. not up-right) rectangle. Each
/// rectangle is specified by the center point (mass center), length of each
/// side (represented by `Size2f`) and the rotation angle in degrees.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct RotatedRect {
    center: Point2f,
    size: Size2f,
    angle: f32,
}

impl RotatedRect {
    /// Return 4 vertices of the rectangle.
    pub fn points(&self) -> [Point2f; 4] {
        let angle = self.angle * ::std::f32::consts::PI / 180.0;

        let b = angle.cos() * 0.5;
        let a = angle.sin() * 0.5;

        let mut pts: [Point2f; 4] = [Point2f::default(); 4];
        pts[0].x = self.center.x - a * self.size.height - b * self.size.width;
        pts[0].y = self.center.y + b * self.size.height - a * self.size.width;
        pts[1].x = self.center.x + a * self.size.height - b * self.size.width;
        pts[1].y = self.center.y - b * self.size.height - a * self.size.width;

        pts[2].x = 2.0 * self.center.x - pts[0].x;
        pts[2].y = 2.0 * self.center.y - pts[0].y;
        pts[3].x = 2.0 * self.center.x - pts[1].x;
        pts[3].y = 2.0 * self.center.y - pts[1].y;
        pts
    }

    /// Return the minimal up-right rectangle containing the rotated rectangle
    pub fn bounding_rect(&self) -> Rect {
        let pt = self.points();
        let x = pt.iter().map(|p| p.x).fold(0. / 0., f32::min).floor() as i32;
        let y = pt.iter().map(|p| p.y).fold(0. / 0., f32::min).floor() as i32;

        let width = pt.iter().map(|p| p.x).fold(0. / 0., f32::max).ceil() as i32 - x + 1;
        let height = pt.iter().map(|p| p.y).fold(0. / 0., f32::max).ceil() as i32 - y + 1;
        Rect::new(x, y, width, height)
    }
}

/// Opaque VideoWriter type.
pub enum CvVideoWriter {}

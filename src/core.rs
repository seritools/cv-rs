//! Core data structures in OpenCV

use libc::c_int;
use errors::*;

use super::wrapper::*;

/// This wraps OpenCV's `Mat` class which is designed for n-dimensional dense
/// array. It's the most widely used data structure in image/video processing
/// since images are often stored as `Mat`.
#[derive(Debug)]
pub struct Mat {
    /// Pointer to the actual C/C++ data structure
    pub(crate) inner: *mut CMat,
}

impl Mat {
    #[inline]
    /// Creates a `Mat` object from raw `CMat` pointer. This will read the rows
    /// and cols of the image.
    pub(crate) fn from_raw(raw: *mut CMat) -> Mat {
        Mat {
            inner: raw,
        }
    }

    /// Creates an empty `Mat` struct.
    pub fn new() -> Mat {
        Mat::from_raw(CMat::new())
    }

    /// Creates a new `Mat` from buffer. Note that internally opencv function
    /// won't take ownership of the Mat, but when we call `drop`, it will
    /// deallocate the memory. To prevent double-freeing, you must `mem::forget`
    /// it after use.
    ///
    /// The following example shows how to get the data from an image and create
    /// a new image with the data (also forgets it).
    ///
    /// ```rust,ignore
    /// let buffer = image.data();
    /// let size = image.size();
    /// let s = (size.width * size.height * 3) as usize;
    ///
    /// let mut vec = Vec::with_capacity(s);
    /// unsafe {
    ///   vec.set_len(s);
    ///   copy(buffer, vec.as_mut_ptr(), s);
    /// }
    /// let new_image = Mat::from_buffer(
    ///   size.height, size.width, CvType::Cv8UC3 as i32, &vec);
    ///
    ///  // . . . use new_image here, such as new_image.show(..) . . .
    ///
    /// ::std::mem::forget(new_image);
    /// ```
    pub fn from_buffer(rows: i32, cols: i32, cv_type: i32, buf: &Vec<u8>) -> Mat {
        let raw = unsafe { cv_mat_from_buffer(rows, cols, cv_type, buf.as_ptr()) };
        Mat::from_raw(raw)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn with_size(rows: c_int, cols: c_int, mat_type: MatType) -> Self {
        let m = unsafe { cv_mat_new_with_size(rows, cols, mat_type.as_opencv_value()) };
        Mat::from_raw(m)
    }

    /// Create an empty `Mat` with specific size (rows, cols and types).
    pub fn zeros(rows: i32, cols: i32, t: i32) -> Self {
        let m = unsafe { cv_mat_zeros(rows, cols, t) };
        Mat::from_raw(m)
    }

    /// Returns the raw data (as a uchar pointer)
    pub fn data(&self) -> *const u8 {
        unsafe { cv_mat_data(self.inner) }
    }

    /// Returns the total number of array elements. The method returns the
    /// number of array elements (a number of pixels if the array represents an
    /// image). For example, images with 1920x1080 resolution will return
    /// 2073600.
    pub fn total(&self) -> usize {
        unsafe { cv_mat_total(self.inner) }
    }

    /// Returns the matrix element size in bytes. The method returns the matrix
    /// element size in bytes. For example, if the matrix type is CV_16SC3 , the
    /// method returns 3*sizeof(short) or 6.
    pub fn elem_size(&self) -> usize {
        unsafe { cv_mat_elem_size(self.inner) }
    }

    /// Returns the height of this matrix.
    pub fn rows(&self) -> c_int {
        unsafe { cv_mat_rows(self.inner) }
    }

    /// Returns the width of this matrix.
    pub fn cols(&self) -> c_int {
        unsafe { cv_mat_cols(self.inner) }
    }

    /// Returns the size of this matrix.
    pub fn size(&self) -> Size2i {
        Size2i {
            width: self.cols(),
            height: self.rows(),
        }
    }

    /// Check if the `Mat` is valid or not.
    pub fn is_valid(&self) -> bool {
        unsafe { cv_mat_is_valid(self.inner) }
    }

    /// Return a region of interest from a `Mat` specfied by a `Rect`.
    pub fn roi(&self, rect: Rect) -> Mat {
        let cmat = unsafe { cv_mat_roi(self.inner, rect) };
        Mat::from_raw(cmat)
    }

    /// Apply a mask to myself.
    // TODO(benzh): Find the right reference in OpenCV for this one. Provide a
    // shortcut for `image &= mask`
    pub fn logic_and(&mut self, mask: Mat) {
        unsafe {
            cv_mat_logic_and(self.inner, mask.inner);
        }
    }

    /// Flips an image around vertical, horizontal, or both axes.
    pub fn flip(&mut self, code: FlipCode) {
        let code = match code {
            FlipCode::XAxis => 0,
            FlipCode::YAxis => 1,
            FlipCode::XYAxis => -1,
        };
        unsafe {
            cv_mat_flip(self.inner, code);
        }
    }

    /// Returns the image's type.
    pub fn mat_type(&self) -> MatType {
        let raw_type = unsafe { cv_mat_type(self.inner) };

        // all opencv types are ok, so unwrap instead of returning Result
        MatType::from_opencv_value(raw_type).unwrap()
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            cv_mat_drop(self.inner);
        }
    }
}

// TODO(benzh): Should consider Unique<T>,
// https://github.com/rust-lang/rust/issues/27730
unsafe impl Send for Mat {}

/// The `Rect2f` are rectangles in float.
#[derive(Default, Debug, Clone, Copy)]
pub struct Rect2f {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect2f {
    /// Normalize the rectangle according to the image. This will restore the
    /// Rect in absolute pixel numbers.
    pub fn normalize_to_mat(&self, mat: &Mat) -> Rect {
        let cols = mat.cols();
        let rows = mat.rows();
        Rect {
            x: (self.x * cols as f32) as i32,
            y: (self.y * rows as f32) as i32,
            width: (self.width * cols as f32) as i32,
            height: (self.height * rows as f32) as i32,
        }
    }
}

/// Line type
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineTypes {
    /// Default type
    Filled = -1,

    /// 4-connected line
    Line4 = 4,

    /// 8-connected line
    Line8 = 8,

    /// antialiased line
    LineAA = 16,
}

/// A flag to specify how to flip the image. see
/// [Mat::flip](struct.Mat.html#method.flip)
#[derive(Debug, Clone, Copy)]
pub enum FlipCode {
    /// Along x-axis: dst[i, j] = src[src.rows - i - 1, j]
    XAxis,
    /// Along y-axis: dst[i, j] = src[i, src.cols - j - 1]
    YAxis,
    /// Along both axis: dst[i, j] = src[src.rows - i - 1, src.cols - j - 1]
    XYAxis,
}

/// Here is the `CvType` in an easy-to-read table.
///
/// |        | C1 | C2 | C3 | C4 | C(5) | C(6) | C(7) | C(8) |
/// |--------|----|----|----|----|------|------|------|------|
/// | CV_8U  |  0 |  8 | 16 | 24 |   32 |   40 |   48 |   56 |
/// | CV_8S  |  1 |  9 | 17 | 25 |   33 |   41 |   49 |   57 |
/// | CV_16U |  2 | 10 | 18 | 26 |   34 |   42 |   50 |   58 |
/// | CV_16S |  3 | 11 | 19 | 27 |   35 |   43 |   51 |   59 |
/// | CV_32S |  4 | 12 | 20 | 28 |   36 |   44 |   52 |   60 |
/// | CV_32F |  5 | 13 | 21 | 29 |   37 |   45 |   53 |   61 |
/// | CV_64F |  6 | 14 | 22 | 30 |   38 |   46 |   54 |   62 |
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum CvType {
    /// 8 bit, single channel (grey image)
    Cv8UC1 = 0,

    /// 8 bit, two channel (rarelly seen)
    Cv8UC2 = 8,

    /// 8 bit, three channels (RGB image)
    Cv8UC3 = 16,
}

/// Channel depths of mats.
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum MatDepth {
    /// `u8`
    Unsigned8 = 0,
    /// `i8`
    Signed8 = 1,
    /// `u16`
    Unsigned16 = 2,
    /// `i16`
    Signed16 = 3,
    /// `i32`
    Signed32 = 4,
    /// `f32`
    Float32 = 5,
    /// `f64`
    Float64 = 6,
    /// Custom, user-defined depth
    UserDefined = 7
}

/// Represents a valid mat type.
///
/// A mat type consists of a channel count and the depth per channel.
/// The maximum supported channel count is 511.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MatType {
    pub(crate) depth: MatDepth,
    pub(crate) channels: u16,
}

impl MatType {
    pub(crate) const MAX_CHANNELS: u16 = 512;

    /// Initializes a new mat type.
    ///
    /// Returns `Ok<MatType>` if the channel count supplied is 511 or less.
    pub fn new(depth: MatDepth, channels: u16) -> Result<Self> {
        if channels < Self::MAX_CHANNELS {
            Ok(Self {
                depth,
                channels
            })
        } else {
            Err(ErrorKind::UnsupportedChannelCount(channels, Self::MAX_CHANNELS).into())
        }
    }

    /// Returns the depth per channel.
    pub fn depth(&self) -> MatDepth {
        self.depth
    }

    /// Returns the channel count.
    pub fn channels(&self) -> u16 {
        self.channels
    }
}

// =============================================================================
// core array
// =============================================================================

/// Normalization type. Please refer to [OpenCV's
/// documentation](http://docs.cv.org/trunk/d2/de8/group__core__array.html).
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum NormTypes {
    /// Normalized using `max`
    NormInf = 1,
    /// Normalized using L1 distance
    NormL1 = 2,
    /// Normalized using L2 distance
    NormL2 = 4,
    /// Normalized using L2 sqr distance
    NormL2Sqr = 5,
    /// Normalized using hamming distance
    NormHamming = 6,
    /// Normalized using hamming2 distance
    NormHamming2 = 7,
    /// Normalized using relative distance
    NormRelative = 8,
    /// Normalized using minmax distance
    NormMinMax = 32,
}

impl Mat {
    /// Check if Mat elements lie between the elements of two other arrays
    /// (lowerb and upperb). The output Mat has the same size as `self` and
    /// CV_8U type.
    pub fn in_range(&self, lowerb: Scalar, upperb: Scalar) -> Mat {
        let m = CMat::new();
        unsafe { cv_in_range(self.inner, lowerb, upperb, m) }
        Mat::from_raw(m)
    }

    /// Copy specified channels from `self` to the specified channels of output
    /// `Mat`.
    // TODO(benzh) Avoid using raw pointers but rather take a vec for `from_to`?
    // The usage (self.depth) here is buggy, it should actually be the type!
    pub fn mix_channels(
        &self,
        nsrcs: isize,
        ndsts: isize,
        from_to: *const i32,
        npairs: isize,
    ) -> Mat {
        let m = Mat::with_size(self.rows(), self.cols(), self.mat_type());
        unsafe {
            cv_mix_channels(self.inner, nsrcs, m.inner, ndsts, from_to, npairs);
        }
        m
    }

    /// Normalize the Mat according to the normalization type.
    pub fn normalize(&self, alpha: f64, beta: f64, t: NormTypes) -> Mat {
        let m = CMat::new();
        unsafe { cv_normalize(self.inner, m, alpha, beta, t as i32) }
        Mat::from_raw(m)
    }

    /// Computes bitwise conjunction between two Mat
    pub fn and(&self, another: &Mat) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_and(self.inner, another.inner, m) }
        Mat::from_raw(m)
    }

    /// Computes bitwise disjunction between two Mat
    pub fn or(&self, another: &Mat) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_or(self.inner, another.inner, m) }
        Mat::from_raw(m)
    }

    /// Computes bitwise "exclusive or" between two Mat
    pub fn xor(&self, another: &Mat) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_xor(self.inner, another.inner, m) }
        Mat::from_raw(m)
    }

    /// Computes bitwise "exclusive or" between two Mat
    pub fn not(&self) -> Mat {
        let m = CMat::new();
        unsafe { cv_bitwise_not(self.inner, m) }
        Mat::from_raw(m)
    }

    /// Counts non-zero array elements.
    pub fn count_non_zero(&self) -> i32 {
        unsafe { cv_count_non_zero(self.inner) }
    }
}

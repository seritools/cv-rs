use super::*;

#[link(name = "opencv-wrapper", kind = "static")]
extern "C" {
    pub fn cv_mat_new() -> *mut CMat;
    pub fn cv_mat_new_with_size(rows: c_int, cols: c_int, mat_type: c_int) -> *mut CMat;
    pub fn cv_mat_zeros(rows: c_int, cols: c_int, mat_type: c_int) -> *mut CMat;
    pub fn cv_mat_is_valid(mat: *mut CMat) -> bool;
    pub fn cv_mat_rows(cmat: *const CMat) -> c_int;
    pub fn cv_mat_cols(cmat: *const CMat) -> c_int;
    pub fn cv_mat_type(cmat: *const CMat) -> c_int;
    pub fn cv_mat_data(cmat: *const CMat) -> *const c_uchar;
    pub fn cv_mat_total(cmat: *const CMat) -> size_t;
    pub fn cv_mat_elem_size(cmat: *const CMat) -> size_t;
    pub fn cv_mat_roi(cmat: *const CMat, rect: Rect) -> *mut CMat;
    pub fn cv_mat_logic_and(cimage: *mut CMat, cmask: *const CMat);
    pub fn cv_mat_flip(src: *mut CMat, code: c_int);
    pub fn cv_mat_drop(mat: *mut CMat);
    pub fn cv_in_range(cmat: *const CMat, lowerb: Scalar, upperb: Scalar, dst: *mut CMat);
    pub fn cv_mix_channels(
        cmat: *const CMat,
        nsrcs: isize,
        dst: *mut CMat,
        ndsts: isize,
        from_to: *const i32,
        npairs: isize,
    );
    pub fn cv_normalize(
        csrc: *const CMat,
        cdst: *mut CMat,
        alpha: c_double,
        beta: c_double,
        norm_type: c_int,
    );

    pub fn cv_bitwise_and(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    pub fn cv_bitwise_not(src: *const CMat, dst: *mut CMat);
    pub fn cv_bitwise_or(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    pub fn cv_bitwise_xor(src1: *const CMat, src2: *const CMat, dst: *mut CMat);
    pub fn cv_count_non_zero(src: *const CMat) -> i32;

    pub fn cv_named_window(name: *const c_char, flags: c_int);
    pub fn cv_destroy_window(name: *const c_char);
    pub fn cv_set_mouse_callback(
        name: *const c_char,
        on_mouse: Option<extern "C" fn(e: i32, x: i32, y: i32, f: i32, data: *mut c_void)>,
        userdata: *mut c_void,
    );
    pub fn cv_imshow(name: *const c_char, cmat: *mut CMat);
    pub fn cv_wait_key(delay_ms: c_int) -> c_int;

    pub fn cv_imread(input: *const c_char, flags: c_int) -> *mut CMat;
    pub fn cv_imdecode(buf: *const uint8_t, l: size_t, m: c_int) -> *mut CMat;
    pub fn cv_imencode(
        ext: *const c_char,
        inner: *const CMat,
        flag_ptr: *const c_int,
        flag_size: size_t,
    ) -> ImencodeResult;

    pub fn cv_rectangle(
        cmat: *mut CMat,
        rect: Rect,
        color: Scalar,
        thickness: c_int,
        linetype: c_int,
    );

    pub fn cv_ellipse(
        cmat: *mut CMat,
        center: Point2i,
        axes: Size2i,
        angle: c_double,
        start_angle: c_double,
        end_angle: c_double,
        color: Scalar,
        thickness: c_int,
        linetype: c_int,
        shift: c_int,
    );

    pub fn cv_cvt_color(cmat: *const CMat, output: *mut CMat, code: i32);
    pub fn cv_pyr_down(cmat: *const CMat, output: *mut CMat);
    pub fn cv_resize(
        from: *const CMat,
        to: *mut CMat,
        dsize: Size2i,
        fx: c_double,
        fy: c_double,
        interpolation: c_int,
    );
    pub fn cv_calc_hist(
        cimages: *const CMat,
        nimages: i32,
        channels: *const c_int,
        cmask: *const CMat,
        chist: *mut CMat,
        dims: c_int,
        hist_size: *const c_int,
        ranges: *const *const c_float,
    );
    pub fn cv_calc_back_project(
        cimages: *const CMat,
        nimages: c_int,
        channels: *const c_int,
        chist: *const CMat,
        cback_project: *mut CMat,
        ranges: *const *const c_float,
    );

    pub fn cv_hog_new() -> *mut CHogDescriptor;
    pub fn cv_hog_drop(hog: *mut CHogDescriptor);
    pub fn cv_hog_set_svm_detector(hog: *mut CHogDescriptor, svm: *mut CSvmDetector);
    pub fn cv_hog_detect(
        hog: *mut CHogDescriptor,
        image: *mut CMat,
        objs: *mut CVecOfRect,
        weights: *mut CVecDouble,
        win_stride: Size2i,
        padding: Size2i,
        scale: c_double,
        final_threshold: c_double,
        use_means_shift: bool,
    );

    pub fn cv_term_criteria_new(t: i32, count: i32, epsilon: f64) -> *mut CTermCriteria;
    pub fn cv_term_criteria_drop(criteria: *mut CTermCriteria);
    pub fn cv_camshift(image: *mut CMat, w: Rect, c_criteria: *const CTermCriteria) -> RotatedRect;

    pub fn cv_videowriter_default() -> *mut CvVideoWriter;
    pub fn cv_videowriter_new(
        path: *const c_char,
        fourcc: c_int,
        fps: c_double,
        frame_size: Size2i,
        is_color: bool,
    ) -> *mut CvVideoWriter;
    pub fn cv_videowriter_drop(w: *mut CvVideoWriter);

    pub fn cv_videowriter_open(
        w: *mut CvVideoWriter,
        path: *const c_char,
        fourcc: c_int,
        fps: c_double,
        frame_size: Size2i,
        is_color: bool,
    ) -> bool;
    pub fn cv_videowriter_is_opened(w: *mut CvVideoWriter) -> bool;
    pub fn cv_videowriter_write(w: *mut CvVideoWriter, m: *mut CMat);
    pub fn cv_videowriter_set(w: *mut CvVideoWriter, property: c_int, value: c_double) -> bool;
    pub fn cv_videowriter_get(w: *mut CvVideoWriter, property: c_int) -> c_double;
}

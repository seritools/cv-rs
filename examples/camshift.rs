use std::sync::{Arc, Mutex};

extern crate cv;

use cv::*;
use cv::highgui;
use cv::highgui::{MouseEventType, WindowFlags};
use cv::imgproc::*;
use cv::video::tracking::*;
use cv::videoio::*;

#[derive(Copy, Clone)]
struct SelectionStatus {
    selection: Rect,
    status: bool,
}

fn on_mouse(e: i32, x: i32, y: i32, _: i32, data: &Arc<Mutex<SelectionStatus>>) {
    let event: MouseEventType = unsafe { std::mem::transmute(e) };


    match event {
        MouseEventType::LButtonDown => {
            let mut selection_status = data.lock().unwrap();
            selection_status.selection.x = x;
            selection_status.selection.y = y;
        }
        MouseEventType::LButtonUp => {
            let mut selection_status = data.lock().unwrap();
            selection_status.selection.width = x - selection_status.selection.x;
            selection_status.selection.height = y - selection_status.selection.y;

            if selection_status.selection.width > 0 && selection_status.selection.height > 0 {
                selection_status.status = true;
            }
        }
        _ => {}
    }
}

fn main() {
    let selection_status = Arc::new(Mutex::new(SelectionStatus {
        selection: Rect::default(),
        status: false,
    }));

    let cap = VideoCapture::new(0);
    assert!(cap.is_open());

    highgui::create_named_window("Window", WindowFlags::WINDOW_AUTOSIZE);

    let callback_handle = highgui::set_mouse_callback("Window", on_mouse, selection_status.clone());

    let mut is_tracking = false;

    let mut hist = Mat::new();
    let hsize = 16;
    let hranges = [0_f32, 180_f32];
    let phranges: [*const f32; 1] = [&hranges[0] as *const f32];
    let mut track_window = Rect::default();

    while let Some(mut m) = cap.read() {
        m.flip(FlipCode::YAxis);

        let hsv = m.cvt_color(ColorConversionCodes::BGR2HSV);

        let ch = [0, 0];
        let hue = hsv.mix_channels(1, 1, &ch[0] as *const i32, 1);
        let mask = hsv.in_range(Scalar::new(0, 30, 10, 0), Scalar::new(180, 256, 256, 0));

        {
            let mut current = selection_status.lock().unwrap();

            if current.status {
                println!("Initialize tracking, setting up CAMShift search");
                let selection = current.selection;
                let roi = hue.roi(selection);
                let maskroi = mask.roi(selection);

                let raw_hist = roi.calc_hist(
                    std::ptr::null(),
                    maskroi,
                    1,
                    &hsize,
                    &phranges[0] as *const *const f32,
                );
                hist = raw_hist.normalize(0.0, 255.0, NormTypes::NormMinMax);

                track_window = selection;
                m.rectangle(selection);
                current.status = false;
                is_tracking = true;
            }
        }


        if is_tracking {
            let mut back_project =
                hue.calc_back_project(std::ptr::null(), &hist, &phranges[0] as *const *const f32);
            back_project.logic_and(mask);
            let criteria = TermCriteria::new(TermType::Count, 10, 1.0);
            let track_box = back_project.camshift(track_window, &criteria);

            m.rectangle(track_box.bounding_rect());
        }

        m.show("Window", 30).unwrap();
    }

    // remove the mouse callback from the window
    std::mem::drop(callback_handle);
}

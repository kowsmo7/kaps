use std::os::raw::{c_int, c_uint, c_ulong};
use x11::xlib;

#[derive(Debug, Copy, Clone)]
pub struct Region {
    pub x: c_int,
    pub y: c_int,
    pub w: c_uint,
    pub h: c_uint,
}

impl Region {
    #[inline]
    pub fn new(x: c_int, y: c_int, w: c_uint, h: c_uint) -> Region {
        Region { x, y, w, h }
    }
}

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn from_x11(image: crate::x11::Image, snip: Region) -> Image {
        let Region { w: width, h: height, x: offset_x, y: offset_y } = snip;
        let mut data = Vec::with_capacity((width * height * 4) as usize);

        let raw_img = unsafe { *image.ptr };
        let red_mask = raw_img.red_mask;
        let green_mask = raw_img.green_mask;
        let blue_mask = raw_img.blue_mask;

        let red_shift = c_ulong::trailing_zeros(red_mask);
        let green_shift = c_ulong::trailing_zeros(green_mask);
        let blue_shift = c_ulong::trailing_zeros(blue_mask);

        for y in 0..height {
            for x in 0..width {
                let px = unsafe {
                    xlib::XGetPixel(
                        image.ptr as *mut xlib::XImage,
                        offset_x + x as c_int,
                        offset_y + y as c_int,
                    )
                };

                let r = (px & red_mask) >> red_shift;
                let g = (px & green_mask) >> green_shift;
                let b = (px & blue_mask) >> blue_shift;

                data.push(r as u8);
                data.push(g as u8);
                data.push(b as u8);
                data.push(0xFF);
            }
        }

        Image {
            data,
            width,
            height,
        }
    }
}


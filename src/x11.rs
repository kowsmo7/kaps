use crate::image::Region;
use anyhow::ensure;
use std::mem;
use std::ptr;
use x11::xlib;

pub struct Display {
    pub ptr: *const xlib::Display,
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.ptr as _);
        }
    }
}

pub struct Image {
    pub ptr: *const xlib::XImage,
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyImage(self.ptr as _);
        }
    }
}

pub fn connect() -> anyhow::Result<Display> { 
    let ptr = unsafe { xlib::XOpenDisplay(ptr::null()) } as *const xlib::Display;
    ensure!(!ptr.is_null(), "Failed to connect to the Xorg server.");

    Ok(Display { ptr })
}

pub fn display_size(display: &Display) -> anyhow::Result<Region> {
    unsafe {
        let root = xlib::XDefaultRootWindow(display.ptr as _);
        let mut attrs = mem::zeroed();
        xlib::XGetWindowAttributes(display.ptr as _, root, &mut attrs);

        Ok(Region::new(
            attrs.x,
            attrs.y,
            attrs.width as _,
            attrs.height as _,
        ))
    }
}

pub fn root_image(
    display: &Display,
    size: Region,
) -> anyhow::Result<Image> {
    unsafe {
        Ok(Image {
            ptr: xlib::XGetImage(
                display.ptr as _,
                xlib::XDefaultRootWindow(display.ptr as _),
                size.x,
                size.y,
                size.w,
                size.h,
                xlib::XAllPlanes(),
                xlib::ZPixmap,
            )
        })
    }
}


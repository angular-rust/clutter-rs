// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::Point;
use ffi;
use glib::translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Rect(Boxed<ffi::ClutterRect>);

    match fn {
        copy => |ptr| ffi::clutter_rect_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_rect_free(ptr),
        get_type => || ffi::clutter_rect_get_type(),
    }
}

impl Rect {
    pub fn alloc() -> Rect {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::clutter_rect_alloc()) }
    }

    pub fn clamp_to_pixel(&mut self) {
        unsafe {
            ffi::clutter_rect_clamp_to_pixel(self.to_glib_none_mut().0);
        }
    }

    pub fn contains_point(&mut self, point: &mut Point) -> bool {
        unsafe {
            from_glib(ffi::clutter_rect_contains_point(
                self.to_glib_none_mut().0,
                point.to_glib_none_mut().0,
            ))
        }
    }

    pub fn contains_rect(&mut self, b: &mut Rect) -> bool {
        unsafe {
            from_glib(ffi::clutter_rect_contains_rect(
                self.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
            ))
        }
    }

    pub fn equals(&mut self, b: &mut Rect) -> bool {
        unsafe {
            from_glib(ffi::clutter_rect_equals(
                self.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
            ))
        }
    }

    pub fn get_center(&mut self) -> Point {
        unsafe {
            let mut center = Point::uninitialized();
            ffi::clutter_rect_get_center(self.to_glib_none_mut().0, center.to_glib_none_mut().0);
            center
        }
    }

    pub fn get_height(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_height(self.to_glib_none_mut().0) }
    }

    pub fn get_width(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_width(self.to_glib_none_mut().0) }
    }

    pub fn get_x(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_x(self.to_glib_none_mut().0) }
    }

    pub fn get_y(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_y(self.to_glib_none_mut().0) }
    }

    pub fn init(&mut self, x: f32, y: f32, width: f32, height: f32) -> Option<Rect> {
        unsafe {
            from_glib_none(ffi::clutter_rect_init(
                self.to_glib_none_mut().0,
                x,
                y,
                width,
                height,
            ))
        }
    }

    pub fn inset(&mut self, d_x: f32, d_y: f32) {
        unsafe {
            ffi::clutter_rect_inset(self.to_glib_none_mut().0, d_x, d_y);
        }
    }

    pub fn intersection(&mut self, b: &mut Rect) -> Option<Rect> {
        unsafe {
            let mut res = Rect::uninitialized();
            let ret = from_glib(ffi::clutter_rect_intersection(
                self.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
                res.to_glib_none_mut().0,
            ));
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }

    pub fn normalize(&mut self) -> Option<Rect> {
        unsafe { from_glib_full(ffi::clutter_rect_normalize(self.to_glib_none_mut().0)) }
    }

    pub fn offset(&mut self, d_x: f32, d_y: f32) {
        unsafe {
            ffi::clutter_rect_offset(self.to_glib_none_mut().0, d_x, d_y);
        }
    }

    pub fn union(&mut self, b: &mut Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            ffi::clutter_rect_union(
                self.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn zero() -> Option<Rect> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::clutter_rect_zero()) }
    }
}

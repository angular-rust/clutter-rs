use crate::Point;
use glib::translate::*;
use std::mem;

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
    /// Creates a new, empty `Rect`.
    ///
    /// You can use `Rect::init` to initialize the returned rectangle,
    /// for instance:
    ///
    ///
    /// ```text
    ///   rect = clutter_rect_init (clutter_rect_alloc (), x, y, width, height);
    /// ```
    ///
    /// # Returns
    ///
    /// the newly allocated `Rect`.
    ///  Use `Rect::free` to free its resources
    pub fn alloc() -> Rect {
        unsafe { from_glib_full(ffi::clutter_rect_alloc()) }
    }

    /// Rounds the origin of `self` downwards to the nearest integer, and rounds
    /// the size of `self` upwards to the nearest integer, so that `self` is
    /// updated to the smallest rectangle capable of fully containing the
    /// original, fractional rectangle.
    pub fn clamp_to_pixel(&mut self) {
        unsafe {
            ffi::clutter_rect_clamp_to_pixel(self.to_glib_none_mut().0);
        }
    }

    /// Checks whether `point` is contained by `self`, after normalizing the
    /// rectangle.
    /// ## `point`
    /// the point to check
    ///
    /// # Returns
    ///
    /// `true` if the `point` is contained by `self`.
    pub fn contains_point(&mut self, point: &mut Point) -> bool {
        unsafe {
            from_glib(ffi::clutter_rect_contains_point(
                self.to_glib_none_mut().0,
                point.to_glib_none_mut().0,
            ))
        }
    }

    /// Checks whether `self` contains `b`.
    ///
    /// The first rectangle contains the second if the union of the
    /// two `Rect` is equal to the first rectangle.
    /// ## `b`
    /// a `Rect`
    ///
    /// # Returns
    ///
    /// `true` if the first rectangle contains the second.
    pub fn contains_rect(&mut self, b: &mut Rect) -> bool {
        unsafe {
            from_glib(ffi::clutter_rect_contains_rect(
                self.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
            ))
        }
    }

    /// Checks whether `self` and `b` are equals.
    ///
    /// This function will normalize both `self` and `b` before comparing
    /// their origin and size.
    /// ## `b`
    /// a `Rect`
    ///
    /// # Returns
    ///
    /// `true` if the rectangles match in origin and size.
    pub fn equals(&mut self, b: &mut Rect) -> bool {
        unsafe {
            from_glib(ffi::clutter_rect_equals(
                self.to_glib_none_mut().0,
                b.to_glib_none_mut().0,
            ))
        }
    }

    /// Retrieves the center of `self`, after normalizing the rectangle,
    /// and updates `center` with the correct coordinates.
    /// ## `center`
    /// a `Point`
    pub fn get_center(&mut self) -> Point {
        unsafe {
            let mut center = Point::uninitialized();
            ffi::clutter_rect_get_center(self.to_glib_none_mut().0, center.to_glib_none_mut().0);
            center
        }
    }

    /// Retrieves the height of `self`.
    ///
    /// # Returns
    ///
    /// the height of the rectangle
    pub fn get_height(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_height(self.to_glib_none_mut().0) }
    }

    /// Retrieves the width of `self`.
    ///
    /// # Returns
    ///
    /// the width of the rectangle
    pub fn get_width(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_width(self.to_glib_none_mut().0) }
    }

    /// Retrieves the X coordinate of the origin of `self`.
    ///
    /// # Returns
    ///
    /// the X coordinate of the origin of the rectangle
    pub fn get_x(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_x(self.to_glib_none_mut().0) }
    }

    /// Retrieves the Y coordinate of the origin of `self`.
    ///
    /// # Returns
    ///
    /// the Y coordinate of the origin of the rectangle
    pub fn get_y(&mut self) -> f32 {
        unsafe { ffi::clutter_rect_get_y(self.to_glib_none_mut().0) }
    }

    /// Initializes a `Rect` with the given origin and size.
    /// ## `x`
    /// X coordinate of the origin
    /// ## `y`
    /// Y coordinate of the origin
    /// ## `width`
    /// width of the rectangle
    /// ## `height`
    /// height of the rectangle
    ///
    /// # Returns
    ///
    /// the updated rectangle
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

    /// Normalizes the `self` and offsets its origin by the `d_x` and `d_y` values;
    /// the size is adjusted by (2 * `d_x`, 2 * `d_y`).
    ///
    /// If `d_x` and `d_y` are positive the size of the rectangle is decreased; if
    /// the values are negative, the size of the rectangle is increased.
    ///
    /// If the resulting rectangle has a negative width or height, the size is
    /// set to 0.
    /// ## `d_x`
    /// an horizontal value; a positive `d_x` will create an inset rectangle,
    ///  and a negative value will create a larger rectangle
    /// ## `d_y`
    /// a vertical value; a positive `d_x` will create an inset rectangle,
    ///  and a negative value will create a larger rectangle
    pub fn inset(&mut self, d_x: f32, d_y: f32) {
        unsafe {
            ffi::clutter_rect_inset(self.to_glib_none_mut().0, d_x, d_y);
        }
    }

    /// Computes the intersection of `self` and `b`, and places it in `res`, if `res`
    /// is not `None`.
    ///
    /// This function will normalize both `self` and `b` prior to computing their
    /// intersection.
    ///
    /// This function can be used to simply check if the intersection of `self` and `b`
    /// is not empty, by using `None` for `res`.
    /// ## `b`
    /// a `Rect`
    /// ## `res`
    /// a `Rect`, or `None`
    ///
    /// # Returns
    ///
    /// `true` if the intersection of `self` and `b` is not empty
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

    /// Normalizes a `Rect`.
    ///
    /// A `Rect` is defined by the area covered by its size; this means
    /// that a `Rect` with `Rect.origin` in [ 0, 0 ] and a
    /// `Rect.size` of [ 10, 10 ] is equivalent to a `Rect` with
    /// `Rect.origin` in [ 10, 10 ] and a `Rect.size` of [ -10, -10 ].
    ///
    /// This function is useful to ensure that a rectangle has positive width
    /// and height; it will modify the passed `self` and normalize its size.
    pub fn normalize(&mut self) -> Option<Rect> {
        unsafe { from_glib_full(ffi::clutter_rect_normalize(self.to_glib_none_mut().0)) }
    }

    /// Offsets the origin of `self` by the given values, after normalizing
    /// the rectangle.
    /// ## `d_x`
    /// the horizontal offset value
    /// ## `d_y`
    /// the vertical offset value
    pub fn offset(&mut self, d_x: f32, d_y: f32) {
        unsafe {
            ffi::clutter_rect_offset(self.to_glib_none_mut().0, d_x, d_y);
        }
    }

    /// Computes the smallest possible rectangle capable of fully containing
    /// both `self` and `b`, and places it into `res`.
    ///
    /// This function will normalize both `self` and `b` prior to computing their
    /// union.
    /// ## `b`
    /// a `Rect`
    /// ## `res`
    /// a `Rect`
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

    /// A `Rect` with `Rect.origin` set at (0, 0) and a size
    /// of 0.
    ///
    /// The returned value can be used as a guard.
    ///
    /// # Returns
    ///
    /// a rectangle with origin in (0, 0) and a size of 0.
    ///  The returned `Rect` is owned by Clutter and it should not
    ///  be modified or freed.
    pub fn zero() -> Option<Rect> {
        unsafe { from_glib_none(ffi::clutter_rect_zero()) }
    }
}

#[doc(hidden)]
impl Uninitialized for Rect {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc()
    }
}
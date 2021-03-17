use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct ActorBox(Boxed<ffi::ClutterActorBox>);

    match fn {
        copy => |ptr| ffi::clutter_actor_box_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_actor_box_free(ptr),
        get_type => || ffi::clutter_actor_box_get_type(),
    }
}

impl ActorBox {
    pub fn new(x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> ActorBox {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::clutter_actor_box_new(x_1, y_1, x_2, y_2)) }
    }

    //pub fn clamp_to_pixel(&self) {
    //    unsafe { TODO: call clutter_sys:clutter_actor_box_clamp_to_pixel() }
    //}

    pub fn contains(&self, x: f32, y: f32) -> bool {
        unsafe {
            from_glib(ffi::clutter_actor_box_contains(
                self.to_glib_none().0,
                x,
                y,
            ))
        }
    }

    fn equal(&self, box_b: &ActorBox) -> bool {
        unsafe {
            from_glib(ffi::clutter_actor_box_equal(
                self.to_glib_none().0,
                box_b.to_glib_none().0,
            ))
        }
    }

    //pub fn from_vertices(&mut self, verts: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 16 }; 4) {
    //    unsafe { TODO: call clutter_sys:clutter_actor_box_from_vertices() }
    //}

    pub fn get_area(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_area(self.to_glib_none().0) }
    }

    pub fn get_height(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_height(self.to_glib_none().0) }
    }

    pub fn get_origin(&self) -> (f32, f32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::clutter_actor_box_get_origin(
                self.to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }

    pub fn get_size(&self) -> (f32, f32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::clutter_actor_box_get_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    pub fn get_width(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_width(self.to_glib_none().0) }
    }

    pub fn get_x(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_x(self.to_glib_none().0) }
    }

    pub fn get_y(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_y(self.to_glib_none().0) }
    }

    pub fn init(&mut self, x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> Option<ActorBox> {
        unsafe {
            from_glib_none(ffi::clutter_actor_box_init(
                self.to_glib_none_mut().0,
                x_1,
                y_1,
                x_2,
                y_2,
            ))
        }
    }

    pub fn init_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            ffi::clutter_actor_box_init_rect(
                self.to_glib_none_mut().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    pub fn interpolate(&self, final_: &ActorBox, progress: f64) -> ActorBox {
        unsafe {
            let mut result = ActorBox::uninitialized();
            ffi::clutter_actor_box_interpolate(
                self.to_glib_none().0,
                final_.to_glib_none().0,
                progress,
                result.to_glib_none_mut().0,
            );
            result
        }
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        unsafe {
            ffi::clutter_actor_box_set_origin(self.to_glib_none_mut().0, x, y);
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        unsafe {
            ffi::clutter_actor_box_set_size(self.to_glib_none_mut().0, width, height);
        }
    }

    pub fn union(&self, b: &ActorBox) -> ActorBox {
        unsafe {
            let mut result = ActorBox::uninitialized();
            ffi::clutter_actor_box_union(
                self.to_glib_none().0,
                b.to_glib_none().0,
                result.to_glib_none_mut().0,
            );
            result
        }
    }

    pub fn alloc() -> Option<ActorBox> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::clutter_actor_box_alloc()) }
    }
}

impl PartialEq for ActorBox {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for ActorBox {}

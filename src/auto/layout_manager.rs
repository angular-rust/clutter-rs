// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::Actor;
use super::ActorBox;
use super::AllocationFlags;
use super::Container;
use super::LayoutMeta;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct LayoutManager(Object<ffi::ClutterLayoutManager, ffi::ClutterLayoutManagerClass, LayoutManagerClass>);

    match fn {
        get_type => || ffi::clutter_layout_manager_get_type(),
    }
}

pub const NONE_LAYOUT_MANAGER: Option<&LayoutManager> = None;

pub trait LayoutManagerExt: 'static {
    fn allocate<P: IsA<Container>>(
        &self,
        container: &P,
        allocation: &ActorBox,
        flags: AllocationFlags,
    );

    //fn child_get<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn child_get_property<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, property_name: &str, value: /*Ignored*/&mut glib::Value);

    //fn child_set<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn child_set_property<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, property_name: &str, value: /*Ignored*/&glib::Value);

    //fn find_child_property(&self, name: &str) -> /*Ignored*/Option<glib::ParamSpec>;

    fn get_child_meta<P: IsA<Container>, Q: IsA<Actor>>(
        &self,
        container: &P,
        actor: &Q,
    ) -> Option<LayoutMeta>;

    fn get_preferred_height<P: IsA<Container>>(&self, container: &P, for_width: f32) -> (f32, f32);

    fn get_preferred_width<P: IsA<Container>>(&self, container: &P, for_height: f32) -> (f32, f32);

    fn layout_changed(&self);

    //fn list_child_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec>;

    fn set_container<P: IsA<Container>>(&self, container: Option<&P>);

    fn connect_layout_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LayoutManager>> LayoutManagerExt for O {
    fn allocate<P: IsA<Container>>(
        &self,
        container: &P,
        allocation: &ActorBox,
        flags: AllocationFlags,
    ) {
        unsafe {
            ffi::clutter_layout_manager_allocate(
                self.as_ref().to_glib_none().0,
                container.as_ref().to_glib_none().0,
                allocation.to_glib_none().0,
                flags.to_glib(),
            );
        }
    }

    //fn child_get<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_layout_manager_child_get() }
    //}

    //fn child_get_property<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, property_name: &str, value: /*Ignored*/&mut glib::Value) {
    //    unsafe { TODO: call clutter_sys:clutter_layout_manager_child_get_property() }
    //}

    //fn child_set<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_layout_manager_child_set() }
    //}

    //fn child_set_property<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, property_name: &str, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call clutter_sys:clutter_layout_manager_child_set_property() }
    //}

    //fn find_child_property(&self, name: &str) -> /*Ignored*/Option<glib::ParamSpec> {
    //    unsafe { TODO: call clutter_sys:clutter_layout_manager_find_child_property() }
    //}

    fn get_child_meta<P: IsA<Container>, Q: IsA<Actor>>(
        &self,
        container: &P,
        actor: &Q,
    ) -> Option<LayoutMeta> {
        unsafe {
            from_glib_none(ffi::clutter_layout_manager_get_child_meta(
                self.as_ref().to_glib_none().0,
                container.as_ref().to_glib_none().0,
                actor.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preferred_height<P: IsA<Container>>(&self, container: &P, for_width: f32) -> (f32, f32) {
        unsafe {
            let mut min_height_p = mem::MaybeUninit::uninit();
            let mut nat_height_p = mem::MaybeUninit::uninit();
            ffi::clutter_layout_manager_get_preferred_height(
                self.as_ref().to_glib_none().0,
                container.as_ref().to_glib_none().0,
                for_width,
                min_height_p.as_mut_ptr(),
                nat_height_p.as_mut_ptr(),
            );
            let min_height_p = min_height_p.assume_init();
            let nat_height_p = nat_height_p.assume_init();
            (min_height_p, nat_height_p)
        }
    }

    fn get_preferred_width<P: IsA<Container>>(&self, container: &P, for_height: f32) -> (f32, f32) {
        unsafe {
            let mut min_width_p = mem::MaybeUninit::uninit();
            let mut nat_width_p = mem::MaybeUninit::uninit();
            ffi::clutter_layout_manager_get_preferred_width(
                self.as_ref().to_glib_none().0,
                container.as_ref().to_glib_none().0,
                for_height,
                min_width_p.as_mut_ptr(),
                nat_width_p.as_mut_ptr(),
            );
            let min_width_p = min_width_p.assume_init();
            let nat_width_p = nat_width_p.assume_init();
            (min_width_p, nat_width_p)
        }
    }

    fn layout_changed(&self) {
        unsafe {
            ffi::clutter_layout_manager_layout_changed(self.as_ref().to_glib_none().0);
        }
    }

    //fn list_child_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec> {
    //    unsafe { TODO: call clutter_sys:clutter_layout_manager_list_child_properties() }
    //}

    fn set_container<P: IsA<Container>>(&self, container: Option<&P>) {
        unsafe {
            ffi::clutter_layout_manager_set_container(
                self.as_ref().to_glib_none().0,
                container.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_layout_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layout_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterLayoutManager,
            f: glib_sys::gpointer,
        ) where
            P: IsA<LayoutManager>,
        {
            let f: &F = &*(f as *const F);
            f(&LayoutManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"layout-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    layout_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for LayoutManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LayoutManager")
    }
}

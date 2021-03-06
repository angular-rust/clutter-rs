use crate::{Actor, ActorBox, AllocationFlags, Container, LayoutMeta};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

glib_wrapper! {
    pub struct LayoutManager(Object<ffi::ClutterLayoutManager, ffi::ClutterLayoutManagerClass, LayoutManagerClass>) @extends gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_layout_manager_get_type(),
    }
}

pub const NONE_LAYOUT_MANAGER: Option<&LayoutManager> = None;

/// Trait containing all `LayoutManager` methods.
///
/// # Implementors
///
/// [`BinLayout`](struct.BinLayout.html), [`BoxLayout`](struct.BoxLayout.html), [`FixedLayout`](struct.FixedLayout.html), [`FlowLayout`](struct.FlowLayout.html), [`GridLayout`](struct.GridLayout.html), [`LayoutManager`](struct.LayoutManager.html)
pub trait LayoutManagerExt: 'static {
    /// Allocates the children of `container` given an area
    ///
    /// See also `ActorExt::allocate`
    /// ## `container`
    /// the `Container` using `self`
    /// ## `allocation`
    /// the `ActorBox` containing the allocated area
    ///  of `container`
    /// ## `flags`
    /// the allocation flags
    fn allocate<P: IsA<Container>>(
        &self,
        container: &P,
        allocation: &ActorBox,
        flags: AllocationFlags,
    );

    //fn child_get<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Gets a property on the `LayoutMeta` created by `self` and
    /// attached to a child of `container`
    ///
    /// The `gobject::Value` must already be initialized to the type of the property
    /// and has to be unset with `gobject::Value::unset` after extracting the real
    /// value out of it
    /// ## `container`
    /// a `Container` using `self`
    /// ## `actor`
    /// a `Actor` child of `container`
    /// ## `property_name`
    /// the name of the property to get
    /// ## `value`
    /// a `gobject::Value` with the value of the property to get
    fn child_get_property<P: IsA<Container>, Q: IsA<Actor>>(
        &self,
        container: &P,
        actor: &Q,
        property_name: &str,
        value: &mut glib::Value,
    );

    //fn child_set<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets a property on the `LayoutMeta` created by `self` and
    /// attached to a child of `container`
    /// ## `container`
    /// a `Container` using `self`
    /// ## `actor`
    /// a `Actor` child of `container`
    /// ## `property_name`
    /// the name of the property to set
    /// ## `value`
    /// a `gobject::Value` with the value of the property to set
    fn child_set_property<P: IsA<Container>, Q: IsA<Actor>>(
        &self,
        container: &P,
        actor: &Q,
        property_name: &str,
        value: &glib::Value,
    );

    /// Retrieves the `gobject::ParamSpec` for the layout property `name` inside
    /// the `LayoutMeta` sub-class used by `self`
    /// ## `name`
    /// the name of the property
    ///
    /// # Returns
    ///
    /// a `gobject::ParamSpec` describing the property,
    ///  or `None` if no property with that name exists. The returned
    ///  `gobject::ParamSpec` is owned by the layout manager and should not be
    ///  modified or freed
    fn find_child_property(&self, name: &str) -> Option<glib::ParamSpec>;

    /// Retrieves the `LayoutMeta` that the layout `self` associated
    /// to the `actor` child of `container`, eventually by creating one if the
    /// `LayoutManager` supports layout properties
    /// ## `container`
    /// a `Container` using `self`
    /// ## `actor`
    /// a `Actor` child of `container`
    ///
    /// # Returns
    ///
    /// a `LayoutMeta`, or `None` if the
    ///  `LayoutManager` does not have layout properties. The returned
    ///  layout meta instance is owned by the `LayoutManager` and it
    ///  should not be unreferenced
    fn get_child_meta<P: IsA<Container>, Q: IsA<Actor>>(
        &self,
        container: &P,
        actor: &Q,
    ) -> Option<LayoutMeta>;

    /// Computes the minimum and natural heights of the `container` according
    /// to `self`.
    ///
    /// See also `ActorExt::get_preferred_height`
    /// ## `container`
    /// the `Container` using `self`
    /// ## `for_width`
    /// the width for which the height should be computed, or -1
    /// ## `min_height_p`
    /// return location for the minimum height
    ///  of the layout, or `None`
    /// ## `nat_height_p`
    /// return location for the natural height
    ///  of the layout, or `None`
    fn get_preferred_height<P: IsA<Container>>(&self, container: &P, for_width: f32) -> (f32, f32);

    /// Computes the minimum and natural widths of the `container` according
    /// to `self`.
    ///
    /// See also `ActorExt::get_preferred_width`
    /// ## `container`
    /// the `Container` using `self`
    /// ## `for_height`
    /// the height for which the width should be computed, or -1
    /// ## `min_width_p`
    /// return location for the minimum width
    ///  of the layout, or `None`
    /// ## `nat_width_p`
    /// return location for the natural width
    ///  of the layout, or `None`
    fn get_preferred_width<P: IsA<Container>>(&self, container: &P, for_height: f32) -> (f32, f32);

    /// Emits the `LayoutManager::layout-changed` signal on `self`
    ///
    /// This function should only be called by implementations of the
    /// `LayoutManager` class
    fn layout_changed(&self);

    /// Retrieves all the `gobject::ParamSpec`<!-- -->s for the layout properties
    /// stored inside the `LayoutMeta` sub-class used by `self`
    /// ## `n_pspecs`
    /// return location for the number of returned
    ///  `gobject::ParamSpec`<!-- -->s
    ///
    /// # Returns
    ///
    /// the newly-allocated,
    ///  `None`-terminated array of `gobject::ParamSpec`<!-- -->s. Use `g_free` to free the
    ///  resources allocated for the array
    fn list_child_properties(&self) -> Vec<glib::ParamSpec>;

    /// If the `LayoutManager` sub-class allows it, allow
    /// adding a weak reference of the `container` using `self`
    /// from within the layout manager
    ///
    /// The layout manager should not increase the reference
    /// count of the `container`
    /// ## `container`
    /// a `Container` using `self`
    fn set_container<P: IsA<Container>>(&self, container: Option<&P>);

    /// The ::layout-changed signal is emitted each time a layout manager
    /// has been changed. Every `Actor` using the `manager` instance
    /// as a layout manager should connect a handler to the ::layout-changed
    /// signal and queue a relayout on themselves:
    ///
    ///
    /// ```text
    ///   static void layout_changed (ClutterLayoutManager *manager,
    ///                               ClutterActor         *self)
    ///   {
    ///     clutter_actor_queue_relayout (self);
    ///   }
    ///   ...
    ///     self->manager = g_object_ref_sink (manager);
    ///     g_signal_connect (self->manager, "layout-changed",
    ///                       G_CALLBACK (layout_changed),
    ///                       self);
    /// ```
    ///
    /// Sub-classes of `LayoutManager` that implement a layout that
    /// can be controlled or changed using parameters should emit the
    /// ::layout-changed signal whenever one of the parameters changes,
    /// by using `LayoutManagerExt::layout_changed`.
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

    fn child_get_property<P: IsA<Container>, Q: IsA<Actor>>(
        &self,
        container: &P,
        actor: &Q,
        property_name: &str,
        value: &mut glib::Value,
    ) {
        unsafe {
            ffi::clutter_layout_manager_child_get_property(
                self.as_ref().to_glib_none().0,
                container.as_ref().to_glib_none().0,
                actor.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
    }

    //fn child_set<P: IsA<Container>, Q: IsA<Actor>>(&self, container: &P, actor: &Q, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_layout_manager_child_set() }
    //}

    fn child_set_property<P: IsA<Container>, Q: IsA<Actor>>(
        &self,
        container: &P,
        actor: &Q,
        property_name: &str,
        value: &glib::Value,
    ) {
        unsafe {
            ffi::clutter_layout_manager_child_set_property(
                self.as_ref().to_glib_none().0,
                container.as_ref().to_glib_none().0,
                actor.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn find_child_property(&self, name: &str) -> Option<glib::ParamSpec> {
        unsafe {
            from_glib_none(ffi::clutter_layout_manager_find_child_property(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

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

    fn list_child_properties(&self) -> Vec<glib::ParamSpec> {
        unsafe {
            let mut n_pspecs = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::clutter_layout_manager_list_child_properties(
                    self.as_ref().to_glib_none().0,
                    n_pspecs.as_mut_ptr(),
                ),
                n_pspecs.assume_init() as usize,
            );
            ret
        }
    }

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

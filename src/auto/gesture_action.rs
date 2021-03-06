use crate::{Action, Actor, ActorMeta, Event, EventSequence, GestureTriggerEdge, InputDevice};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

glib_wrapper! {
    pub struct GestureAction(Object<ffi::ClutterGestureAction, ffi::ClutterGestureActionClass, GestureActionClass>) @extends Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_gesture_action_get_type(),
    }
}

impl GestureAction {
    /// Creates a new `GestureAction` instance.
    ///
    /// # Returns
    ///
    /// the newly created `GestureAction`
    pub fn new() -> GestureAction {
        unsafe { Action::from_glib_none(ffi::clutter_gesture_action_new()).unsafe_cast() }
    }
}

impl Default for GestureAction {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GESTURE_ACTION: Option<&GestureAction> = None;

/// Trait containing all `GestureAction` methods.
///
/// # Implementors
///
/// [`GestureAction`](struct.GestureAction.html), [`PanAction`](struct.PanAction.html), [`RotateAction`](struct.RotateAction.html), [`SwipeAction`](struct.SwipeAction.html), [`TapAction`](struct.TapAction.html), [`ZoomAction`](struct.ZoomAction.html)
pub trait GestureActionExt: 'static {
    /// Cancel a `GestureAction` before it begins
    fn cancel(&self);

    /// Retrieves the `InputDevice` of a touch point.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    ///
    /// # Returns
    ///
    /// the `InputDevice` of a touch point.
    fn get_device(&self, point: u32) -> Option<InputDevice>;

    /// Retrieves a reference to the last `ClutterEvent` for a touch point. Call
    /// `clutter_event_copy` if you need to store the reference somewhere.
    /// ## `point`
    /// index of a point currently active
    ///
    /// # Returns
    ///
    /// the last `ClutterEvent` for a touch point.
    fn get_last_event(&self, point: u32) -> Option<Event>;

    /// Retrieves the coordinates, in stage space, of the latest motion
    /// event during the dragging.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `motion_x`
    /// return location for the latest motion
    ///  event's X coordinate
    /// ## `motion_y`
    /// return location for the latest motion
    ///  event's Y coordinate
    fn get_motion_coords(&self, point: u32) -> (f32, f32);

    /// Retrieves the incremental delta since the last motion event
    /// during the dragging.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `delta_x`
    /// return location for the X axis
    ///  component of the incremental motion delta
    /// ## `delta_y`
    /// return location for the Y axis
    ///  component of the incremental motion delta
    ///
    /// # Returns
    ///
    /// the distance since last motion event
    fn get_motion_delta(&self, point: u32) -> (f32, f32, f32);

    /// Retrieves the number of points currently active.
    ///
    /// # Returns
    ///
    /// the number of points currently active.
    fn get_n_current_points(&self) -> u32;

    /// Retrieves the number of requested points to trigger the gesture.
    ///
    /// # Returns
    ///
    /// the number of points to trigger the gesture.
    fn get_n_touch_points(&self) -> i32;

    /// Retrieves the coordinates, in stage space, of the press event
    /// that started the dragging for a specific touch point.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `press_x`
    /// return location for the press
    ///  event's X coordinate
    /// ## `press_y`
    /// return location for the press
    ///  event's Y coordinate
    fn get_press_coords(&self, point: u32) -> (f32, f32);

    /// Retrieves the coordinates, in stage space, where the touch point was
    /// last released.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `release_x`
    /// return location for the X coordinate of
    ///  the last release
    /// ## `release_y`
    /// return location for the Y coordinate of
    ///  the last release
    fn get_release_coords(&self, point: u32) -> (f32, f32);

    /// Retrieves the `EventSequence` of a touch point.
    /// ## `point`
    /// index of a point currently active
    ///
    /// # Returns
    ///
    /// the `EventSequence` of a touch point.
    fn get_sequence(&self, point: u32) -> Option<EventSequence>;

    /// Retrieves the threshold trigger distance of the gesture `self`,
    /// as set using `GestureActionExt::set_threshold_trigger_distance`.
    /// ## `x`
    /// The return location for the horizontal distance, or `None`
    /// ## `y`
    /// The return location for the vertical distance, or `None`
    fn get_threshold_trigger_distance(&self) -> (f32, f32);

    /// Retrieves the edge trigger of the gesture `self`, as set using
    /// `GestureActionExt::set_threshold_trigger_edge`.
    ///
    /// # Returns
    ///
    /// the edge trigger
    fn get_threshold_trigger_edge(&self) -> GestureTriggerEdge;

    /// Retrieves the velocity, in stage pixels per millisecond, of the
    /// latest motion event during the dragging.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `velocity_x`
    /// return location for the latest motion
    ///  event's X velocity
    /// ## `velocity_y`
    /// return location for the latest motion
    ///  event's Y velocity
    fn get_velocity(&self, point: u32) -> (f32, f32, f32);

    /// Sets the number of points needed to trigger the gesture.
    /// ## `nb_points`
    /// a number of points
    fn set_n_touch_points(&self, nb_points: i32);

    /// Sets the threshold trigger distance for the gesture drag threshold, if any.
    ///
    /// This function should only be called by sub-classes of
    /// `GestureAction` during their construction phase.
    /// ## `x`
    /// the distance on the horizontal axis
    /// ## `y`
    /// the distance on the vertical axis
    fn set_threshold_trigger_distance(&self, x: f32, y: f32);

    /// Sets the edge trigger for the gesture drag threshold, if any.
    ///
    /// This function should only be called by sub-classes of
    /// `GestureAction` during their construction phase.
    /// ## `edge`
    /// the `GestureTriggerEdge`
    fn set_threshold_trigger_edge(&self, edge: GestureTriggerEdge);

    /// The horizontal trigger distance to be used by the action to either
    /// emit the `GestureAction::gesture-begin` signal or to emit
    /// the `GestureAction::gesture-cancel` signal.
    ///
    /// A negative value will be interpreted as the default drag threshold.
    fn get_property_threshold_trigger_distance_x(&self) -> f32;

    /// The vertical trigger distance to be used by the action to either
    /// emit the `GestureAction::gesture-begin` signal or to emit
    /// the `GestureAction::gesture-cancel` signal.
    ///
    /// A negative value will be interpreted as the default drag threshold.
    fn get_property_threshold_trigger_distance_y(&self) -> f32;

    /// The ::gesture_begin signal is emitted when the `Actor` to which
    /// a `GestureAction` has been applied starts receiving a gesture.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    ///
    /// # Returns
    ///
    /// `true` if the gesture should start, and `false` if
    ///  the gesture should be ignored.
    fn connect_gesture_begin<F: Fn(&Self, &Actor) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::gesture-cancel signal is emitted when the ongoing gesture gets
    /// cancelled from the `GestureAction::gesture-progress` signal handler.
    ///
    /// This signal is emitted if and only if the `GestureAction::gesture-begin`
    /// signal has been emitted first.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_gesture_cancel<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::gesture-end signal is emitted at the end of the gesture gesture,
    /// when the pointer's button is released
    ///
    /// This signal is emitted if and only if the `GestureAction::gesture-begin`
    /// signal has been emitted first.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_gesture_end<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::gesture-progress signal is emitted for each motion event after
    /// the `GestureAction::gesture-begin` signal has been emitted.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    ///
    /// # Returns
    ///
    /// `true` if the gesture should continue, and `false` if
    ///  the gesture should be cancelled.
    fn connect_gesture_progress<F: Fn(&Self, &Actor) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_n_touch_points_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<GestureAction>> GestureActionExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::clutter_gesture_action_cancel(self.as_ref().to_glib_none().0);
        }
    }

    fn get_device(&self, point: u32) -> Option<InputDevice> {
        unsafe {
            from_glib_none(ffi::clutter_gesture_action_get_device(
                self.as_ref().to_glib_none().0,
                point,
            ))
        }
    }

    fn get_last_event(&self, point: u32) -> Option<Event> {
        unsafe {
            from_glib_none(ffi::clutter_gesture_action_get_last_event(
                self.as_ref().to_glib_none().0,
                point,
            ))
        }
    }

    fn get_motion_coords(&self, point: u32) -> (f32, f32) {
        unsafe {
            let mut motion_x = mem::MaybeUninit::uninit();
            let mut motion_y = mem::MaybeUninit::uninit();
            ffi::clutter_gesture_action_get_motion_coords(
                self.as_ref().to_glib_none().0,
                point,
                motion_x.as_mut_ptr(),
                motion_y.as_mut_ptr(),
            );
            let motion_x = motion_x.assume_init();
            let motion_y = motion_y.assume_init();
            (motion_x, motion_y)
        }
    }

    fn get_motion_delta(&self, point: u32) -> (f32, f32, f32) {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            let ret = ffi::clutter_gesture_action_get_motion_delta(
                self.as_ref().to_glib_none().0,
                point,
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            );
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            (ret, delta_x, delta_y)
        }
    }

    fn get_n_current_points(&self) -> u32 {
        unsafe { ffi::clutter_gesture_action_get_n_current_points(self.as_ref().to_glib_none().0) }
    }

    fn get_n_touch_points(&self) -> i32 {
        unsafe { ffi::clutter_gesture_action_get_n_touch_points(self.as_ref().to_glib_none().0) }
    }

    fn get_press_coords(&self, point: u32) -> (f32, f32) {
        unsafe {
            let mut press_x = mem::MaybeUninit::uninit();
            let mut press_y = mem::MaybeUninit::uninit();
            ffi::clutter_gesture_action_get_press_coords(
                self.as_ref().to_glib_none().0,
                point,
                press_x.as_mut_ptr(),
                press_y.as_mut_ptr(),
            );
            let press_x = press_x.assume_init();
            let press_y = press_y.assume_init();
            (press_x, press_y)
        }
    }

    fn get_release_coords(&self, point: u32) -> (f32, f32) {
        unsafe {
            let mut release_x = mem::MaybeUninit::uninit();
            let mut release_y = mem::MaybeUninit::uninit();
            ffi::clutter_gesture_action_get_release_coords(
                self.as_ref().to_glib_none().0,
                point,
                release_x.as_mut_ptr(),
                release_y.as_mut_ptr(),
            );
            let release_x = release_x.assume_init();
            let release_y = release_y.assume_init();
            (release_x, release_y)
        }
    }

    fn get_sequence(&self, point: u32) -> Option<EventSequence> {
        unsafe {
            from_glib_none(ffi::clutter_gesture_action_get_sequence(
                self.as_ref().to_glib_none().0,
                point,
            ))
        }
    }

    fn get_threshold_trigger_distance(&self) -> (f32, f32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::clutter_gesture_action_get_threshold_trigger_distance(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }

    fn get_threshold_trigger_edge(&self) -> GestureTriggerEdge {
        unsafe {
            from_glib(ffi::clutter_gesture_action_get_threshold_trigger_edge(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_velocity(&self, point: u32) -> (f32, f32, f32) {
        unsafe {
            let mut velocity_x = mem::MaybeUninit::uninit();
            let mut velocity_y = mem::MaybeUninit::uninit();
            let ret = ffi::clutter_gesture_action_get_velocity(
                self.as_ref().to_glib_none().0,
                point,
                velocity_x.as_mut_ptr(),
                velocity_y.as_mut_ptr(),
            );
            let velocity_x = velocity_x.assume_init();
            let velocity_y = velocity_y.assume_init();
            (ret, velocity_x, velocity_y)
        }
    }

    fn set_n_touch_points(&self, nb_points: i32) {
        unsafe {
            ffi::clutter_gesture_action_set_n_touch_points(
                self.as_ref().to_glib_none().0,
                nb_points,
            );
        }
    }

    fn set_threshold_trigger_distance(&self, x: f32, y: f32) {
        unsafe {
            ffi::clutter_gesture_action_set_threshold_trigger_distance(
                self.as_ref().to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn set_threshold_trigger_edge(&self, edge: GestureTriggerEdge) {
        unsafe {
            ffi::clutter_gesture_action_set_threshold_trigger_edge(
                self.as_ref().to_glib_none().0,
                edge.to_glib(),
            );
        }
    }

    fn get_property_threshold_trigger_distance_x(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"threshold-trigger-distance-x\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `threshold-trigger-distance-x` getter")
                .unwrap()
        }
    }

    fn get_property_threshold_trigger_distance_y(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"threshold-trigger-distance-y\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `threshold-trigger-distance-y` getter")
                .unwrap()
        }
    }

    fn connect_gesture_begin<F: Fn(&Self, &Actor) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn gesture_begin_trampoline<P, F: Fn(&P, &Actor) -> bool + 'static>(
            this: *mut ffi::ClutterGestureAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<GestureAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"gesture-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    gesture_begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_gesture_cancel<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn gesture_cancel_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterGestureAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"gesture-cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    gesture_cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_gesture_end<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn gesture_end_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterGestureAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"gesture-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    gesture_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_gesture_progress<F: Fn(&Self, &Actor) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn gesture_progress_trampoline<P, F: Fn(&P, &Actor) -> bool + 'static>(
            this: *mut ffi::ClutterGestureAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<GestureAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"gesture-progress\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    gesture_progress_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_n_touch_points_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_touch_points_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterGestureAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureAction>,
        {
            let f: &F = &*(f as *const F);
            f(&GestureAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-touch-points\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_touch_points_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GestureAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureAction")
    }
}

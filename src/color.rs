use super::Color;

impl Color {
    pub fn uninitialized() -> Color {
        unimplemented!() // TODO

        // assert_initialized_main_thread!();
        // unsafe {
        //     Transition::from_glib_full(ffi::clutter_transition_group_new()).unsafe_cast()
        // }
    }
}

// impl Default for Color {
//     fn default() -> Self {
//         unimplemented!() // TODO

//         // Self::new()
//     }
// }

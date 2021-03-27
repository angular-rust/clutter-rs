// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::LayoutManager;
use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TableLayout(Object<ffi::ClutterTableLayout, ffi::ClutterTableLayoutClass, TableLayoutClass>) @extends LayoutManager;

    match fn {
        get_type => || ffi::clutter_table_layout_get_type(),
    }
}

impl TableLayout {}

pub const NONE_TABLE_LAYOUT: Option<&TableLayout> = None;

impl fmt::Display for TableLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TableLayout")
    }
}

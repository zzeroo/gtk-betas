// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate gdk;
extern crate glib;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate gtk;
extern crate xmz_server;

pub mod errors;
pub mod gui;
#[macro_export] pub mod macros;

use errors::*;

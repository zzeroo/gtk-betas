#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://zzeroo.github.io/gtk-betas")]
//! xMZ-Mod-Touch-GUI
//!
//! Grafische Oberfläche der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/zzeroo/gtk-betas.git

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate gdk;
extern crate glib;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate gtk;
extern crate rand;
extern crate xmz_server;

pub mod errors;
pub mod gui;
pub mod server;

use errors::*;

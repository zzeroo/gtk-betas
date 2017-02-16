// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;

pub mod errors;
pub mod gui;

use errors::*;

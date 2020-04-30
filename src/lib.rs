#![cfg_attr(not(feature = "std"), no_std)]
#![crate_type = "rlib"]
#![cfg_attr(feature = "std", crate_type = "staticlib")]

mod random;

pub mod ffi;
pub mod logic;
pub mod ui;

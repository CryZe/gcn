#![no_std]

extern crate arrayvec;
#[cfg(feature = "serialize")]
extern crate bincode;
#[cfg(feature = "serialize")]
extern crate serde;

pub mod card;
pub mod gx;
pub mod os;
pub mod print;
pub mod time;

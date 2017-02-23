#![feature(core_intrinsics,asm,lang_items)]

// We won't use the standard library because it requires OS abstractions like
// threads and files and those are not available on this platform.
#![no_std]

#[macro_use]
pub mod prelude;

#[macro_use]
pub mod f437;

#[macro_use]
pub mod m4;

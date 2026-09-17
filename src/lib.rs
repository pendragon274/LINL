#![feature(ptr_metadata)]
#![feature(once_cell_get_mut)]
#![allow(static_mut_refs)]
#![no_std]
#![no_main]
#![macro_use]

mod kernel_main;
mod panic;
mod vga_display;
mod memory;
mod asm_ops;
mod collections;
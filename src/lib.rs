#![feature(ptr_metadata)]
#![feature(once_cell_get_mut)]
#![feature(associated_type_defaults)]
#![feature(layout_for_ptr)]
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
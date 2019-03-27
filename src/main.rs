#![no_std]
#![no_main]
#![feature(alloc)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use panic_ramdump as _;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!()
}

#[global_allocator]
static A: MyAllocator = MyAllocator;

#[no_mangle]
fn main() {
    
}

#[doc(hidden)]
#[no_mangle]
#[link_section = ".iram0.vectors"]
pub unsafe extern "C" fn Reset() -> ! {
    // CURRENTLY DOES NOTHING
    loop {
        
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // CURRENTLY DOES NOTHING
    loop {
        
    }
}




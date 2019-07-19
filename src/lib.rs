#![feature(lang_items)]
#![no_std]
#[cfg(debug_assertions)]
extern crate panic_semihosting;
#[cfg(not(debug_assertions))]
extern crate panic_abort;

extern crate typenum;

#[lang = "eh_personality"] extern fn eh_personality() {}

pub mod cstd_alloc;
pub mod stacked_alloc;

#[no_mangle]
pub unsafe extern "C" fn rust_main(n: usize) {
    use cstd_alloc::RUST_CSTD_ALLOCATOR;
    let test = RUST_CSTD_ALLOCATOR.array::<usize>(n);
    RUST_CSTD_ALLOCATOR.free(test);
}
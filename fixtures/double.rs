#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub fn double(x: i32) -> i32 {
    x * 2
}

// Declare a panic handler so that `no_std` will actually allow us to compile
// without a runtime.
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32
) -> ! {
    loop {}
}

#![no_std]
#![feature(lang_items, asm, phase)]

extern crate core;
extern crate rlibc;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[cfg(not(test))]
#[lang="begin_unwind"]
extern fn begin_unwind() {}

mod device;
mod devices {
    mod mma7361;
}

#[no_mangle]
#[start]
pub fn main(argc: int, argv: *const *const u8) -> int {
    run();
    loop {};
    return 0;
}

fn run() {
    unsafe { asm!("nop") }
}

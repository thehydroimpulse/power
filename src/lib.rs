#![no_std]
#![feature(lang_items, asm, phase)]

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[cfg(not(test))]
#[lang="begin_unwind"]
extern fn begin_unwind() {}

#[lang="sized"]
pub trait Sized for Sized? {}

mod device;
mod devices {
    mod mma7361;
}

#[no_mangle]
#[start]
pub unsafe extern fn main() {
    run();
    loop {};    
}

fn run() {
    unsafe { asm!("nop") }
}

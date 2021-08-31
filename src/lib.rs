#![no_std]
pub mod sys;
use core::panic::PanicInfo;

#[panic_handler]
fn _panic(_: &PanicInfo) -> ! {
    loop {}

}


pub fn boot() -> usize {
    42
}
#![no_std]
#![no_main]

use bootloader::{BootInfo, entry_point};

entry_point!(kernal_main);

pub fn kernal_main(_boot_info: &'static BootInfo) -> ! {
    let mut terminal = jerry::sys::vga::TerminalWriter::new();
    terminal.clear_screen();
    terminal.write_string("Hello, World");
    
    loop {}
}
#![no_std]
#![no_main]
#![feature(asm, global_asm, naked_functions, panic_info_message)]

mod hw;
mod init;
mod panic;
mod uart;

#[no_mangle]
extern "C" fn kmain() {
    println!("Hello RISC-V from Rust!");
}

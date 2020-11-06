use crate::{print, println};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Kernel panic - aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

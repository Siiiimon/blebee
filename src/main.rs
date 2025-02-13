#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blebee::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blebee::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blebee::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blebee::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blebee::init();

    #[cfg(test)]
    test_main();

    println!("no crash yippie!");

    blebee::hlt_loop();
}

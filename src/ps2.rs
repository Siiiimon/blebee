pub mod scancode;

use crate::{interrupts, print, println};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    match scancode::interpret(scancode) {
        Ok(interp) => {
            if interp.0 {
                print!("{}", interp.1);
            }
        }
        Err(masked_scancode) => println!("failed to interpet {}", masked_scancode),
    }

    unsafe {
        interrupts::PICS
            .lock()
            .notify_end_of_interrupt(interrupts::InterruptIndex::Keyboard.as_u8());
    }
}

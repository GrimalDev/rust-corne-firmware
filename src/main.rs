#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::panic::PanicInfo;

const GPIO_ENABLE_W1TS: *mut u32 = 0x60000310 as *mut u32;
const GPIO_OUT_W1TS: *mut u32 = 0x60000304 as *mut u32;
const GPIO_OUT_W1TC: *mut u32 = 0x60000308 as *mut u32;

#[no_mangle]
pub extern "C" fn start() -> ! {
    // Set GPIO 2 as output
    unsafe {
        // Enable GPIO 2
        *GPIO_ENABLE_W1TS = 1 << 5;
        *GPIO_OUT_W1TC = 1 << 2;
        *GPIO_OUT_W1TS = 0 << 2;
    }

    loop {
        // Set GPIO 2 high
        unsafe {
            *GPIO_OUT_W1TS = 1 << 5;
        }
        delay(500);
        // Set GPIO 2 low
        unsafe {
            *GPIO_OUT_W1TS = 0 << 5;
        }
        delay(500);    }
}

fn delay(ms: u32) {
    // Busy wait delay for Xtensa LX106
    let cycles = ms * 80_000; // 80 MHz clock, 1 ms = 80,000 cycles
    let mut cycle_count: u32;
    unsafe {
        core::arch::asm!("rsr {0}, CCOUNT", out(reg) cycle_count);
    }
    let end = cycle_count + cycles;
    while {
        unsafe {
            core::arch::asm!("rsr {0}, CCOUNT", out(reg) cycle_count);
        }
        cycle_count < end
    } {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#![no_std]
#![feature(asm)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_abort;
extern crate volatile_register;

// extern crate NUCLEO_F401RE_led_blink;
mod peripheral;

use cortex_m::asm;
use peripheral::{gpio, rcc};

pub fn delay(ticks: u32) {
    for _ in 1..ticks {
        unsafe { asm!("") }
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}

fn main() {
    let rcc = rcc::RCC_BASE as *const rcc::RegisterMap;
    let gpioa = gpio::GPIOA_BASE as *const gpio::RegisterMap;
    const PIN_A5: u32 = 1 << 5;

    unsafe {
        (*rcc)
            .ahb1enr
            .modify(|v| v | rcc::ahb1enr::Gpioaen::Enable as u32);
        (*gpioa).moder.modify(|v| {
            (v & !((0b11 as u32) << (5 * 2))) | ((gpio::moder::Modery::Output as u32) << (5 * 2))
        });
        (*gpioa)
            .otyper
            .modify(|v| (v & !((0b1 as u32) << 5)) | ((gpio::otyper::Oty::PushPull as u32) << 5));
        (*gpioa).ospeedr.modify(|v| {
            (v & !((0b11 as u32) << (5 * 2))) | ((gpio::ospeedr::Ospeedr::High as u32) << (5 * 2))
        });
        (*gpioa).pupdr.modify(|v| {
            (v & !((0b11 as u32) << (5 * 2))) | (gpio::pupdr::Pupdr::NoPuPd as u32) << (5 * 2)
        });
    }

    loop {
        unsafe {
            (*gpioa).odr.modify(|v| v ^ PIN_A5);
        }
        delay(1000);
    }
}

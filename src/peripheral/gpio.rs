#![allow(dead_code)]
use volatile_register::{RO, RW, WO};

pub const GPIOA_BASE: u32 = 0x4002_0000;
pub const GPIOB_BASE: u32 = 0x4002_0400;
pub const GPIOC_BASE: u32 = 0x4002_0800;
pub const GPIOD_BASE: u32 = 0x4002_0C00;
pub const GPIOE_BASE: u32 = 0x4002_1000;
pub const GPIOH_BASE: u32 = 0x4002_1C00;

#[repr(C)]
pub struct RegisterMap {
    pub moder: RW<u32>,
    pub otyper: RW<u32>,
    pub ospeedr: RW<u32>,
    pub pupdr: RW<u32>,
    pub idr: RO<u32>,
    pub odr: RW<u32>,
    pub bsrr: WO<u32>,
    pub lckr: RW<u32>,
    pub afr: [RW<u32>; 2],
}

pub mod moder {
    /// Port x configuration bits
    #[derive(Clone, Copy, Debug)]
    pub enum Modery {
        Input = 0b00,
        Output = 0b01,
        Alternate = 0b10,
        Analog = 0b11,
    }
}

pub mod otyper {
    /// Port x configuration bits
    #[derive(Clone, Copy, Debug)]
    pub enum Oty {
        PushPull = 0b0,
        OpenDrain = 0b1,
    }
}

pub mod ospeedr {
    /// Port x configuration bits
    #[derive(Clone, Copy, Debug)]
    pub enum Ospeedr {
        Low = 0b00,
        Medium = 0b01,
        Fast = 0b10,
        High = 0b11,
    }
}

pub mod pupdr {
    /// Port x configuration bits
    #[derive(Clone, Copy, Debug)]
    pub enum Pupdr {
        NoPuPd = 0b00,
        PullUp = 0b01,
        PuuDown = 0b10,
    }
}

pub mod afr {
    /// Alternate function selection for port x bit (0..7)
    #[derive(Clone, Copy, Debug)]
    pub enum Afry {
        AF0 = 0b0000,
        AF1 = 0b0001,
        AF2 = 0b0010,
        AF3 = 0b0011,
        AF4 = 0b0100,
        AF5 = 0b0101,
        AF6 = 0b0110,
        AF7 = 0b0111,
        AF8 = 0b1000,
        AF9 = 0b1001,
        AF10 = 0b1010,
        AF11 = 0b1011,
        AF12 = 0b1100,
        AF13 = 0b1101,
        AF14 = 0b1110,
        AF15 = 0b1111,
    }
}

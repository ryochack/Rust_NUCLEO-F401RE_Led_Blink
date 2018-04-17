#![allow(dead_code)]
use volatile_register::RW;

pub const RCC_BASE: u32 = 0x4002_3800;

#[repr(C)]
pub struct RegisterMap {
    pub cr: RW<u32>,
    pub pllcfgr: RW<u32>,
    pub cfgr: RW<u32>,
    pub cir: RW<u32>,
    pub ahb1rstr: RW<u32>,
    pub ahb2rstr: RW<u32>,
    reserved0: [u32; 2],
    pub apb1rstr: RW<u32>,
    pub apb2rstr: RW<u32>,
    reserved1: [u32; 2],
    pub ahb1enr: RW<u32>,
    pub ahb2enr: RW<u32>,
    reserved2: [u32; 2],
    pub apb1enr: RW<u32>,
    pub apb2enr: RW<u32>,
    reserved3: [u32; 2],
    pub ahb1lpenr: RW<u32>,
    pub ahb2lpenr: RW<u32>,
    reserved4: [u32; 2],
    pub apb1lpenr: RW<u32>,
    pub apb2lpenr: RW<u32>,
    reserved5: [u32; 2],
    pub bdcr: RW<u32>,
    pub csr: RW<u32>,
    reserved6: [u32; 2],
    pub sscgr: RW<u32>,
    pub plli2scfgr: RW<u32>,
    reserved7: u32,
    pub dckcfgr: RW<u32>,
}

pub mod cr {
    /// PLLI2S clock ready flag
    pub enum Plli2srdy {
        Unlocked = 0b0 << 27,
        Locked = 0b1 << 27,
    }
    /// PLLI2S enable
    pub enum Plli2son {
        Off = 0b0 << 26,
        On = 0b1 << 26,
    }
    /// Main PLL clock ready flag
    pub enum Pllrdy {
        Unlocked = 0b0 << 25,
        Locked = 0b1 << 25,
    }
    /// Main PLL enable
    pub const PLLON_MASK: u32 = 0x1 << 24;
    pub enum Pllon {
        Off = 0b0 << 24,
        On = 0b1 << 24,
    }
    /// Clock security system enable
    pub enum Csson {
        Off = 0b0 << 19,
        On = 0b1 << 19,
    }
    /// HSE clock bypass
    pub enum Hsebyp {
        NotBypassed = 0b0 << 18,
        BypassedWithExternalClock = 0b1 << 18,
    }
    /// HSE clock ready flag
    pub enum Hserdy {
        NotReady = 0b0 << 17,
        Ready = 0b1 << 17,
    }
    /// HSE clock enable
    pub enum Hseon {
        Off = 0b0 << 16,
        On = 0b1 << 16,
    }
    /// Internal high-speed clock calibration
    pub const HSICAL_MASK: u32 = 0xFF << 8;
    /// Internal high-speed clock trimming
    pub const HSITRIM_SHIFT: u32 = 3;
    pub const HSITRIM_MASK: u32 = 0x1F << 3;
    /// Internal high-speed clock ready flag
    pub enum Hsirdy {
        NotReady = 0b0 << 1,
        Ready = 0b1 << 1,
    }
    /// Internal high-speed clock enable
    pub enum Hsion {
        Off = 0b0 << 0,
        On = 0b1 << 0,
    }
}

pub mod pllcfgr {
    /// Main PLL division factor for USB OTG FS,
    /// SDIO and random number generator clocks
    pub const PLLQ_MASK: u32 = 0x0F00_0000;
    /// Main PLL and audio PLL(PLLI2S) entry clock source
    pub const PLLSRC_MASK: u32 = 0b1 << 22;
    pub enum Pllsrc {
        HsiClock = 0b0 << 22,
        HseOscillatorClock = 0b1 << 22,
    }
    /// Main PLL division factor for main system clock
    pub enum Pllp {
        _2 = 0b00 << 16,
        _4 = 0b01 << 16,
        _6 = 0b10 << 16,
        _8 = 0b11 << 16,
    }
    /// Main PLL multiplication factor for VCO
    pub const PLLN_MASK: u32 = 0x7FC0;
    pub const PLLN_SHIFT: u32 = 6;
    /// Division factor for the main PLL and audio PLL(PLLI2S) input clock
    pub const PLLM_MASK: u32 = 0x3F;
}

pub mod cfgr {
    /// Microcontroller clock output 2
    pub enum Moc2 {
        SystemClock = 0b00 << 30,
        Plli2sClock = 0b01 << 30,
        HseOscillatorClock = 0b10 << 30,
        PllClock = 0b11 << 30,
    }
    /// MCO2 prescaler
    pub enum Moc2pre {
        Div2 = 0b100 << 27,
        Div3 = 0b101 << 27,
        Div4 = 0b110 << 27,
        Div5 = 0b111 << 27,
    }
    /// MCO1 prescaler
    pub enum Moc1pre {
        Div2 = 0b100 << 24,
        Div3 = 0b101 << 24,
        Div4 = 0b110 << 24,
        Div5 = 0b111 << 24,
    }
    /// I2S clock selection
    pub enum I2ssrc {
        Plli2sClock = 0b0 << 23,
        ExternalClock = 0b1 << 23, // External clock mapped on the I2S_CKIN pin used as I2S clock source
    }
    /// Microcontroller clock output 1
    pub enum Mco1 {
        HsiClock = 0b00 << 21,
        LseOscillator = 0b01 << 21,
        HseOscillatorClock = 0b10 << 21,
        PllClock = 0b11 << 21,
    }
    /// HSE division factor for RTC clock
    pub const RTCPRE_MASK: u32 = 0x001F_0000;
    /// APB high-speed prescaler (APB2)
    pub const PPRE2_MASK: u32 = 0b111 << 13;
    pub enum Ppre2 {
        Div1 = 0b000 << 13,
        Div2 = 0b100 << 13,
        Div4 = 0b101 << 13,
        Div8 = 0b110 << 13,
        Div16 = 0b111 << 13,
    }
    /// APB Low speed prescaler (APB1)
    pub const PPRE1_MASK: u32 = 0b111 << 10;
    pub enum Ppre1 {
        Div1 = 0b000 << 10,
        Div2 = 0b100 << 10,
        Div4 = 0b101 << 10,
        Div8 = 0b110 << 10,
        Div16 = 0b111 << 10,
    }
    /// AHB prescaler
    pub const HPRE_MASK: u32 = 0b1111 << 4;
    pub enum Hpre {
        Div1 = 0b0000 << 4, // sytem clock not divided
        Div2 = 0b1000 << 4, // sytem clock divided by 2
        Div4 = 0b1001 << 4,
        Div8 = 0b1010 << 4,
        Div16 = 0b1011 << 4,
        Div64 = 0b1100 << 4,
        Div128 = 0b1101 << 4,
        Div256 = 0b1110 << 4,
        Div512 = 0b1111 << 4,
    }
    /// System clock switch status
    pub const SWS_MASK: u32 = 0b11 << 2;
    pub enum Sws {
        HsiOscillator = 0b00 << 2, // used as system clock
        HseOscillator = 0b01 << 2,
        Pll = 0b10 << 2,
        NotApplicable = 0b11 << 2,
    }
    /// System clock switch
    pub const SW_MASK: u32 = 0b11 << 0;
    pub enum Sw {
        HsiOscillator = 0b00 << 0, // selected as system clock
        HseOscillator = 0b01 << 0,
        Pll = 0b10 << 0,
        NotAllow = 0b11 << 0,
    }
}

pub mod ahb1enr {
    /// DMA2 clock enable
    pub enum Dma2en {
        Disable = 0b0 << 22,
        Enable = 0b1 << 22,
    }
    /// DMA1 clock enable
    pub enum Dma1en {
        Disable = 0b0 << 21,
        Enable = 0b1 << 21,
    }
    /// CRC clock enable
    pub enum Crcen {
        Disable = 0b0 << 12,
        Enable = 0b1 << 12,
    }
    /// IO port H clock enable
    pub enum Gpiohen {
        Disable = 0b0 << 7,
        Enable = 0b1 << 7,
    }
    /// IO port E clock enable
    pub enum Gpioeen {
        Disable = 0b0 << 4,
        Enable = 0b1 << 4,
    }
    /// IO port D clock enable
    pub enum Gpioden {
        Disable = 0b0 << 3,
        Enable = 0b1 << 3,
    }
    /// IO port C clock enable
    pub enum Gpiocen {
        Disable = 0b0 << 2,
        Enable = 0b1 << 2,
    }
    /// IO port B clock enable
    pub enum Gpioben {
        Disable = 0b0 << 1,
        Enable = 0b1 << 1,
    }
    /// IO port A clock enable
    pub enum Gpioaen {
        Disable = 0b0 << 0,
        Enable = 0b1 << 0,
    }
}

pub mod apb1enr {
    /// Power interface clock enable
    pub enum Pwren {
        Disable = 0b0 << 28,
        Enable = 0b1 << 28,
    }
    /// I2C3 clock enable
    pub enum I2c3en {
        Disable = 0b0 << 23,
        Enable = 0b1 << 23,
    }
    /// I2C2 clock enable
    pub enum I2c2en {
        Disable = 0b0 << 22,
        Enable = 0b1 << 22,
    }
    /// I2C1 clock enable
    pub enum I2c1en {
        Disable = 0b0 << 21,
        Enable = 0b1 << 21,
    }
    /// USART2 clock enable
    pub enum Usart2en {
        Disable = 0b0 << 17,
        Enable = 0b1 << 17,
    }
    /// SPI3 clock enable
    pub enum Spi3en {
        Disable = 0b0 << 15,
        Enable = 0b1 << 15,
    }
    /// SPI2 clock enable
    pub enum Spi2en {
        Disable = 0b0 << 14,
        Enable = 0b1 << 14,
    }
    /// Window watchdog clock enable
    pub enum Wwdgen {
        Disable = 0b0 << 11,
        Enable = 0b1 << 11,
    }
    /// TIM5 clock enable
    pub enum Tim5en {
        Disable = 0b0 << 3,
        Enable = 0b1 << 3,
    }
    /// TIM4 clock enable
    pub enum Tim4en {
        Disable = 0b0 << 2,
        Enable = 0b1 << 2,
    }
    /// TIM3 clock enable
    pub enum Tim3en {
        Disable = 0b0 << 1,
        Enable = 0b1 << 1,
    }
    /// TIM2 clock enable
    pub enum Tim2en {
        Disable = 0b0 << 0,
        Enable = 0b1 << 0,
    }
}

pub mod dckcfgr {
    /// Timers clocks prescalers selection
    pub const TIMPRE_MASK: u32 = 0b1 << 24;
    pub enum Timpre {
        X2 = 0b0 << 24,
        X4 = 0b1 << 24,
    }
}

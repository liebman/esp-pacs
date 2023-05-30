#![doc = "Peripheral access API for ESP32-C2 microcontrollers (generated using svd2rust v0.28.0 (2d02a20 2023-05-30))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/46717278")]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn WIFI_MAC();
    fn WIFI_MAC_NMI();
    fn WIFI_PWR();
    fn WIFI_BB();
    fn BT_MAC();
    fn BT_BB();
    fn BT_BB_NMI();
    fn LP_TIMER();
    fn COEX();
    fn BLE_TIMER();
    fn BLE_SEC();
    fn I2C_MST();
    fn APB_CTRL();
    fn GPIO();
    fn GPIO_NMI();
    fn SPI_INTR_1();
    fn SPI_INTR_2();
    fn UART0();
    fn UART1();
    fn LEDC();
    fn EFUSE();
    fn RTC_CORE();
    fn I2C_EXT0();
    fn TG0_T0_LEVEL();
    fn TG0_WDT_LEVEL();
    fn CACHE_IA();
    fn SYSTIMER_TARGET0();
    fn SYSTIMER_TARGET1();
    fn SYSTIMER_TARGET2();
    fn SPI_MEM_REJECT_CACHE();
    fn ICACHE_PRELOAD0();
    fn ICACHE_SYNC0();
    fn APB_ADC();
    fn DMA_CH0();
    fn SHA();
    fn ECC();
    fn FROM_CPU_INTR0();
    fn FROM_CPU_INTR1();
    fn FROM_CPU_INTR2();
    fn FROM_CPU_INTR3();
    fn ASSIST_DEBUG();
    fn ETS_CORE0_PIF_PMS_SIZE();
}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".trap.rodata"]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 42] = [
    Vector { _handler: WIFI_MAC },
    Vector {
        _handler: WIFI_MAC_NMI,
    },
    Vector { _handler: WIFI_PWR },
    Vector { _handler: WIFI_BB },
    Vector { _handler: BT_MAC },
    Vector { _handler: BT_BB },
    Vector {
        _handler: BT_BB_NMI,
    },
    Vector { _handler: LP_TIMER },
    Vector { _handler: COEX },
    Vector {
        _handler: BLE_TIMER,
    },
    Vector { _handler: BLE_SEC },
    Vector { _handler: I2C_MST },
    Vector { _handler: APB_CTRL },
    Vector { _handler: GPIO },
    Vector { _handler: GPIO_NMI },
    Vector {
        _handler: SPI_INTR_1,
    },
    Vector {
        _handler: SPI_INTR_2,
    },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: LEDC },
    Vector { _handler: EFUSE },
    Vector { _handler: RTC_CORE },
    Vector { _handler: I2C_EXT0 },
    Vector {
        _handler: TG0_T0_LEVEL,
    },
    Vector {
        _handler: TG0_WDT_LEVEL,
    },
    Vector { _handler: CACHE_IA },
    Vector {
        _handler: SYSTIMER_TARGET0,
    },
    Vector {
        _handler: SYSTIMER_TARGET1,
    },
    Vector {
        _handler: SYSTIMER_TARGET2,
    },
    Vector {
        _handler: SPI_MEM_REJECT_CACHE,
    },
    Vector {
        _handler: ICACHE_PRELOAD0,
    },
    Vector {
        _handler: ICACHE_SYNC0,
    },
    Vector { _handler: APB_ADC },
    Vector { _handler: DMA_CH0 },
    Vector { _handler: SHA },
    Vector { _handler: ECC },
    Vector {
        _handler: FROM_CPU_INTR0,
    },
    Vector {
        _handler: FROM_CPU_INTR1,
    },
    Vector {
        _handler: FROM_CPU_INTR2,
    },
    Vector {
        _handler: FROM_CPU_INTR3,
    },
    Vector {
        _handler: ASSIST_DEBUG,
    },
    Vector {
        _handler: ETS_CORE0_PIF_PMS_SIZE,
    },
];
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "APB (Advanced Peripheral Bus) Controller"]
pub struct APB_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_CTRL {}
impl APB_CTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const apb_ctrl::RegisterBlock = 0x6002_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_ctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for APB_CTRL {
    type Target = apb_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for APB_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CTRL").finish()
    }
}
#[doc = "APB (Advanced Peripheral Bus) Controller"]
pub mod apb_ctrl;
#[doc = "SAR (Successive Approximation Register) Analog-to-Digital Converter"]
pub struct APB_SARADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_SARADC {}
impl APB_SARADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const apb_saradc::RegisterBlock = 0x6004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_saradc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for APB_SARADC {
    type Target = apb_saradc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for APB_SARADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC").finish()
    }
}
#[doc = "SAR (Successive Approximation Register) Analog-to-Digital Converter"]
pub mod apb_saradc;
#[doc = "Debug Assist"]
pub struct ASSIST_DEBUG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ASSIST_DEBUG {}
impl ASSIST_DEBUG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const assist_debug::RegisterBlock = 0x600c_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const assist_debug::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ASSIST_DEBUG {
    type Target = assist_debug::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ASSIST_DEBUG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSIST_DEBUG").finish()
    }
}
#[doc = "Debug Assist"]
pub mod assist_debug;
#[doc = "BB Peripheral"]
pub struct BB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BB {}
impl BB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bb::RegisterBlock = 0x6001_d000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BB {
    type Target = bb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BB").finish()
    }
}
#[doc = "BB Peripheral"]
pub mod bb;
#[doc = "DMA (Direct Memory Access) Controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma::RegisterBlock = 0x6003_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA").finish()
    }
}
#[doc = "DMA (Direct Memory Access) Controller"]
pub mod dma;
#[doc = "ECC (ECC Hardware Accelerator)"]
pub struct ECC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECC {}
impl ECC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ecc::RegisterBlock = 0x6003_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ecc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ECC {
    type Target = ecc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ECC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC").finish()
    }
}
#[doc = "ECC (ECC Hardware Accelerator)"]
pub mod ecc;
#[doc = "eFuse Controller"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efuse::RegisterBlock = 0x6000_8800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE").finish()
    }
}
#[doc = "eFuse Controller"]
pub mod efuse;
#[doc = "External Memory"]
pub struct EXTMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTMEM {}
impl EXTMEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const extmem::RegisterBlock = 0x600c_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const extmem::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EXTMEM {
    type Target = extmem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EXTMEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM").finish()
    }
}
#[doc = "External Memory"]
pub mod extmem;
#[doc = "General Purpose Input/Output"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x6000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpio;
#[doc = "I2C (Inter-Integrated Circuit) Controller 0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x6001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "I2C (Inter-Integrated Circuit) Controller 0"]
pub mod i2c0;
#[doc = "Interrupt Controller (Core 0)"]
pub struct INTERRUPT_CORE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INTERRUPT_CORE0 {}
impl INTERRUPT_CORE0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const interrupt_core0::RegisterBlock = 0x600c_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const interrupt_core0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for INTERRUPT_CORE0 {
    type Target = interrupt_core0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for INTERRUPT_CORE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_CORE0").finish()
    }
}
#[doc = "Interrupt Controller (Core 0)"]
pub mod interrupt_core0;
#[doc = "Input/Output Multiplexer"]
pub struct IO_MUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IO_MUX {}
impl IO_MUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const io_mux::RegisterBlock = 0x6000_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const io_mux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IO_MUX {
    type Target = io_mux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IO_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX").finish()
    }
}
#[doc = "Input/Output Multiplexer"]
pub mod io_mux;
#[doc = "LED Control PWM (Pulse Width Modulation)"]
pub struct LEDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDC {}
impl LEDC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ledc::RegisterBlock = 0x6001_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ledc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LEDC {
    type Target = ledc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LEDC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC").finish()
    }
}
#[doc = "LED Control PWM (Pulse Width Modulation)"]
pub mod ledc;
#[doc = "MODEM_CLKRST Peripheral"]
pub struct MODEM_CLKRST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MODEM_CLKRST {}
impl MODEM_CLKRST {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const modem_clkrst::RegisterBlock = 0x6004_d800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const modem_clkrst::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MODEM_CLKRST {
    type Target = modem_clkrst::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MODEM_CLKRST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_CLKRST").finish()
    }
}
#[doc = "MODEM_CLKRST Peripheral"]
pub mod modem_clkrst;
#[doc = "Hardware Random Number Generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rng::RegisterBlock = 0x6002_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RNG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG").finish()
    }
}
#[doc = "Hardware Random Number Generator"]
pub mod rng;
#[doc = "Real-Time Clock Control"]
pub struct RTC_CNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_CNTL {}
impl RTC_CNTL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc_cntl::RegisterBlock = 0x6000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_cntl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC_CNTL {
    type Target = rtc_cntl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_CNTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CNTL").finish()
    }
}
#[doc = "Real-Time Clock Control"]
pub mod rtc_cntl;
#[doc = "SENSITIVE Peripheral"]
pub struct SENSITIVE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SENSITIVE {}
impl SENSITIVE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sensitive::RegisterBlock = 0x600c_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sensitive::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SENSITIVE {
    type Target = sensitive::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SENSITIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENSITIVE").finish()
    }
}
#[doc = "SENSITIVE Peripheral"]
pub mod sensitive;
#[doc = "SHA (Secure Hash Algorithm) Accelerator"]
pub struct SHA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHA {}
impl SHA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sha::RegisterBlock = 0x6003_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sha::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SHA {
    type Target = sha::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SHA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA").finish()
    }
}
#[doc = "SHA (Secure Hash Algorithm) Accelerator"]
pub mod sha;
#[doc = "SPI (Serial Peripheral Interface) Controller 0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi0::RegisterBlock = 0x6000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "SPI (Serial Peripheral Interface) Controller 0"]
pub mod spi0;
#[doc = "SPI (Serial Peripheral Interface) Controller 1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x6000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "SPI (Serial Peripheral Interface) Controller 1"]
pub mod spi1;
#[doc = "SPI (Serial Peripheral Interface) Controller 2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi2::RegisterBlock = 0x6002_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI2 {
    type Target = spi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
#[doc = "SPI (Serial Peripheral Interface) Controller 2"]
pub mod spi2;
#[doc = "System Configuration Registers"]
pub struct SYSTEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM {}
impl SYSTEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const system::RegisterBlock = 0x600c_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSTEM {
    type Target = system::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSTEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM").finish()
    }
}
#[doc = "System Configuration Registers"]
pub mod system;
#[doc = "System Timer"]
pub struct SYSTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTIMER {}
impl SYSTIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const systimer::RegisterBlock = 0x6002_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const systimer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSTIMER {
    type Target = systimer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSTIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER").finish()
    }
}
#[doc = "System Timer"]
pub mod systimer;
#[doc = "Timer Group 0"]
pub struct TIMG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG0 {}
impl TIMG0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timg0::RegisterBlock = 0x6001_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMG0 {
    type Target = timg0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG0").finish()
    }
}
#[doc = "Timer Group 0"]
pub mod timg0;
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x6000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller 0"]
pub mod uart0;
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x6001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller 1"]
pub use self::uart0 as uart1;
#[doc = "XTS-AES-128 Flash Encryption"]
pub struct XTS_AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XTS_AES {}
impl XTS_AES {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const xts_aes::RegisterBlock = 0x600c_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xts_aes::RegisterBlock {
        Self::PTR
    }
}
impl Deref for XTS_AES {
    type Target = xts_aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for XTS_AES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTS_AES").finish()
    }
}
#[doc = "XTS-AES-128 Flash Encryption"]
pub mod xts_aes;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "APB_CTRL"]
    pub APB_CTRL: APB_CTRL,
    #[doc = "APB_SARADC"]
    pub APB_SARADC: APB_SARADC,
    #[doc = "ASSIST_DEBUG"]
    pub ASSIST_DEBUG: ASSIST_DEBUG,
    #[doc = "BB"]
    pub BB: BB,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "ECC"]
    pub ECC: ECC,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "EXTMEM"]
    pub EXTMEM: EXTMEM,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "INTERRUPT_CORE0"]
    pub INTERRUPT_CORE0: INTERRUPT_CORE0,
    #[doc = "IO_MUX"]
    pub IO_MUX: IO_MUX,
    #[doc = "LEDC"]
    pub LEDC: LEDC,
    #[doc = "MODEM_CLKRST"]
    pub MODEM_CLKRST: MODEM_CLKRST,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "RTC_CNTL"]
    pub RTC_CNTL: RTC_CNTL,
    #[doc = "SENSITIVE"]
    pub SENSITIVE: SENSITIVE,
    #[doc = "SHA"]
    pub SHA: SHA,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SYSTEM"]
    pub SYSTEM: SYSTEM,
    #[doc = "SYSTIMER"]
    pub SYSTIMER: SYSTIMER,
    #[doc = "TIMG0"]
    pub TIMG0: TIMG0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "XTS_AES"]
    pub XTS_AES: XTS_AES,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            APB_CTRL: APB_CTRL {
                _marker: PhantomData,
            },
            APB_SARADC: APB_SARADC {
                _marker: PhantomData,
            },
            ASSIST_DEBUG: ASSIST_DEBUG {
                _marker: PhantomData,
            },
            BB: BB {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            ECC: ECC {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            EXTMEM: EXTMEM {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            INTERRUPT_CORE0: INTERRUPT_CORE0 {
                _marker: PhantomData,
            },
            IO_MUX: IO_MUX {
                _marker: PhantomData,
            },
            LEDC: LEDC {
                _marker: PhantomData,
            },
            MODEM_CLKRST: MODEM_CLKRST {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            RTC_CNTL: RTC_CNTL {
                _marker: PhantomData,
            },
            SENSITIVE: SENSITIVE {
                _marker: PhantomData,
            },
            SHA: SHA {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SYSTEM: SYSTEM {
                _marker: PhantomData,
            },
            SYSTIMER: SYSTIMER {
                _marker: PhantomData,
            },
            TIMG0: TIMG0 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            XTS_AES: XTS_AES {
                _marker: PhantomData,
            },
        }
    }
}

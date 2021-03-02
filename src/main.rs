// src/main.rs

// std and main are not available for bare metal software
#![no_std]
#![no_main]

mod lcd;

use lcd::LCD;
use panic_halt as _;

use cortex_m_rt::entry;
use hal::{delay::Delay, pac, prelude::*};
use stm32f1xx_hal as hal;

#[entry]
fn main() -> ! {
    /* Get access to device and core peripherals */
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    /* Get access to RCC, AFIO and GPIOA */
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Freeze clocks
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Get delay instance
    let delay = Delay::new(cp.SYST, clocks);

    // Configure pin for LCD
    let rs = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let en = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let d4 = gpioc.pc0.into_push_pull_output(&mut gpioc.crl);
    let d5 = gpioc.pc1.into_push_pull_output(&mut gpioc.crl);
    let d6 = gpioc.pc2.into_push_pull_output(&mut gpioc.crl);
    let d7 = gpioc.pc3.into_push_pull_output(&mut gpioc.crl);

    let mut lcd = LCD::new(rs, en, d4, d5, d6, d7, delay);
    lcd.init();

    lcd.send_string("Hello World!");

    loop {}
}

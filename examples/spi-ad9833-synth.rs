#![no_std]
#![no_main]

use ad983x::{Ad983x, FrequencyRegister, MODE};
use cortex_m_rt::entry;
use embedded_hal_one::spi::blocking::ExclusiveDevice;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*, spi::NoMiso, timer::Timer};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low();

    let sck = gpiob.pb3.into_alternate();
    let miso = NoMiso {};
    let mosi = gpiob.pb5.into_alternate();
    let cs = gpiob.pb13.into_push_pull_output();

    let mut delay = Timer::syst(cp.SYST, &clocks).delay();

    // Change spi transfer mode to Bidi for more efficient operations.
    // let spi = Spi::new(dp.SPI1, (sck, miso, mosi), mode, 8.MHz(), &clocks).to_bidi_transfer_mode();
    // or
    let spi = dp.SPI1.spi_bidi((sck, miso, mosi), MODE, 8.MHz(), &clocks);
    let spidev = ExclusiveDevice::new(spi, cs);

    let mut synth = Ad983x::new_ad9833(spidev);
    synth.reset().unwrap();
    synth.set_frequency(FrequencyRegister::F0, 4724).unwrap();
    synth.enable().unwrap();
    // Given a 25 MHz clock, this now outputs a sine wave
    // with a frequency of 440 Hz, which is a standard
    // A4 tone.

    loop {
        delay.delay_ms(500_u16);
        led.toggle();
    }
}

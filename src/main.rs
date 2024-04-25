//! Trigger a relay on one of the GPIO ports.

#![no_std]
#![no_main]


use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::bind_interrupts;
use embassy_time::{Timer};

use {defmt_rtt as _, panic_probe as _};

pub mod ws2812;
use crate::ws2812::Ws2812;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

// ================================================================================

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Start");

    let p = embassy_rp::init(Default::default());
    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);
    let mut ws2812 = Ws2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_15);

    // =====

    let mut gpio1 = Output::new(p.PIN_18, Level::Low);
    let mut gpio2 = Output::new(p.PIN_19, Level::Low);
    let mut gpio3 = Output::new(p.PIN_28, Level::Low);

    let mut led1 = Output::new(p.PIN_6, Level::Low);
    let mut led2 = Output::new(p.PIN_7, Level::Low);
    let mut led3 = Output::new(p.PIN_8, Level::Low);

    // =====

    info!("NeoPixel off");
    ws2812.write(&[(0,0,0).into()]).await;
    Timer::after_secs(1).await;

    loop {
	// RED
	ws2812.write(&[(0,255,0).into()]).await;
	Timer::after_secs(1).await;

	led1.set_high();
	gpio1.set_high();
	Timer::after_secs(2).await;
	gpio1.set_low();
	led1.set_low();

	led2.set_high();
	gpio2.set_high();
	Timer::after_secs(2).await;
	gpio2.set_low();
	led2.set_low();

	led3.set_high();
	gpio3.set_high();
	Timer::after_secs(2).await;
	gpio3.set_low();
	led3.set_low();

	// BLUE
	ws2812.write(&[(0,0,255).into()]).await;
	Timer::after_secs(1).await;
    }
}

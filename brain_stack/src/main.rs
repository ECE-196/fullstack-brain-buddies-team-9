#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::{
    gpio::{Io, Level, Output},
    timer::timg::TimerGroup,
};
use esp_println::println;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut led = Output::new(io.pins.gpio17, Level::Low);

    loop {
        println!("Hello, World!");
        led.toggle();
        Timer::after_millis(1_000).await;
    }
}

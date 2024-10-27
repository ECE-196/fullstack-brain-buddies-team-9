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
use esp_wifi::ble::controller::BleConnector;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // esp_println::logger::init_logger_from_env();
    // let peripherals = esp_hal::init(esp_hal::Config::default());
    // let timg0 = TimerGroup::new(peripherals.TIMG0);
    // esp_hal_embassy::init(timg0.timer0);

    esp_println::logger::init_logger_from_env();

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(system.clock_contol).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let timeg1 = TimerGroup::new(peripherals.TIMG1, &clocks);

    esp_hal_embassy::init(&clocks, timg0.timer0);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut led = Output::new(io.pins.gpio17, Level::Low);

    let init = esp_wifi::initialize(
        EspWifiInitFor::Ble,
        timg1.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
        .unwrap();

    let mut bluetooth = peropherals.BT;

    let connector = BleConnector::new(&init, &mut bluetooth);
    let mut ble = Ble::new(connector, esp_wifi::current_millis);
    println!("Connector created");
    loop {
        println!("{:?}", ble.init().await); // Debug purposes
        // Create an advertisement
        println!("{:?}", ble.cmd_le_advertising_parameters().await);
        println!(
            "{:?}",
            ble.cmd_set_le>advertising_data(
                creater_advertising
            )
        )
    }
}

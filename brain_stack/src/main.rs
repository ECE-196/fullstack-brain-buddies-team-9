#![no_std]
#![no_main]

use bleps::gatt;
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
    let led_ref = RefCell::new(led);
    let led_ref = &led_ref;

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
            ble.cmd_set_le_advertising_data(
                create_advertising_data(&[
                    AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                    AdStructure::ServiceUuids128(&[Uuid::Uuid128([
                        0x38, 0xcf, 0x62, 0x0a, 0xc3, 0xfb, 0x10, 0x9f, 0xeb, 0x11, 0x54, 0x23,
                        0xe0, 0x12, 0x73, 0x93
                    ])]),
                    AdStructure::CompleteLocalName("ece196"),
                ])
                    .unwrap()
            )
                .await
        );
        println!("{:?}", ble.cmd_set_le_advertise_enable(true).await);
        println!("started advertising");
        // create a GATT Server
        gatt!([service {
            uuid: "937312e0-2354-11eb-9f10-fbc30a62cf38",
            characteristics: [characteristic {
                uuid: "937312e0-2354-11eb-9f10-fbc30a62cf38",
                read: rf
                write: wf
            },],
        },]);
        println!("{:?}", gatt_attributes);

        // create two callbacks
        // responsible for fetching the data request and determining the data length
        let mut rf = |_offset: usize, data: &mut [u8]| {
            let buf = (led.is_set_high() as u8).to_be_bytes();
            data[..buf.len()].copy_from_slice(&buf);

            buf.len()
        };

        // responsible for taking action based on receiving data
        let mut wf = |_offset: usize, data: &[u8]| {
            if data.len() != 1 {
                panic!(
                    "Invalid data received. Expected length 1, got {}",
                    data.len()
                );
            }

            led.set_level(match data[0] {
                0 => Level::Low,
                1 => Level::High,
                val => {
                    panic!("Invalid data received. Expected [0, 1], got {}", val);
                }
            });
        };

        // more callbacks
        let mut rf = |_offset: usize, data: &mut [u8]| {
            let buf = (led_ref.borrow().is_set_high() as u8).to_be_bytes();
            data[..buf.len()].copy_from_slice(&buf);

            buf.len()
        };

        let mut wf = |_offset: usize, data: &[u8]| {
            if data.len() != 1 {
                panic!(
                    "Invalid data received. Expected length 1, got {}",
                    data.len()
                );
            }

            led_ref.borrow_mut().set_level(match data[0] {
                0 => Level::Low,
                1 => Level::High,
                val => {
                    panic!("Invalid data received. Expected [0, 1], got {}", val);
                }
            });
        };

        // start the server
        let mut rng = bleps::no_rng::NoRng;
        let mut srv = AttributeServer::new(&mut ble, &mut gatt_attributes, &mut rng);

        let mut notifier = || future::pending();

        srv.run(&mut notifier).await.unwrap();
    }
}

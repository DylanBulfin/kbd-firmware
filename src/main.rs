//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use rp2040_boot2;
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

use rp2040_hal::{
    self as hal, entry,
    gpio::{DynFunction, DynPinId, FunctionSio, Pin, Pins, PullDown, PullUp, SioInput, SioOutput},
    usb::UsbBus,
    Timer,
};

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::{InputPin, OutputPin};
use fugit::ExtU32;
use panic_probe as _;

use cortex_m::prelude::*;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use usb_device::device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid};
use usb_device::{bus::UsbBusAllocator, device::UsbDevice};

use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::prelude::UsbHidClassBuilder;
use usbd_human_interface_device::UsbHidError;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(
            usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig::default(),
        )
        .build(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .strings(&[StringDescriptors::default()
            .manufacturer("Dylan Bulfin")
            .product("Boot keyboard")
            .serial_number("TEST")])
        .unwrap()
        .build();

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(1.millis());

    let mut scan_count_down = timer.count_down();
    scan_count_down.start(10.millis());

    let mut row_pins: [Pin<DynPinId, FunctionSio<SioInput>, PullDown>; 4] = [
        pins.gpio4.into_pull_down_input().into_dyn_pin(),
        pins.gpio5.into_pull_down_input().into_dyn_pin(),
        pins.gpio6.into_pull_down_input().into_dyn_pin(),
        pins.gpio7.into_pull_down_input().into_dyn_pin(),
    ];

    let mut r_col_pins: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; 6] = [
        pins.gpio20.into_push_pull_output().into_dyn_pin(),
        pins.gpio22.into_push_pull_output().into_dyn_pin(),
        pins.gpio26.into_push_pull_output().into_dyn_pin(),
        pins.gpio27.into_push_pull_output().into_dyn_pin(),
        pins.gpio28.into_push_pull_output().into_dyn_pin(),
        pins.gpio29.into_push_pull_output().into_dyn_pin(),
    ];

    loop {
        if tick_count_down.wait().is_ok() {
            match keyboard.tick() {
                Err(UsbHidError::WouldBlock) | Ok(_) => {}
                Err(e) => core::panic!("Failed to process keyboard tick: {:?}", e),
            }

            watchdog.feed();
        }

        if usb_dev.poll(&mut [&mut keyboard]) {
            keyboard.device().read_report();
        }

        if scan_count_down.wait().is_ok() {
            let keys = do_matrix_scan(&mut row_pins, &mut r_col_pins, timer);
            // let keys = [Keyboard::Z; 1];

            match keyboard.device().write_report(keys) {
                Err(UsbHidError::WouldBlock) => {}
                Err(UsbHidError::Duplicate) => {}
                Ok(_) => {}
                Err(e) => {
                    core::panic!("Failed to write keyboard report: {:?}", e)
                }
            }
        }
    }
}

const BASE_KEYS: [[Keyboard; 6]; 4] = [
    [
        Keyboard::A,
        Keyboard::B,
        Keyboard::C,
        Keyboard::D,
        Keyboard::E,
        Keyboard::F,
    ],
    [
        Keyboard::G,
        Keyboard::H,
        Keyboard::I,
        Keyboard::J,
        Keyboard::K,
        Keyboard::L,
    ],
    [
        Keyboard::M,
        Keyboard::N,
        Keyboard::O,
        Keyboard::P,
        Keyboard::Q,
        Keyboard::R,
    ],
    [
        Keyboard::S,
        Keyboard::T,
        Keyboard::U,
        Keyboard::V,
        Keyboard::W,
        Keyboard::X,
    ],
];

fn do_matrix_scan(
    row_pins: &mut [Pin<DynPinId, FunctionSio<SioInput>, PullDown>; 4],
    col_pins: &mut [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; 6],
    timer: Timer,
) -> [Keyboard; 24] {
    let mut res: [Keyboard; 24] = [Keyboard::NoEventIndicated; 24];

    for cpin in col_pins.iter_mut() {
        cpin.set_low().ok();
    }

    for (c, cpin) in col_pins.iter_mut().enumerate() {
        {
            // Wait for debounce
            let mut bounce_timer = timer.count_down();
            bounce_timer.start(10.micros());
            while let Err(_) = bounce_timer.wait() {}
        }

        // Set High
        cpin.set_high().ok();

        {
            // Wait for debounce
            let mut bounce_timer = timer.count_down();
            bounce_timer.start(10.micros());
            while let Err(_) = bounce_timer.wait() {}
        }

        for (r, rpin) in row_pins.iter_mut().enumerate() {
            if rpin.is_high().unwrap_or(false) {
                // Key is active
                res[6 * r + c] = BASE_KEYS[r][c];
            }
        }

        cpin.set_low().ok();
    }

    res
}

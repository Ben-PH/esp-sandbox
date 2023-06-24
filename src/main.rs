#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    pulse_control::{ClockSource, ConfiguredChannel, OutputChannel, PulseCode, RepeatMode},
    timer::TimerGroup,
    PulseControl, Rtc, IO,
};
use esp_backtrace as _;
// use esp_hal_smartled::{smartLedAdapter, SmartLedsAdapter};
// use esp_println::println;
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handles for the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
        ClockSource::APB,
        0,
        0,
        0,
    )
    .unwrap();
    let mut rmt_channel0 = pulse.channel0;
    rmt_channel0
        .set_idle_output_level(false)
        .set_carrier_modulation(false)
        .set_channel_divider(1)
        .set_idle_output(true);
    let mut rmt_channel0 = rmt_channel0.assign_pin(io.pins.gpio2);
    let seq = [PulseCode {
        level1: true,
        length1: 400u32.nanos(),
        level2: false,
        length2: 1350u32.nanos(),
    }; 24];
    // Initialize and create handle for LEDC peripheral

    loop {
        rmt_channel0
            .send_pulse_sequence(RepeatMode::SingleShot, &seq)
            .unwrap();
    }
}

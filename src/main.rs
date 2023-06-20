#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    delay::Delay,
    ledc::{
        channel,
        timer::{self},
        LSGlobalClkSource, LowSpeed, LEDC,
    },
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handles for the RTC and TIMG watchdog timers
    // let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    // let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    // let mut wdt0 = timer_group0.wdt;
    // let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    // let mut wdt1 = timer_group1.wdt;

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led_pin = io.pins.gpio7.into_push_pull_output();
    // rtc.swd.disable();
    // rtc.rwdt.disable();
    // wdt0.disable();
    // wdt1.disable();
    // Initialize and create handle for LEDC peripheral
    let mut ledc = LEDC::new(
        peripherals.LEDC,
        &clocks,
        &mut system.peripheral_clock_control,
    );

    // Set up global clock source for LEDC to APB Clk
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    // set up a means to delay
    let mut delay = Delay::new(&clocks);
    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 100u32.Hz(),
        })
        .unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, led_pin);

    let mut moddo = 70;
    let rollover = 200;
    loop {
        let pct = if moddo < (rollover / 2) {
            moddo
        } else {
            rollover - moddo
        };
        channel0
            .configure(channel::config::Config {
                timer: &lstimer0,
                duty_pct: pct,
            })
            .unwrap();
        moddo += 1;
        moddo %= rollover;
        println!("pct: {}", pct);
        delay.delay_ms(10u32);
    }
}

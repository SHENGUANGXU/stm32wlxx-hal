// Prints over RTT when the state of B3 on the NUCLEO-WL55JC2 changes.

#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::rprintln;
use stm32wl_hal::{
    gpio::{pins, Input, Level, PortC, Pull},
    pac,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let channels = rtt_target::rtt_init! {
        up: {
            0: {
                size: 4096
                mode: BlockIfFull
                name: "Terminal"
            }
        }
    };
    rtt_target::set_print_channel(channels.up.0);
    rprintln!("Hello from rprintln!");

    let mut dp: pac::Peripherals = pac::Peripherals::take().unwrap();

    let gpioc: PortC = PortC::split(dp.GPIOC, &mut dp.RCC);
    let pc6: Input<pins::C6> = Input::new(gpioc.pc6, Pull::Up);

    let mut prev_level: Level = pc6.level();
    rprintln!("B3 initial level: {:?}", prev_level);

    loop {
        let level: Level = pc6.level();
        if level != prev_level {
            rprintln!("B3 state changed from {:?} to {:?}", prev_level, level);
            prev_level = level;
        }
    }
}
//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

pub use cortex_m_rt::entry;
pub use f3::{
    hal::{
        prelude::*,
        delay::Delay,
        prelude,
        stm32f30x,
        stm32f30x::GPIOA,
        gpio,
        gpio::gpioa
    },
    led::Leds,
};

pub fn init() -> (Delay, Leds, Button, OutPorts) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));
    let (button, ports) = createLev(&mut dp.GPIOA.split(&mut rcc.ahb));
       
    (delay, leds, button, ports)
}

fn createLev(&mut gpa: &mut gpioa::Parts) -> (Button, Ports) {
    gpa.pa0.into_floating_input(&mut gpa.moder, &mut gpa.pupdr);
    ( Button {
        idr : 
        unsafe {
            &(*GPIOA::ptr()).idr
        }
    },
    OutPorts{
        pa1:  gpa.pa1.into_push_pull_output(&mut gpa.moder, &mut gpa.otyper),
        pa2:  gpa.pa2.into_push_pull_output(&mut gpa.moder, &mut gpa.otyper),
        pa3:  gpa.pa3.into_push_pull_output(&mut gpa.moder, &mut gpa.otyper),
        pa4:  gpa.pa4.into_push_pull_output(&mut gpa.moder, &mut gpa.otyper)
    })
}

pub struct Button {
    idr : &'static stm32f30x::gpioa::IDR
}

impl Button {
     fn isPushed(&self) -> bool {
        self.idr.read().bits() & 0x1 == 1
    }
}

pub struct OutPorts {
    pa1 : gpioa::PA1<gpio::Output<gpio::PushPull>>,
    pa2 : gpioa::PA2<gpio::Output<gpio::PushPull>>,
    pa3 : gpioa::PA3<gpio::Output<gpio::PushPull>>,
    pa4 : gpioa::PA4<gpio::Output<gpio::PushPull>>
}


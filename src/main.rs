#![no_std]
#![no_main]


use tm4c123x_hal::{self as hal,prelude::*};
use cortex_m_rt::entry;
use panic_probe as _;

#[entry]
fn main() ->! {
    let p =hal::Peripherals::take().unwrap();

    let  sc = p.SYSCTL.constrain();
    // sc.clock_setup.oscillator = hal::sysctl::oscillator::main(hal::sysctl::CrystalFrequency::_16mhz,
    //     hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz));
    
    // let clock=sc.clock_setup.freeze();

    let porta = p.GPIO_PORTF.split(&sc.power_control);

    let mut led = porta.pf1.into_push_pull_output();
  
    led.set_high();

    loop{
        // led.into_pull_down_input();  
        // led.into_pull_up_input();       
                         

    }

}


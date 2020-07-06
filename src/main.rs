#![no_main]
#![no_std]
#[allow(unused_extern_crates)] 
#[allow(unused_imports)]
use panic_halt;
extern crate embedded_hal;
extern crate stm32f4xx_hal;
use stm32f4xx_hal::block;
use stm32f4xx_hal::serial::{config::Config, Serial};

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;

use hal::prelude::*; // need for the GpioExt trait (-> .split) 

#[rt::entry]
fn main() -> ! {
   if let Some(peripherals) = hal::stm32::Peripherals::take(){
       // let rcc= peripherals.SYSCFG.exticr4.
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        
        let gpiod = peripherals.GPIOD.split();
        let  tx = gpiod.pd8.into_alternate_af7(); 
        let  rx = gpiod.pd9.into_alternate_af7();
        let serial = Serial::usart3(
            peripherals.USART3,
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            clocks,
        )
        .unwrap();

      // Separate out the sender and receiver of the serial port
      let (mut tx, mut rx) = serial.split();
      
        
        
      loop {
         
        
          // Read character and echo it back
          let received = block!(rx.read()).unwrap();
          block!(tx.write(received)).ok();
        }
      }
    
  

  loop {
      continue;
  }
}
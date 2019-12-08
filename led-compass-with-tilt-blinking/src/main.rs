// code 4


#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::PI;
pub extern crate l3gd20;


#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, Direction, I16x3};
use m::Float;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {

        let I16x3 { x, y, z } = lsm303dlhc.mag().unwrap();

        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));


        let theta = (y as f32).atan2(x as f32); // in radians

        let dir = if theta < -7. * PI / 8. {
            Direction::North
        } else if theta < -5. * PI / 8. {
            Direction::Northwest
        } else if theta < -3. * PI / 8. {
            Direction::West
        } else if theta < -PI / 8. {
            Direction::Southwest
        } else if theta < PI / 8. {
            Direction::South
        } else if theta < 3. * PI / 8. {
            Direction::Southeast
        } else if theta < 5. * PI / 8. {
            Direction::East
        } else if theta < 7. * PI / 8. {
            Direction::Northeast
        } else {
            Direction::North
        };

        

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(100_u16);


        let I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();
        let x = i32::from(x)/100;
        let y = i32::from(y)/100;
        let z = i32::from(z)/100;

        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));




            if y > 3 || y <-3{
            while y > 3 || y <-3{

            let I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();

            let x = i32::from(x)/100;
            let y = i32::from(y)/100;
            let z = i32::from(z)/100;

            leds[0].on();
            leds[1].on();
            leds[2].on();
            leds[3].on();
            leds[4].on();
            leds[5].on();
            leds[6].on();
            leds[7].on();
            delay.delay_ms(100_u16);
            leds[0].off();
            leds[1].off();
            leds[2].off();
            leds[3].off();
            leds[4].off();
            leds[5].off();
            leds[6].off();
            leds[7].off();
            delay.delay_ms(100_u16);

            if y < 3 || y > -3 {
                break;
            }

            }
      }

    }
}


// code 4 (end)
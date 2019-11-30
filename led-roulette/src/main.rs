// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use aux5::entry;

// #[entry]
// fn main() -> ! {
//     let _y;
//     let x = 42;
//     _y = x;

//     // infinite loop; just so we don't leave this stack frame
//     loop {}
// }



// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use aux5::{entry, prelude::*, Delay, Leds};

// #[entry]
// fn main() -> ! {
//     let (mut delay, mut leds): (Delay, Leds) = aux5::init();

//     let ms = 150_u16;
//     loop {
//         loop {

//             leds[0].on();
//             delay.delay_ms(ms);
//             leds[0].off();
//             delay.delay_ms(ms);

//             leds[2].on();
//             delay.delay_ms(ms);
//             leds[2].off();
//             delay.delay_ms(ms);

//             leds[4].on();
//             delay.delay_ms(ms);
//             leds[4].off();
//             delay.delay_ms(ms);

//             leds[6].on();
//             delay.delay_ms(ms);
//             leds[6].off();
//             delay.delay_ms(ms);

//             break;
//         }

//         for curr in 0..8 {
//             let next = (curr + 1) % 8;

//             leds[next].on();
//             delay.delay_ms(ms);
//             leds[curr].off();
//         }

//         loop {

//             leds[1].on();
//             delay.delay_ms(ms);
//             leds[1].off();
//             delay.delay_ms(ms);

//             leds[3].on();
//             delay.delay_ms(ms);
//             leds[3].off();
//             delay.delay_ms(ms);

//             leds[5].on();
//             delay.delay_ms(ms);
//             leds[5].off();
//             delay.delay_ms(ms);

//             leds[7].on();
//             delay.delay_ms(ms);
//             leds[7].off();
//             delay.delay_ms(ms);

//             break;
//         }

//     }




//  #![deny(unsafe_code)]
//  #![no_main]
//  #![no_std]

//  use aux5::{entry, prelude::*, Delay, Leds};

//  #[entry]
//  fn main() -> ! {
//      let (mut delay, mut leds): (Delay, Leds) = aux5::init();

//      let ms = 500_u16;
//      loop {
//          for curr in 0..8 {
//              let next = (curr + 2) % 8;

//              leds[next].on();
//              delay.delay_ms(ms);
//              leds[curr].off();
//              delay.delay_ms(ms);
//          }
//     }
// }



 #![deny(unsafe_code)]
 #![no_main]
 #![no_std]

 use aux5::{entry, prelude::*, Delay, Leds};

 #[entry]
 fn main() -> ! {
     let (mut delay, mut leds): (Delay, Leds) = aux5::init();

     let ms = 150_u16;
     loop {

         for a in 0..8{
             leds[a].on();
             delay.delay_ms(ms);
         }

         for b in 0..8{
             leds[b].off();
             delay.delay_ms(ms);
         }

         for c in 0..8{
             leds[c].on();
             delay.delay_ms(ms);
         }

         for d in (0..8).rev(){

             leds[d].off();
             delay.delay_ms(ms);

         }
}
}
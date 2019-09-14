use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, Raspberry! from Mac");

    // Create a new LED attached to Pin 17
    let mut yellow = LED::new(21);
    let mut red = LED::new(26);
    // blink the LED
    //led.off();
       // led.blink(2.0, 3.0);
 //   loop{

       red.on();
       yellow.off();
       sleep(Duration::from_millis(200));
        yellow.on();
        sleep(Duration::from_millis(200));
        red.off();
        sleep(Duration::from_millis(200));
        yellow.off();
 //   }
        
}

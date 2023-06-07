extern crate wiringpi;

use wiringpi::pin::Value::{High, Low};
use std::{thread, time};

fn main() {
    
    println!("process has started");
    let interval = time::Duration::from_millis(1000);
    let wiringpi = wiringpi::setup();
    let pin = wiringpi.output_pin(0);
    loop {
        pin.digital_write(High);
        thread::sleep(interval);
        pin.digital_write(Low);
        thread::sleep(interval);
    }
}

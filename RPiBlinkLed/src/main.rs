use rust_gpiozero::*;
use std::time;
use std::thread;

fn main() {
    let blue = LED::new(17);
    let green = LED::new(27);
    let red = LED::new(22);
    blue.on();
    green.on();
    red.on();

    let sleep_time = time::Duration::from_secs(2); 
    loop{
       blue.off(); 
       green.on();
       red.on();
       thread::sleep(sleep_time);

       blue.on(); 
       green.off();
       red.on();
       thread::sleep(sleep_time);

       blue.on(); 
       green.on();
       red.off();
       thread::sleep(sleep_time);
    }
}

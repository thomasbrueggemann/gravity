use rppal::gpio::{Error, Gpio, Level, Trigger};
use std::time::SystemTime;
use std::io::stdin;
use std::thread;

mod serial;

fn main() -> Result<(), Error> {
    let gpio = Gpio::new()?;
    let mut input_pin = gpio.get(23)?.into_input_pullup();

    input_pin.set_async_interrupt(Trigger::Both, handle_input_change)?;
    
    thread::spawn(|| serial::handle_serial_port());

    stdin().read_line(&mut String::new())?;

    return Ok(());
}

fn handle_input_change(level: Level) {
    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    println!("{}: {}", epoch.as_secs(), level);
}
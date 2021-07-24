use std::io::stdin;
use rppal::gpio::{Error, Gpio, Trigger, Level};

fn main() -> Result<(), Error> {
    let gpio = Gpio::new()?;
    let mut input_pin = gpio.get(23)?.into_input_pullup();

    println!("{}", input_pin.read());

	input_pin.set_async_interrupt(Trigger::Both,  handle_input_change)?;

    stdin().read_line(&mut String::new())?;

	return Ok(());
}

fn handle_input_change(level: Level) {
	println!("{}", level);
}

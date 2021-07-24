use rppal::gpio::Gpio;

fn main() -> Result<(), rppal::gpio::Error> {
    let gpio = Gpio::new()?;
    let pin = gpio.get(23)?.into_input();

    println!("{}", pin.read());

	return Ok(());
}

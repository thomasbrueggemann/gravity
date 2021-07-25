use rppal::gpio::{Error, Gpio, Level, Trigger};
use std::time::{Duration, SystemTime};
use std::io::{self, stdin};
use std::thread;

fn main() -> Result<(), Error> {
    let gpio = Gpio::new()?;
    let mut input_pin = gpio.get(23)?.into_input_pullup();

    input_pin.set_async_interrupt(Trigger::Both, handle_input_change)?;

	thread::spawn(|| {
		let mut port = serialport::new("/dev/ttyACM0", 9_600)
			.timeout(Duration::from_millis(100))
			.open()
			.expect("Failed to open port");

		let mut serial_buf: Vec<u8> = vec![0; 1000];
		println!("Receiving data via serial port");

		loop {
			match port.read(serial_buf.as_mut_slice()) {
				Ok(t) => {
					let received_bytes = serial_buf[..t].to_vec();

					let stringed_buffer = String::from_utf8(received_bytes).unwrap();
					println!("{}", stringed_buffer);
				},
				Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
				Err(e) => eprintln!("{:?}", e),
			}
		
		}
	});

	stdin().read_line(&mut String::new())?;

	return Ok(());
}

fn handle_input_change(level: Level) {
    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    println!("{}: {}", epoch.as_secs(), level);
}

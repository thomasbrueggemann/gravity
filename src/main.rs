use rppal::gpio::{Error, Gpio, Level, Trigger};
use std::time::{Duration, SystemTime};
use std::io::{self, stdin};
use std::thread;
use std::sync::mpsc::{channel, Sender};

fn main() -> Result<(), Error> {
    let gpio = Gpio::new()?;
    let mut input_pin = gpio.get(23)?.into_input_pullup();

    input_pin.set_async_interrupt(Trigger::Both, handle_input_change)?;

	let (tx, rx) = channel::<Vec<u8>>();

    thread::spawn(|| handle_serial_port(tx));

    stdin().read_line(&mut String::new())?;

    return Ok(());
}

fn handle_input_change(level: Level) {
    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    println!("{}: {}", epoch.as_secs(), level);
}

fn handle_serial_port(tx: Sender<Vec<u8>>) {
	let mut port = serialport::new("/dev/ttyACM0", 9_600)
		.timeout(Duration::from_millis(100))
		.open()
		.expect("Failed to open port");

	let mut received_data: Vec<u8> = Vec::new();
	let seperator: Vec<u8> = vec![71, 86, 84, 89];

	println!("Receiving data via serial port");
	let mut serial_buf: Vec<u8> = vec![0; 1000];

	loop {
		match port.read(serial_buf.as_mut_slice()) {
			Ok(t) => {
				received_data.extend_from_slice(&serial_buf[..t]);

				let seq_start = find_subsequence(&received_data, &seperator);
				if seq_start.is_some() {
					let start_idx = seq_start.unwrap();
					received_data.drain(0..start_idx);
				}

				let seq_end = find_subsequence(&received_data[seperator.len()..], &seperator);
				if seq_end.is_some() {
					let end_idx = seq_end.unwrap();

					let seq = received_data
						.drain(0..end_idx)
						.as_slice()
						.to_vec();
					
					tx.send(seq).unwrap();
				}
			},
			Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
			Err(e) => eprintln!("{:?}", e)
		}
	}
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    return haystack
		.windows(needle.len())
		.position(|window| window == needle);
}
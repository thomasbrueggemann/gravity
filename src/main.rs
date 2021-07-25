use rppal::gpio::{Error, Gpio, Level, Trigger};
use std::time::{Duration, SystemTime};
use std::io::{self, stdin};
use std::thread;
use byteorder::{ByteOrder, LittleEndian};

struct Potis {
    pot1: i16
}

fn main() -> Result<(), Error> {
    let gpio = Gpio::new()?;
    let mut input_pin = gpio.get(23)?.into_input_pullup();

    input_pin.set_async_interrupt(Trigger::Both, handle_input_change)?;
    
    thread::spawn(|| handle_serial_port());

    stdin().read_line(&mut String::new())?;

    return Ok(());
}

fn handle_input_change(level: Level) {
    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    println!("{}: {}", epoch.as_secs(), level);
}

fn handle_serial_port() {
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

                println!("received data {}", received_data.len());

                let seq_start = find_subsequence(&received_data, &seperator);
                if seq_start.is_some() {
                    let start_idx = seq_start.unwrap();
                    received_data.drain(0..start_idx);
                }

                let seq_end = find_subsequence(&received_data[seperator.len()..], &seperator);
                if seq_end.is_some() {
                    let end_idx = seq_end.unwrap() + seperator.len();

                    let seq = received_data
                        .drain(0..end_idx)
                        .as_slice()
                        .to_vec();
                    
                    let parsed_seq = handle_serial_message_parsing(seq);
                    if parsed_seq.is_some() {
                        println!("{}", parsed_seq.unwrap().pot1)
                    }
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e)
        }
    }
}

fn handle_serial_message_parsing(data: Vec<u8>) -> Option<Potis> {
    if data.len() >= 6 {
		println!("{:?} - {:?}", data, data[4..5]);
        let pot1 = LittleEndian::read_i16(&data[4..5]);

        let potis = Potis {
            pot1
        };

        return Some(potis);
    }

    return None;
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    return haystack
        .windows(needle.len())
        .position(|window| window == needle);
}
use std::time::Duration;
use byteorder::{ByteOrder, LittleEndian};
use std::io;

pub struct Potis {
    pot1: i16,
	pot2: i16
}

pub fn handle_serial_port() {
    let mut port = serialport::new("/dev/ttyACM0", 9_600)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    let mut received_data: Vec<u8> = Vec::new();
    let mut serial_buf: Vec<u8> = vec![0; 1000];

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                received_data.extend_from_slice(&serial_buf[..t]);

				if received_data.len() <= 4 {
					return;
				}

                detect_next_message(&mut received_data);
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e)
        }
    }
}

fn detect_next_message(data: &mut Vec<u8>) {
    let seperator: Vec<u8> = vec![71, 86, 84, 89];

	let seq_start = find_subsequence(&data, &seperator);
	if seq_start.is_some() {
		let start_idx = seq_start.unwrap();
		data.drain(0..start_idx);
	}

	let seq_end = find_subsequence(&data[seperator.len()..], &seperator);
	if seq_end.is_some() {
		let end_idx = seq_end.unwrap() + seperator.len();

		let seq = data
			.drain(0..end_idx)
			.as_slice()
			.to_vec();
		
		let parsed_seq = handle_serial_message_parsing(seq);
		if parsed_seq.is_some() {
			let s = parsed_seq.unwrap();
			println!("{}, {}", s.pot1, s.pot2)
		}
	}
}

fn handle_serial_message_parsing(data: Vec<u8>) -> Option<Potis> {
    if data.len() >= 6 {
        let pot1 = LittleEndian::read_i16(&data[4..6]);
		let pot2 = LittleEndian::read_i16(&data[6..8]);

        let potis = Potis {
            pot1,
			pot2
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

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(test)]
	mod find_subsequence {
		use super::*;

		#[test]
		fn should_detect_a_subsequence_of_two_bytes_at_beginning() {
			let seq = vec![1, 2, 3, 4, 5];
			let sub_seq = vec![1, 2];

			let idx = find_subsequence(&seq, &sub_seq);

			assert_eq!(0, idx.unwrap());
		}

		#[test]
		fn should_detect_a_subsequence_of_two_bytes_at_the_end() {
			let seq = vec![1, 2, 3, 4, 5];
			let sub_seq = vec![4, 5];

			let idx = find_subsequence(&seq, &sub_seq);

			assert_eq!(3, idx.unwrap());
		}

		#[test]
		fn should_not_detect_a_subsequence_when_main_sequence_is_empty() {
			let seq = vec![];
			let sub_seq = vec![4, 5];

			let idx = find_subsequence(&seq, &sub_seq);

			assert_eq!(true, idx.is_none());
		}
	}
}
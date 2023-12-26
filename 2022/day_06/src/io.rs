use std::fs::File;
use std::io::{BufReader, Read};

pub fn fill_buffer(buffer: &mut [u8], reader: &mut BufReader<File>) -> bool {
    let bytes_read = reader
        .read(buffer)
        .expect("Could not read starting bytes into buffer.");
    return bytes_read < buffer.len();
}

pub fn next_sequence(buffer: &mut [u8], reader: &mut BufReader<File>) {
    if let Some(next_char) = next_char(reader) {
        rotate_buffer(buffer, next_char);
        return;
    }
    panic!("Reached end of the line without finding marker.");
}

pub fn advance_to_next_line(reader: &mut BufReader<File>) -> bool {
    loop {
        if let Some(next_char) = next_char(reader) {
            if next_char == b'\n' {
                return true;
            }
        } else {
            return false; // file is over
        }
    }
}

fn rotate_buffer(buffer: &mut [u8], next_byte: u8) {
    for i in 1..buffer.len() {
        buffer[i - 1] = buffer[i];
    }
    buffer[buffer.len() - 1] = next_byte;
}

fn next_char(reader: &mut BufReader<File>) -> Option<u8> {
    let mut next_byte = [0u8; 1];
    let bytes_read = reader
        .read(&mut next_byte)
        .expect("Could not read next byte");

    return if bytes_read == 0 {
        None
    } else {
        Some(next_byte[0])
    };
}

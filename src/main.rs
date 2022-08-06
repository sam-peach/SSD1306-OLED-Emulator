extern crate minifb;
extern crate simple_slip;

use ggdram::GGDRAM;
use minifb::{Error, Key, Window, WindowOptions};
use serial_line_ip::Decoder;
use serialport::SerialPort;
use simple_slip::decode;
use std::io::{self, BufRead};
use std::time::Duration;
use std::u32;

mod ggdram;

pub const BASE_WIDTH: usize = 128;
pub const BASE_HEIGHT: usize = 64;
pub const SCALE: usize = 1;
pub const PAGE_SIZE: usize = 8 * SCALE;
pub const WIDTH: usize = BASE_WIDTH * SCALE;
pub const HEIGHT: usize = BASE_HEIGHT * SCALE;
pub const WINDOW_NAME: &str = "Display Emulator";

pub const END: u8 = 192;
pub const ESC: u8 = 219;
pub const ESC_END: u8 = 220;
pub const ESC_ESC: u8 = 221;

fn main() -> io::Result<()> {
  let mut ggdram = GGDRAM::new(vec![0; WIDTH * WIDTH]);

  let mut port: Box<dyn SerialPort> = start_serial_port_comm("COM6", 9600)?;

  let mut port_read_buffer: Vec<u8> = vec![0; 64];

  let mut window = Window::new(WINDOW_NAME, WIDTH, HEIGHT, WindowOptions::default())
    .unwrap_or_else(|e| unsafe {
      panic!("{}", e);
    });

  let mut is_data_stream: bool = false;
  let mut first_time = false;

  let mut main_buffer: Vec<u8> = Vec::new();

  window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

  while window.is_open() && !window.is_key_down(Key::Escape) {
    // unsafe { println!("To read: {:?}", port.read_data_set_ready()) }
    match port.read_exact(&mut port_read_buffer) {
      Ok(_) => {
        // unsafe { println!("Received: {:?}", port_read_buffer) }
        decode_frame(&port_read_buffer);
      }
      Err(err) => {
        unsafe {
          println!("{:?}", err);
        }
        continue;
      }
    }

    window
      .update_with_buffer(&ggdram.buffer, BASE_WIDTH, BASE_HEIGHT)
      .unwrap();
  }

  // let file = File::open("input.txt")?;
  // let lines = io::BufReader::new(file);

  // let mut x = 0;
  // let mut y = 0;
  // for data in lines.lines() {
  //   let d = data.unwrap();
  //   let int = u32::from_str_radix(&d, 16).unwrap();

  //   gddram.set_page_column(x, y, int);

  //   if x % (BASE_WIDTH) == 0 {
  //     y += PAGE_SIZE - 1;
  //   }

  //   x += 1;
  // }

  Ok(())
}

fn start_serial_port_comm(
  path: &str,
  baud_rate: u32,
) -> Result<Box<dyn SerialPort>, serialport::Error> {
  return serialport::new(path, baud_rate)
    .timeout(Duration::from_millis(10))
    .open();
}

fn process_serial_byte(serial_buffer: &Vec<u8>, bytes_since_start: u32) {
  let bit: u8 = serial_buffer[0];
}

fn is_data_bit(byte: u8) -> bool {
  return byte == 64;
}

fn decode_frame(encoded_buffer: &[u8]) {
  let decoded = decode(&encoded_buffer).unwrap();

  unsafe { println!("{:?}", decoded) }
  // let mut decodedBuffer: Vec<u8> = vec![0; 64];

  // let mut decoder = Decoder::new();

  // for (i, _) in encoded_buffer.iter().enumerate() {
  //   if encoded_buffer[i] == END {
  //     let (input_bytes_processed, output_slice, is_end_of_packet) = decoder
  //       .decode(&encoded_buffer[i..], &mut decodedBuffer)
  //       .unwrap();

  //     unsafe {
  //       println!("Decoded: {:?}", output_slice);
  //       println!("Processed: {:?}", input_bytes_processed);
  //       println!("Is end: {:?}", is_end_of_packet)
  //     }
  //     break;
  //   }
  // }

  // let mut read_index: usize = 0;
  // let mut write_index: usize = 0;

  // while read_index < 6 {
  //   if encodedBuffer[read_index] == END {
  //     // flush or done
  //     read_index += 1;
  //   } else if encodedBuffer[read_index] == ESC {
  //     if encodedBuffer[read_index + 1] == ESC_END {
  //       decodedBuffer[write_index] = END;
  //       write_index += 1;
  //       read_index += 2;
  //     } else if encodedBuffer[read_index + 1] == ESC_ESC {
  //       decodedBuffer[write_index] = ESC;
  //       write_index += 1;
  //       read_index += 2;
  //     } else {
  //       // This case is considered a protocol violation.
  //     }
  //   } else {
  //     decodedBuffer[write_index] = encodedBuffer[read_index];
  //     write_index += 1;
  //     read_index += 1;
  //   }
  // }
}

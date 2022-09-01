#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use constants::*;
use emulator::Emulator;
use minifb::{Icon, Key, Window, WindowOptions};
use serialport::SerialPort;
use settings::EmulatorSettingsWindow;
use simple_slip::decode_packets;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;
use std::time::Duration;
use utils::MessageData;

mod command_table;
mod constants;
mod emulator;
mod settings;
mod utils;

fn main() {
  let (tx, rx) = mpsc::sync_channel::<MessageData>(READ_BUFF_SIZE);

  EmulatorSettingsWindow::launch(tx.clone());

  let port_name = wait_for_port_name(&rx);

  start_serial_thread(tx, port_name);

  let mut window = build_window();

  let mut emu = Emulator::default();

  while window.is_open() && !window.is_key_down(Key::Escape) {
    let packets = wait_for_packets(&rx);

    emu.process_packets(&packets);

    // Working on it!
    if emu.scrolling {
      emu.scroll();
    }

    window
      .update_with_buffer(&emu.display_buffer, WIDTH as usize, HEIGHT as usize)
      .unwrap();
  }
}

fn start_serial_thread(tx: SyncSender<MessageData>, port_name: String) {
  thread::spawn(move || {
    let mut port: Box<dyn SerialPort> = start_serial_port_comm(&port_name, 9600).unwrap();
    let mut port_read_buffer: Vec<u8> = vec![0; READ_BUFF_SIZE];
    let mut remainder_buffer: Option<Vec<u8>> = None;

    loop {
      match port.read(&mut port_read_buffer) {
        Ok(_) => {
          match remainder_buffer {
            Some(mut rem) => {
              rem.extend(&port_read_buffer);
              port_read_buffer = rem;
            }
            None => (),
          }

          let (packets, remainder): (Vec<Vec<u8>>, Vec<u8>) = decode_packets(&port_read_buffer);

          remainder_buffer = Some(remainder);
          port_read_buffer = vec![0; READ_BUFF_SIZE];

          tx.send(MessageData::Packets(packets)).unwrap();
        }
        Err(_) => match remainder_buffer {
          Some(_) => remainder_buffer = None,
          _ => (),
        },
      }
    }
  });
}

fn start_serial_port_comm(
  path: &str,
  baud_rate: u32,
) -> Result<Box<dyn SerialPort>, serialport::Error> {
  serialport::new(path, baud_rate)
    .timeout(Duration::from_millis(250))
    .open()
}

fn wait_for_port_name(rx: &Receiver<MessageData>) -> String {
  match rx.recv().unwrap() {
    MessageData::Text(port_name) => port_name,
    _ => panic!(),
  }
}

fn wait_for_packets(rx: &Receiver<MessageData>) -> Vec<Vec<u8>> {
  match rx.recv().unwrap() {
    MessageData::Packets(packets) => packets,
    _ => panic!(),
  }
}

fn build_window() -> Window {
  let mut window = Window::new(
    WINDOW_NAME,
    WIDTH as usize,
    HEIGHT as usize,
    WindowOptions {
      scale: minifb::Scale::X2,
      ..Default::default()
    },
  )
  .unwrap();

  window.set_icon(Icon::from_str("src/icon.ico").unwrap());

  window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

  window
}

use crate::emulator::Emulator;

pub enum MessageData {
  Text(String),
  Packets(Vec<Vec<u8>>),
}

pub struct Command {
  pub name: &'static str,
  pub handler: fn(&mut Emulator),
  pub arg_bytes: u8,
}

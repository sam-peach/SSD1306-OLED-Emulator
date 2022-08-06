use serialport;
use serialport::FlowControl;
use std::time::Duration;
fn main() {
  let mut port = serialport::new("COM6", 9600)
    .timeout(Duration::from_millis(10))
    .open()
    .unwrap();

  loop {
    unsafe { println!("{:?}", port.read_clear_to_send()) }
  }
}

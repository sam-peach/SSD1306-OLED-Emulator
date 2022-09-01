use crate::constants::SETTINGS_WINDOW_NAME;
use crate::utils::MessageData;
use eframe::egui;
use eframe::epaint::Vec2;
use serialport::SerialPortInfo;
use std::sync::mpsc::SyncSender;

pub struct EmulatorSettingsWindow {
  available_ports: Vec<SerialPortInfo>,
  port_name: String,
  tx: SyncSender<MessageData>,
}

impl EmulatorSettingsWindow {
  pub fn launch(tx: SyncSender<MessageData>) {
    let mut available_ports = serialport::available_ports().unwrap();
    available_ports.sort_by(|a, b| a.port_name.partial_cmp(&b.port_name).unwrap());

    let window = EmulatorSettingsWindow {
      available_ports: available_ports,
      port_name: String::new(),
      tx: tx,
    };

    eframe::run_native(
      SETTINGS_WINDOW_NAME,
      eframe::NativeOptions {
        initial_window_size: Some(Vec2 { x: 250.0, y: 250.0 }),
        ..Default::default()
      },
      Box::new(|_cc| Box::new(window)),
    );
  }
}

impl eframe::App for EmulatorSettingsWindow {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Pick a port");

      for port in self.available_ports.iter() {
        let name = if port.port_type != serialport::SerialPortType::Unknown {
          format!("{}  >>   Device detected", port.port_name)
        } else {
          port.port_name.clone()
        };

        ui.radio_value(&mut self.port_name, port.port_name.clone(), name);
      }

      if ui.button("Submit").clicked() {
        if self.port_name != "" {
          self
            .tx
            .send(MessageData::Text(self.port_name.clone()))
            .unwrap();
          frame.close();
        }
      };
    });
  }
}

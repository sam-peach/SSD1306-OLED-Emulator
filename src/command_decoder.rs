mod GGDRAM;
use GGDRAM::{MemAddressingMode, ScrollDir};

pub struct CommandDecoder {
  ggdram: GGDRAM,
}

impl CommandDecoder {
  fn set_contrast_control(&self, command: &u8, arg_bytes: &[u8]) {
    self.ggdram.contrast = arg_bytes[0];
  }

  fn entire_display_on(&self, command: &u8, arg_bytes: &[u8]) {}

  fn set_normal(&self, command: &u8, arg_bytes: &[u8]) {}

  fn set_display_on_off(&self, command: &u8, arg_bytes: &[u8]) {
    let mask: u8 = 1;

    if (command & mask) == 0 {
      self.ggdram.sleep = true;
    } else {
      self.ggdram.sleep = false;
    }
  }

  fn cont_horiz_scroll_setup(&self, command: &u8, arg_bytes: &[u8]) {
    let scroll_mask: u8 = 1;
    let mask = 0b111;

    self.gddram.scroll_dir = if (scroll_mask & command) == 0 {
      ScrollDir::HorizRight
    } else {
      ScrollDir::HorizLeft
    };
    self.set_scroll_pages(&arg_bytes);
  }

  fn cont_vert_horiz_scroll_setup(&self, command: &u8, arg_bytes: &[u8]) {
    let scroll_mask: u8 = 0b11;

    self.gddram.scroll_dir = if (scroll_mask & command) == 1 {
      ScrollDir::VertRight
    } else {
      ScrollDir::VertLeft
    };
    self.set_scroll_pages(&arg_bytes);

    self.ggdram.vert_scorolling_offset = arg_bytes;
  }

  fn deactivate_scroll(&self, command: u8, arg_bytes: &[u8]) {
    self.gddram.scroll = true;
  }

  fn activate_scroll(&self, command: u8, arg_bytes: &[u8]) {
    self.gddram.scroll = false;
  }

  fn set_vertical_scroll_area(&self, command: u8, arg_bytes: &[u8]) {}

  fn set_lower_col_start_page(&self, command: u8, arg_bytes: &[u8]) {
    self.gddram.lower_col_start = command & 0b1111;
  }

  fn set_higher_col_start_page(&self, command: u8, arg_bytes: &[u8]) {
    self.gddram.higher_col_start = command & 0b1111;
  }

  fn set_mem_address_mode(&self, command: u8, arg_bytes: &[u8]) {
    let addr_mode = match arg_bytes[0] & 0b11 {
      0 => MemAddressingMode::Horizontal,
      1 => MemAddressingMode::Vertical,
      2 => MemAddressingMode::Page,
    };

    self.gddram.mem_addressing_mode = addr_mode;
  }

  fn set_col_address(&self, command: u8, arg_bytes: &[u8]) {
    self.ggdram.column_start_addr = arg_bytes[0] & 0b1111111;
    self.ggdram.column_end_addr = arg_bytes[1] & 0b1111111;
  }

  fn set_page_address(&self, command: u8, arg_bytes: &[u8]) {
    self.ggdram.page_start_addr = arg_bytes[0] & 0b111;
    self.ggdram.page_end_addr = arg_bytes[1] & 0b111;
  }

  fn set_page_start_address_page(&self, command: u8, arg_bytes: &[u8]) {
    self.ggdram.page_start_addr = command & 0b111;
  }

  fn set_display_start_line(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_segment_remap(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_multiplex_ration(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_com_output_scan_dir(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_display_offset(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_com_pins_config(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_display_clock_ratio(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_precharge_period(&self, command: u8, arg_bytes: &[u8]) {}
  fn set_charge_pump(&self, command: u8, arg_bytes: &[u8]) {}
  fn no_op(&self, command: u8, arg_bytes: &[u8]) {}
  fn illegal_command(&self, command: u8, arg_bytes: &[u8]) {}

  fn set_scroll_pages(&self, arg_bytes: &[u8]) {
    // ** arg_bytes[0]; Dummy byte **
    self.ggdram.start_page = arg_bytes[1] & mask;
    self.ggdram.scroll_time_interval = arg_bytes[2] & mask;
    self.ggdram.end_page_address = arg_bytes[3] & mask;
  }
}

struct Command {
  Fn: function,
  u8: arg_bytes,
}

struct Instruction {
  register: String,
  value: u8,
}

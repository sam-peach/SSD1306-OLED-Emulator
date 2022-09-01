use crate::command_table;
use crate::constants::{DATA_MASK, HEIGHT, MAX_CONTRAST, PAGE_SIZE, WIDTH};
use crate::utils::Command;

#[derive(PartialEq)]
enum ScrollDir {
  HorizLeft,
  HorizRight,
  VertLeft,
  VertRight,
}

#[derive(PartialEq, Debug)]
enum MemAddressingMode {
  Horizontal,
  Vertical,
  Page,
}

pub struct Emulator {
  pub display_buffer: Vec<u32>,
  pub scrolling: bool,

  current_command: &'static Command,
  command_byte: u8,
  command_buffer: Vec<u8>,
  width: u32,
  height: u32,
  page_ptr: u8,
  column_ptr: u8,
  column_start_addr: u8,
  column_end_addr: u8,
  contrast: u8,
  sleep: bool,
  scroll_dir: ScrollDir,
  scroll_start_page: u8,
  scroll_end_page: u8,
  scroll_time_interval: u8,
  end_page_address: u8,
  vert_scrolling_offset: u8,
  vert_scroll_area: u8,
  lower_col_start: u8,
  higher_col_start: u8,
  mem_addressing_mode: MemAddressingMode,
  page_start_addr: u8,
  page_end_ptr: u8,
}

impl Default for Emulator {
  fn default() -> Self {
    Emulator {
      display_buffer: vec![0; (WIDTH * HEIGHT) as usize],
      command_buffer: Vec::new(),
      command_byte: 0,
      current_command: &Command {
        name: "no op",
        handler: Emulator::no_op,
        arg_bytes: 0,
      },
      width: WIDTH,
      height: HEIGHT,
      page_ptr: 0,
      column_ptr: 0,
      column_start_addr: 0,
      column_end_addr: (WIDTH - 1) as u8,
      contrast: 255,
      sleep: false,
      scroll_dir: ScrollDir::HorizRight,
      scroll_start_page: (3 * PAGE_SIZE) as u8,
      scroll_end_page: (8 * PAGE_SIZE - 1) as u8,
      scroll_time_interval: 0,
      end_page_address: 100,
      vert_scrolling_offset: 0,
      scrolling: false,
      vert_scroll_area: 0,
      lower_col_start: 0,
      higher_col_start: 0,
      mem_addressing_mode: MemAddressingMode::Horizontal,
      page_start_addr: 0,
      page_end_ptr: (7 * PAGE_SIZE) as u8,
    }
  }
}

impl Emulator {
  fn command_is_data(byte: u8) -> bool {
    byte & DATA_MASK > 0
  }

  pub fn process_packets(&mut self, packets: &Vec<Vec<u8>>) {
    for packet in packets {
      self.process_packet(packet);
    }
  }

  fn process_packet(&mut self, packet: &Vec<u8>) {
    let instruction_byte = packet[0];
    let data_buffer = &packet[1..];

    if Emulator::command_is_data(instruction_byte) {
      self.add_to_display_buffer(data_buffer);
    } else {
      for idx in 0..data_buffer.len() {
        if self.waiting_on_command_args() {
          self.add_to_command_buffer(data_buffer[idx]);
        } else {
          self.process_command();
          let command_byte = data_buffer[idx];
          let command = command_table::lookup(command_byte);
          self.set_current_command(command_byte, command);
        }
      }
    }
  }

  pub fn add_to_display_buffer(&mut self, buffer: &[u8]) {
    for latest_byte in buffer {
      let byte_str = format!("{:08b}", latest_byte);

      for c in byte_str.chars().rev() {
        let (x, y) = match self.mem_addressing_mode {
          MemAddressingMode::Vertical => (self.page_ptr, self.column_ptr),
          _ => (self.column_ptr, self.page_ptr),
        };

        let grid_idx = self.x_y_to_index(x as usize, y as usize);

        self.display_buffer[grid_idx] = if c == '0' {
          0
        } else {
          MAX_CONTRAST - (MAX_CONTRAST - self.contrast as u32)
            | (MAX_CONTRAST - (MAX_CONTRAST - self.contrast as u32) << 8)
        };

        self.page_ptr += 1;
      }

      self.next_ram_page();
    }
  }

  pub fn add_to_command_buffer(&mut self, byte: u8) {
    self.command_buffer.push(byte);
  }

  pub fn set_current_command(&mut self, command_byte: u8, command: &'static Command) {
    self.command_byte = command_byte;
    self.current_command = command;
    self.command_buffer = Vec::new();
  }

  pub fn process_command(&mut self) {
    let handler = self.current_command.handler;
    handler(self);
  }

  pub fn waiting_on_command_args(&self) -> bool {
    self.command_buffer.len() < self.current_command.arg_bytes as usize
  }

  pub fn scroll(&mut self) {
    match self.scroll_dir {
      ScrollDir::HorizRight => self.apply_horiz_scroll(ScrollDir::HorizRight),
      ScrollDir::HorizLeft => self.apply_horiz_scroll(ScrollDir::HorizLeft),
      ScrollDir::VertRight => self.apply_diagonal_scroll(ScrollDir::VertRight),
      ScrollDir::VertLeft => self.apply_diagonal_scroll(ScrollDir::VertLeft),
    }
  }

  fn apply_horiz_scroll(&mut self, dir: ScrollDir) {
    // Working on it!
    let u_width = self.width as usize;
    let (start, end) = (
      self.x_y_to_index(0, (self.scroll_start_page) as usize),
      self.x_y_to_index(u_width - 1, (self.scroll_end_page) as usize),
    );

    let mut idx = match dir {
      ScrollDir::HorizLeft => start,
      _ => end,
    };

    let condition = |dir: &ScrollDir, idx: &usize| -> bool {
      match dir {
        ScrollDir::HorizLeft => *idx < end,
        _ => *idx > start,
      }
    };

    let next_idx = |dir: &ScrollDir, idx: usize| -> usize {
      match dir {
        ScrollDir::HorizLeft => idx + 1,
        _ => idx - 1,
      }
    };

    while condition(&dir, &idx) {
      if idx % u_width == 0 {
        idx = next_idx(&dir, idx);
        continue;
      } else if (idx + 1) % u_width == 0 {
        let new_idx = idx - (u_width - 1);

        let temp_byte = self.display_buffer[new_idx];

        self.display_buffer[new_idx] = self.display_buffer[idx];
        self.display_buffer[idx] = temp_byte;
      } else {
        let new_idx = idx + 1;

        let temp_byte = self.display_buffer[new_idx];

        self.display_buffer[new_idx] = self.display_buffer[idx];
        self.display_buffer[idx] = temp_byte;
      }

      idx = next_idx(&dir, idx);
    }

    self.column_ptr = if self.column_ptr == 0 {
      self.column_end_addr
    } else {
      self.column_ptr - 1
    };
  }

  fn apply_diagonal_scroll(&mut self, dir: ScrollDir) {
    // Working on it!
    if dir == ScrollDir::VertRight {
      self.apply_horiz_scroll(ScrollDir::HorizRight);

      let offset = ((self.vert_scrolling_offset * self.column_end_addr) + 1) as usize;

      let first_lines = &self.display_buffer[0..offset];
      let mut rest = (&self.display_buffer[offset..]).to_vec();
      rest.extend_from_slice(first_lines);

      self.display_buffer = rest;
    } else if dir == ScrollDir::VertLeft {
      self.apply_horiz_scroll(ScrollDir::HorizLeft);

      let offset = ((self.vert_scrolling_offset * self.column_end_addr) + 1) as usize;

      let first_lines = &self.display_buffer[0..offset];
      let mut rest = (&self.display_buffer[offset..]).to_vec();
      rest.extend_from_slice(first_lines);

      self.display_buffer = rest;
    }
  }

  fn next_ram_page(&mut self) {
    match self.mem_addressing_mode {
      MemAddressingMode::Page => {
        self.increment_col_ptr();
      }
      MemAddressingMode::Horizontal => {
        self.next_horiz_ram_page();
      }
      MemAddressingMode::Vertical => {
        self.next_vert_ram_page();
      }
    };
  }

  fn next_horiz_ram_page(&mut self) {
    if self.column_ptr > self.column_end_addr - 1 {
      self.column_ptr = 0;

      if self.page_ptr >= (self.height - 1) as u8 {
        self.page_ptr = 0;
      }
    } else {
      self.column_ptr += 1;
      self.page_ptr -= PAGE_SIZE as u8;
    }
  }

  fn next_vert_ram_page(&mut self) {
    if self.page_ptr >= (self.height - 1) as u8 {
      self.page_ptr = 0;

      if self.column_ptr >= self.column_end_addr - 1 {
        self.column_ptr = 0;
      };
    } else {
      self.page_ptr += 1;
      self.page_ptr -= PAGE_SIZE as u8;
    }
  }

  fn increment_col_ptr(&mut self) {
    self.column_ptr = if self.column_ptr + 1 > self.column_end_addr {
      self.column_start_addr
    } else {
      self.column_ptr + 1
    }
  }

  fn x_y_to_index(&self, x: usize, y: usize) -> usize {
    return x + self.width as usize * y;
  }

  pub fn set_contrast_control(&mut self) {
    self.contrast = self.command_buffer[0];
  }

  pub fn set_display_on_off(&mut self) {
    let mask: u8 = 0b00000001;

    match self.command_byte & mask {
      0 => self.sleep = true,
      1 => self.sleep = false,
      _ => (),
    }
  }

  pub fn cont_horiz_scroll_setup(&mut self) {
    // ** arg_bytes[0]; Dummy byte **

    let scroll_mask: u8 = 0b00000001;

    self.scroll_dir = match self.command_byte & scroll_mask {
      0 => ScrollDir::HorizRight,
      1 => ScrollDir::HorizLeft,
      _ => ScrollDir::HorizRight,
    };

    self.set_scroll_pages();
  }

  pub fn cont_vert_horiz_scroll_setup(&mut self) {
    let scroll_mask: u8 = 0b11;

    self.scroll_dir = if (scroll_mask & self.command_byte) == 1 {
      ScrollDir::VertRight
    } else {
      ScrollDir::VertLeft
    };
    self.set_scroll_pages();

    self.vert_scrolling_offset = self.command_buffer[4];
  }

  pub fn deactivate_scroll(&mut self) {
    self.scrolling = false;
  }

  pub fn activate_scroll(&mut self) {
    self.scrolling = true;
  }

  pub fn set_lower_col_start_page(&mut self) {
    let lower = self.command_byte & 0b00001111;
    self.column_ptr = self.column_ptr | lower;
  }

  pub fn set_higher_col_start_page(&mut self) {
    let upper = self.command_byte & 0b11110000;
    self.column_ptr = self.column_ptr | upper;
  }

  pub fn set_mem_address_mode(&mut self) {
    let mask = 0b11;
    let addr_mode = match self.command_buffer[0] & mask {
      0 => MemAddressingMode::Horizontal,
      1 => MemAddressingMode::Vertical,
      2 => MemAddressingMode::Page,
      _ => MemAddressingMode::Horizontal,
    };

    self.mem_addressing_mode = addr_mode;
  }

  pub fn set_col_address(&mut self) {
    self.column_start_addr = self.command_buffer[0];
    self.column_end_addr = self.command_buffer[1];
    self.column_ptr = self.column_start_addr;
  }

  pub fn set_page_address(&mut self) {
    self.page_ptr = (self.command_buffer[0] & 0b00000111) * PAGE_SIZE as u8;
    self.page_end_ptr = (self.command_buffer[1] & 0b00000111) * PAGE_SIZE as u8;
  }

  pub fn set_page_start_address_page(&mut self) {
    self.page_ptr = (self.command_byte & 0b111) * PAGE_SIZE as u8;
  }

  pub fn set_display_start_line(&mut self) {
    //TODO
    let _mask = 0b00111111;

    self.page_ptr = 0;
    self.column_ptr = 0;
  }

  pub fn set_scroll_pages(&mut self) {
    // ** arg_bytes[0]; Dummy byte **
    let mask = 0b00000111;
    self.scroll_start_page = (self.command_buffer[1] & mask) * PAGE_SIZE as u8;
    self.scroll_time_interval = self.command_buffer[2] & mask;
    self.scroll_end_page =
      ((self.command_buffer[3] & mask) * PAGE_SIZE as u8) + PAGE_SIZE as u8 - 1;
  }

  // TODO!
  pub fn set_segment_remap(&mut self) {}
  pub fn set_multiplex_ration(&mut self) {}
  pub fn set_com_output_scan_dir(&mut self) {}
  pub fn set_display_offset(&mut self) {}
  pub fn set_com_pins_config(&mut self) {}
  pub fn set_display_clock_ratio(&mut self) {}
  pub fn set_precharge_period(&mut self) {}
  pub fn set_charge_pump(&mut self) {}
  pub fn no_op(&mut self) {}
  pub fn illegal_command(&mut self) {}
  pub fn set_vcom_level(&mut self) {}
  pub fn entire_display_on(&mut self) {}
  pub fn set_normal(&mut self) {}
  pub fn set_vertical_scroll_area(&mut self) {}
}

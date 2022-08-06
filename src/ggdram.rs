const BLUE: u32 = 255 | (255 << 8);

use crate::PAGE_SIZE;
use crate::SCALE;

enum ScrollDir {
  HorizLeft,
  HorizRight,
  VertLeft,
  VertRight,
}

enum MemAddressingMode {
  Horizontal,
  Vertical,
  Page,
}

pub struct GGDRAM {
  pub buffer: Vec<u32>,
  page_pointer: u8,
  column_pointer: u8, // pub contrast: u8,
                      // pub sleep: bool,
                      // pub scroll_dir: ScrollDir,
                      // pub start_page: u8,
                      // pub scroll_time_interval: u8,
                      // pub end_page_address: u8,
                      // pub vert_scrolling_offset: u8,
                      // pub scroll: bool,
                      // pub vert_scroll_area: u8,
                      // pub lower_col_start: u8,
                      // pub higher_col_start: u8,
                      // pub mem_addressing_mode: MemAddressingMode,
                      // pub column_start_addr: u8,
                      // pub column_end_addr: u8,
                      // pub page_start_addr: u8,
                      // pub page_end_addr: u8
}

impl GGDRAM {
  pub fn new(buffer: Vec<u32>) -> Self {
    return GGDRAM {
      buffer: buffer,
      page_pointer: 0,
      column_pointer: 0,
    };
  }

  pub fn set_page_column(&mut self, x: usize, y: usize, val: u32) {
    for offset in 0..PAGE_SIZE {
      let color = if (val & (1 << (offset / SCALE))) != 0 {
        BLUE
      } else {
        0
      };
      let address = GGDRAM::x_y_to_index(x, y + offset);
      self.buffer[address] = color;
    }
  }

  fn x_y_to_index(x: usize, y: usize) -> usize {
    return x + crate::WIDTH * y;
  }

  pub fn add_to_buffer(&mut self, buffer: &Vec<u8>) {
    let mut byte_str = "".to_string();

    for latest_byte in buffer {
      unsafe {
        byte_str = format!("{:08b}", latest_byte);
      }

      for c in byte_str.chars().rev() {
        let idx = GGDRAM::x_y_to_index(self.column_pointer as usize, self.page_pointer as usize);

        self.buffer[idx] = if c == '0' { 0 } else { BLUE };

        self.page_pointer += 1;
      }

      if self.column_pointer >= 128 {
        self.column_pointer = 0;

        if self.page_pointer >= 64 {
          self.page_pointer = 0;
        };
      } else {
        self.column_pointer += 1;
        self.page_pointer -= 8;
      }
    }
  }

  // pub fn add_to_buffer(&mut self, latest_byte: u8) {
  //   let mut byte_str = "".to_string();
  //   unsafe {
  //     byte_str = format!("{:08b}", latest_byte);
  //   }

  //   for c in byte_str.chars().rev() {
  //     let idx = GGDRAM::x_y_to_index(self.column_pointer as usize, self.page_pointer as usize);

  //     self.buffer[idx] = if c == '0' { 0 } else { BLUE };

  //     self.page_pointer += 1;
  //   }

  //   if self.column_pointer >= 128 {
  //     self.column_pointer = 0;

  //     if self.page_pointer >= 64 {
  //       self.page_pointer = 0;
  //     };
  //   } else {
  //     self.column_pointer += 1;
  //     self.page_pointer -= 8;
  //   }
  // }
}

use crate::emulator::Emulator;
use crate::utils::Command;

pub fn lookup(byte: u8) -> &'static Command {
  let upper_byte = (byte & 0b11110000) >> 4;
  let lower_byte = byte & 0b00001111;

  &COMMAND_TABLE[upper_byte as usize][lower_byte as usize]
}

const COMMAND_TABLE: [[Command; 16]; 16] = [
  [
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_lower_col_start_page",
      handler: Emulator::set_lower_col_start_page,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_higher_col_start_page",
      handler: Emulator::set_higher_col_start_page,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_mem_address_mode",
      handler: Emulator::set_mem_address_mode,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::set_col_address",
      handler: Emulator::set_col_address,
      arg_bytes: 2,
    },
    Command {
      name: "Emulator::set_page_address",
      handler: Emulator::set_page_address,
      arg_bytes: 2,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::cont_horiz_scroll_setup",
      handler: Emulator::cont_horiz_scroll_setup,
      arg_bytes: 6,
    },
    Command {
      name: "Emulator::cont_horiz_scroll_setup",
      handler: Emulator::cont_horiz_scroll_setup,
      arg_bytes: 6,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::cont_vert_horiz_scroll_setup",
      handler: Emulator::cont_vert_horiz_scroll_setup,
      arg_bytes: 5,
    },
    Command {
      name: "Emulator::cont_vert_horiz_scroll_setup",
      handler: Emulator::cont_vert_horiz_scroll_setup,
      arg_bytes: 5,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::deactivate_scroll",
      handler: Emulator::deactivate_scroll,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::activate_scroll",
      handler: Emulator::activate_scroll,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_start_line",
      handler: Emulator::set_display_start_line,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_contrast_control",
      handler: Emulator::set_contrast_control,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_charge_pump",
      handler: Emulator::set_charge_pump,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_segment_remap",
      handler: Emulator::set_segment_remap,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_segment_remap",
      handler: Emulator::set_segment_remap,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_vertical_scroll_area",
      handler: Emulator::set_vertical_scroll_area,
      arg_bytes: 2,
    },
    Command {
      name: "Emulator::entire_display_on",
      handler: Emulator::entire_display_on,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::entire_display_on",
      handler: Emulator::entire_display_on,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_normal",
      handler: Emulator::set_normal,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_normal",
      handler: Emulator::set_normal,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_multiplex_ration",
      handler: Emulator::set_multiplex_ration,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_on_off",
      handler: Emulator::set_display_on_off,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_on_off",
      handler: Emulator::set_display_on_off,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_page_start_address_page",
      handler: Emulator::set_page_start_address_page,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::set_com_output_scan_dir",
      handler: Emulator::set_com_output_scan_dir,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_com_output_scan_dir",
      handler: Emulator::set_com_output_scan_dir,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_offset",
      handler: Emulator::set_display_offset,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_display_clock_ratio",
      handler: Emulator::set_display_clock_ratio,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::set_precharge_period",
      handler: Emulator::set_precharge_period,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::set_com_pins_config",
      handler: Emulator::set_com_pins_config,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::set_vcom_level",
      handler: Emulator::set_vcom_level,
      arg_bytes: 1,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::no_op",
      handler: Emulator::no_op,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
  [
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
    Command {
      name: "Emulator::illegal_command",
      handler: Emulator::illegal_command,
      arg_bytes: 0,
    },
  ],
];

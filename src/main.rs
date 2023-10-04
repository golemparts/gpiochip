#![allow(dead_code)]

use std::error::Error;
use std::os::unix::io::AsRawFd;

#[macro_use]
mod macros;

mod gpio;
mod ioctl;

use crate::ioctl::*;

fn main() -> Result<(), Box<dyn Error>> {
    let gpiochip = find_gpiochip()?;
    let chip_info = ChipInfo::new(gpiochip.as_raw_fd())?;

    println!(
        "ChipInfo: {:?} {:?} {}",
        cbuf_to_cstring(&chip_info.name),
        cbuf_to_cstring(&chip_info.label),
        chip_info.lines
    );

    for line in 0..=chip_info.lines.min(MAX_OFFSET) {
        if let Ok(line_info) = LineInfo::new(gpiochip.as_raw_fd(), line) {
            // /            if let Ok(line_request) = LineRequest::new(gpiochip.as_raw_fd(), line) {
            //                 let line_values = line_request.levels()?;

            //                 println!(
            //                     "{}: {} Flags:[{}] {} Level:[{}]",
            //                     line_info.offset,
            //                     cbuf_to_string(&line_info.name),
            //                     line_info.flags(),
            //                     line_info.flags,
            //                     if (line_values.bits & 0x01) > 0 {
            //                         "High"
            //                     } else {
            //                         "Low"
            //                     },
            //                 );
            //             } else {
            println!(
                "{}: {} Consumer:[{}] Flags:[{}]",
                line_info.offset,
                cbuf_to_string(&line_info.name),
                cbuf_to_string(&line_info.consumer),
                line_info.flags(),
            );
            // }
        }
    }

    Ok(())
}

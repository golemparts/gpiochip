#![allow(dead_code)]

use std::error::Error;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

#[macro_use]
mod macros;

mod gpio;
mod ioctl;

use crate::ioctl::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Detect any gpiochip devices, and display their details
    for id in 0..=255 {
        if let Ok(file) = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("{}{}", PATH_GPIOCHIP, id))
        {
            let chip_info = ChipInfo::new(file.as_raw_fd())?;
            println!("{:?}", chip_info);
        };
    }

    Ok(())
}

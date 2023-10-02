#![allow(clippy::unnecessary_cast)]
#![allow(dead_code)]

use crate::gpio::{Error, Result};
use libc::{self, c_int, c_ulong, ENOENT};
use std::ffi::CString;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io;
use std::mem;
use std::os::unix::io::AsRawFd;

#[cfg(target_env = "gnu")]
type IoctlLong = libc::c_ulong;
#[cfg(target_env = "musl")]
type IoctlLong = c_int;

pub const PATH_GPIOCHIP: &str = "/dev/gpiochip";
const CONSUMER_LABEL: &str = "RPPAL";
const DRIVER_NAME: &[u8] = b"pinctrl-bcm2835\0";
const DRIVER_NAME_BCM2711: &[u8] = b"pinctrl-bcm2711\0";
const DRIVER_NAME_BCM2712: &[u8] = b"pinctrl-rp1\0";

const NRBITS: u8 = 8;
const TYPEBITS: u8 = 8;
const SIZEBITS: u8 = 14;
const DIRBITS: u8 = 2;
const NRSHIFT: u8 = 0;
const TYPESHIFT: u8 = NRSHIFT + NRBITS;
const SIZESHIFT: u8 = TYPESHIFT + TYPEBITS;
const DIRSHIFT: u8 = SIZESHIFT + SIZEBITS;

const NR_GET_CHIP_INFO: IoctlLong = 0x01 << NRSHIFT;
const NR_GET_LINE_INFO: IoctlLong = 0x05 << NRSHIFT;
const NR_GET_LINE_INFO_WATCH: IoctlLong = 0x06 << NRSHIFT;
const NR_GET_LINE_INFO_UNWATCH: IoctlLong = 0x0C << NRSHIFT;
const NR_GET_LINE: IoctlLong = 0x07 << NRSHIFT;
const NR_LINE_SET_CONFIG: IoctlLong = 0x0D << NRSHIFT;
const NR_LINE_GET_VALUES: IoctlLong = 0x0E << NRSHIFT;
const NR_LINE_SET_VALUES: IoctlLong = 0x0F << NRSHIFT;

const TYPE_GPIO: IoctlLong = (0xB4 as IoctlLong) << TYPESHIFT;

const SIZE_CHIP_INFO: IoctlLong = (mem::size_of::<ChipInfo>() as IoctlLong) << SIZESHIFT;

const DIR_NONE: c_ulong = 0;
const DIR_WRITE: IoctlLong = 1 << DIRSHIFT;
const DIR_READ: IoctlLong = 2 << DIRSHIFT;
const DIR_READ_WRITE: IoctlLong = DIR_READ | DIR_WRITE;

const REQ_GET_CHIP_INFO: IoctlLong = DIR_READ | TYPE_GPIO | NR_GET_CHIP_INFO | SIZE_CHIP_INFO;

const NAME_BUFSIZE: usize = 32;
const LABEL_BUFSIZE: usize = 32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChipInfo {
    pub name: [u8; NAME_BUFSIZE],
    pub label: [u8; LABEL_BUFSIZE],
    pub lines: u32,
}

impl ChipInfo {
    pub fn new(cdev_fd: c_int) -> Result<ChipInfo> {
        let mut chip_info = ChipInfo {
            name: [0u8; NAME_BUFSIZE],
            label: [0u8; LABEL_BUFSIZE],
            lines: 0,
        };

        parse_retval!(unsafe { libc::ioctl(cdev_fd, REQ_GET_CHIP_INFO, &mut chip_info) })?;

        Ok(chip_info)
    }
}

impl fmt::Debug for ChipInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChipInfo")
            .field("name", &cbuf_to_cstring(&self.name))
            .field("label", &cbuf_to_cstring(&self.label))
            .field("lines", &self.lines)
            .finish()
    }
}

// Find the correct gpiochip device based on its label
pub fn find_gpiochip() -> Result<File> {
    for id in 0..=255 {
        let gpiochip = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("{}{}", PATH_GPIOCHIP, id))
        {
            Ok(file) => file,
            Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                return Err(Error::PermissionDenied(format!("{}{}", PATH_GPIOCHIP, id)));
            }
            Err(e) => return Err(Error::from(e)),
        };

        let chip_info = ChipInfo::new(gpiochip.as_raw_fd())?;
        if chip_info.label[0..DRIVER_NAME.len()] == DRIVER_NAME[..]
            || chip_info.label[0..DRIVER_NAME_BCM2711.len()] == DRIVER_NAME_BCM2711[..]
            || chip_info.label[0..DRIVER_NAME_BCM2712.len()] == DRIVER_NAME_BCM2712[..]
        {
            return Ok(gpiochip);
        }
    }

    // File Not Found I/O error
    Err(Error::Io(io::Error::from_raw_os_error(ENOENT)))
}

// Create a CString from a C-style NUL-terminated char array. This workaround
// is needed for fixed-length buffers that fill the remaining bytes with NULs,
// because CString::new() interprets those as a NUL in the middle of the byte
// slice and returns a NulError.
fn cbuf_to_cstring(buf: &[u8]) -> CString {
    CString::new({
        let pos = buf.iter().position(|&c| c == b'\0').unwrap_or(buf.len());
        &buf[..pos]
    })
    .unwrap_or_default()
}

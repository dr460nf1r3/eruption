/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
*/

// use std::fs::File;
// use std::io::prelude::*;
use evdev_rs::enums::EV_KEY;
use failure::Fail;
use std::fs;
use std::path::{Path, PathBuf};
use udev::Enumerator;

use crate::rvdevice;

pub type Result<T> = std::result::Result<T, UtilError>;

#[derive(Debug, Fail)]
pub enum UtilError {
    #[fail(display = "No compatible devices found")]
    NoDevicesFound {},

    #[fail(display = "Error occurred during device enumeration")]
    EnumerationError {},

    #[fail(display = "Could not enumerate udev devices")]
    UdevError {},
}

/// Get the path of the evdev device of the first keyboard from udev
pub fn get_evdev_from_udev() -> Result<String> {
    match Enumerator::new() {
        Ok(mut enumerator) => {
            enumerator.match_subsystem("input").unwrap();

            match enumerator.scan_devices() {
                Ok(devices) => {
                    for device in devices {
                        let found_dev = device
                            .properties()
                            .any(|e| e.name() == "ID_VENDOR" && e.value() == rvdevice::VENDOR_STR)
                            && device.devnode().is_some();

                        if found_dev {
                            return Ok(device.devnode().unwrap().to_str().unwrap().to_string());
                        }
                    }

                    Err(UtilError::NoDevicesFound {})
                }

                Err(_e) => Err(UtilError::EnumerationError {}),
            }
        }

        Err(_e) => Err(UtilError::UdevError {}),
    }
}

// pub fn get_evdev_from_proc() -> Result<String> {
//     let mut file = File::open("/proc/bus/input/devices")?;

//     let mut list = String::new();
//     file.read_to_string(&mut list)?;

//     let list: Vec<&str> = list.split('\n').collect();

//     for (index, stanza) in list.iter().enumerate() {
//         if stanza.starts_with("I:") {
//             let line = &stanza[12..];
//             let pat = format!(
//                 "Vendor={:x} Product={:x}",
//                 crate::rvdevice::VENDOR_ID,
//                 crate::rvdevice::PRODUCT_ID
//             );

//             if line.starts_with(&pat) {
//                 let path = list[index + 2].split('=').collect::<Vec<&str>>()[1];
//                 let result = format!("/dev/input/by-path/{}", path).to_string();

//                 info!("{}", result);

//                 return Ok(result);
//             }
//         }
//     }

//     Err(Error::from(ErrorKind::NotFound))
// }

// pub fn set_process_priority() -> std::result::Result<(), i32> {
//     Ok(())
// }

/// Returns the associated manifest path in `PathBuf` for the script `script_path`.
pub fn get_manifest_for(script_file: &Path) -> PathBuf {
    let mut manifest_path = script_file.to_path_buf();
    manifest_path.set_extension("lua.manifest");

    manifest_path
}

pub fn is_file_accessible<P: AsRef<Path>>(p: P) -> std::io::Result<String> {
    fs::read_to_string(p)
}

/// Checks whether a script file is readable
#[allow(dead_code)]
pub fn is_script_file_accessible(script_file: &Path) -> bool {
    is_file_accessible(script_file).is_ok()
}

/// Checks whether a script's manifest file is readable
#[allow(dead_code)]
pub fn is_manifest_file_accessible(script_file: &Path) -> bool {
    fs::read_to_string(get_manifest_for(script_file)).is_ok()
}

/// Map evdev event codes to key indices, for ISO variant
static EV_TO_INDEX_ISO: [u8; 0x2ff + 1] = [
    0xff, 0x00, 0x06, 0x0c, 0x12, 0x18, 0x1d, 0x21, 0x31, 0x36, 0x3c, 0x42, 0x48, 0x4f, 0x57,
    0x02, // 0x000
    0x07, 0x0d, 0x13, 0x19, 0x1e, 0x22, 0x32, 0x37, 0x3d, 0x43, 0x49, 0x50, 0x58, 0x05, 0x08,
    0x0e, // 0x010
    0x14, 0x1a, 0x1f, 0x23, 0x33, 0x38, 0x3e, 0x44, 0x4a, 0x01, 0x04, 0x60, 0x0f, 0x15, 0x1b,
    0x20, // 0x020
    0x24, 0x34, 0x39, 0x3f, 0x45, 0x4b, 0x52, 0x7c, 0x10, 0x25, 0x03, 0x0b, 0x11, 0x17, 0x1c,
    0x30, // 0x030
    0x35, 0x3b, 0x41, 0x4e, 0x54, 0x71, 0x67, 0x72, 0x78, 0x7d, 0x81, 0x73, 0x79, 0x7e, 0x82,
    0x74, // 0x040
    0x7a, 0x7f, 0x75, 0x80, 0xff, 0xff, 0x09, 0x55, 0x56, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x050
    0x83, 0x59, 0x77, 0x63, 0x46, 0xff, 0x68, 0x6a, 0x6d, 0x66, 0x6f, 0x69, 0x6b, 0x6e, 0x64,
    0x65, // 0x060
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x6c, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0a, 0xff,
    0x53, // 0x070
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x080
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x090
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0a0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0b0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0c0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0d0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0e0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0f0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x100
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x110
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x120
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x130
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x140
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x150
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x160
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x170
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x180
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x190
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1a0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1b0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1c0
    0x4c, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1d0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1e0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1f0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x200
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x210
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x220
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x230
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x240
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x250
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x260
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x270
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x280
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x290
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2a0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2b0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2c0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2d0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2e0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2f0
];

/// Map evdev event codes to key indices, for ANSI variant
static _EV_TO_INDEX_ANSI: [u8; 0x2ff + 1] = [
    0xff, 0x00, 0x06, 0x0c, 0x12, 0x18, 0x1d, 0x21, 0x31, 0x36, 0x3c, 0x42, 0x48, 0x4f, 0x57,
    0x02, // 0x000
    0x07, 0x0d, 0x13, 0x19, 0x1e, 0x22, 0x32, 0x37, 0x3d, 0x43, 0x49, 0x50, 0x58, 0x05, 0x08,
    0x0e, // 0x010
    0x14, 0x1a, 0x1f, 0x23, 0x33, 0x38, 0x3e, 0x44, 0x4a, 0x01, 0x04, 0x51, 0x0f, 0x15, 0x1b,
    0x20, // 0x020
    0x24, 0x34, 0x39, 0x3f, 0x45, 0x4b, 0x52, 0x7c, 0x10, 0x25, 0x03, 0x0b, 0x11, 0x17, 0x1c,
    0x30, // 0x030
    0x35, 0x3b, 0x41, 0x4e, 0x54, 0x71, 0x67, 0x72, 0x78, 0x7d, 0x81, 0x73, 0x79, 0x7e, 0x82,
    0x74, // 0x040
    0x7a, 0x7f, 0x75, 0x80, 0xff, 0xff, 0xff, 0x55, 0x56, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x050
    0x83, 0x59, 0x77, 0x63, 0x46, 0xff, 0x68, 0x6a, 0x6d, 0x66, 0x6f, 0x69, 0x6b, 0x6e, 0x64,
    0x65, // 0x060
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x6c, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0a, 0xff,
    0x53, // 0x070
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x080
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x090
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0a0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0b0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0c0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0d0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0e0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x0f0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x100
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x110
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x120
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x130
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x140
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x150
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x160
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x170
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x180
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x190
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1a0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1b0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1c0
    0x4c, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1d0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1e0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x1f0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x200
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x210
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x220
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x230
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x240
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x250
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x260
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x270
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x280
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x290
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2a0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2b0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2c0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2d0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2e0
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, // 0x2f0
];

pub fn ev_key_to_key_index(key: EV_KEY) -> u8 {
    EV_TO_INDEX_ISO[((key as u8) as usize)] + 1
}

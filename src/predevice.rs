// SPDX-License-Identifier: GPL-2.0-or-later

use crate::domain::Domain;
use std::path::PathBuf;

/// Represents whether and how the user has requested the device to be grabbed.
/// Set through the grab flag or grab= clause on --input arguments.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GrabMode {
    /// The user has requested this device to be grabbed immediately.
    Force,
    /// The user has requested this device to be grabbed once no EV_KEY keys are pressed.
    Auto,
    /// The user has requested this device not be grabbed.
    None,
}

impl GrabMode {
    /// If some input device is specified multiple times with different grab mode,
    /// this function finds the strongest of both of them.
    pub fn combine(first: GrabMode, second: GrabMode) -> GrabMode {
        if first == GrabMode::Force || second == GrabMode::Force {
            GrabMode::Force
        } else if first == GrabMode::Auto || second == GrabMode::Auto {
            GrabMode::Auto
        } else {
            GrabMode::None
        }
    }
}

pub struct PreInputDevice {
    /// The path to this device.
    pub path: PathBuf,
    /// The domain that all events emitted by this device shall have.
    pub domain: Domain,
    /// Whether and how the user has requested this InputDevice be grabbed.
    pub grab_mode: GrabMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    /// The kernel shall generate repeat events for this device.
    Enable,
    /// No repeat events shall be generated by this device.
    Disable,
    /// This device shall not be given a repeat capability, but repeat events shall be written to it.
    Passive,
}

pub struct PreOutputDevice {
    /// All events with this domain shall be written to this device.
    pub domain: Domain,
    /// If Some, the user has requested a symlink to the device to be created at the given path.
    pub create_link: Option<PathBuf>,
    /// The output device will be given this name.
    pub name: String,
    /// Determined by "repeat" or "norepeat" flags on output devices.
    pub repeat_mode: RepeatMode,
}
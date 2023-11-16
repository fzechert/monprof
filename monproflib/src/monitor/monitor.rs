use log::{debug, trace};
use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};

use crate::monitor::{edid::Edid, sysfs};

/// Monitor
#[derive(Debug)]
pub struct Monitor {
    /// Path to the sysfs entry of this monitor.
    sysfs_path: PathBuf,
    /// Graphics Card.
    card: String,
    /// Port name of the monitor.
    port_name: String,
}

impl Monitor {
    pub fn from_sysfs<P>(sysfs_path: P) -> Monitor
    where
        P: AsRef<Path>,
    {
        trace!(
            "Loading monitor information from sysfs {}",
            sysfs_path.as_ref().to_string_lossy()
        );

        let card: String = sysfs_path
            .as_ref()
            .parent()
            .expect("Sysfs path must have a parent directory")
            .file_name()
            .expect("A directory must have a name")
            .to_string_lossy()
            .into_owned();

        let port_name: String = sysfs_path
            .as_ref()
            .file_name()
            .expect("A directory must have a name")
            .to_string_lossy()
            .chars()
            .skip(card.len() + 1)
            .collect();

        let edid_file = sysfs_path.as_ref().join(sysfs::EDID_PATH);
        let edid_bytes = fs::read(&edid_file).expect("Expecting to be able to read from sysfs");
        let edid = Edid::parse(&mut Cursor::new(edid_bytes));

        debug!("Monitor connected to {} on port {}", card, port_name);

        Monitor {
            sysfs_path: sysfs_path.as_ref().into(),
            card: card,
            port_name: port_name,
        }
    }
}

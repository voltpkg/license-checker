use cargo_metadata::Package;

use crate::license::License;

pub trait Licensed {
    fn license(&self) -> License;
}

impl Licensed for Package {
    fn license(&self) -> License {
        self.license
            .as_ref()
            .and_then(|license| license.parse::<License>().ok())
            .or_else(|| self.license_file().map(License::File))
            .unwrap_or_default()
    }
}

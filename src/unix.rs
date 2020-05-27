use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use crate::Absolutize;

use crate::path_dedot::ParseDot;

impl Absolutize for Path {
    #[allow(clippy::let_unit_value)]
    fn absolutize(&self) -> io::Result<PathBuf> {
        if self.is_absolute() {
            self.parse_dot()
        } else {
            let _cwd = get_cwd_pathbuf!();

            let path = Path::join(get_cwd!(_cwd), self);

            path.parse_dot()
        }
    }

    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let mut virtual_root = virtual_root.as_ref().absolutize()?;

        if self.is_absolute() {
            let path = self.parse_dot()?;

            if !path.starts_with(&virtual_root) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let path = self.parse_dot()?;

            if path.is_absolute() {
                if !path.starts_with(&virtual_root) {
                    return Err(io::Error::from(ErrorKind::InvalidInput));
                }

                Ok(path)
            } else {
                virtual_root.push(path);

                Ok(virtual_root)
            }
        }
    }
}

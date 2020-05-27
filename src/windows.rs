use std::ffi::OsString;
use std::io::{self, ErrorKind};
use std::path::{Component, Path, PathBuf};

use crate::path_dedot::{ParseDot, ParsePrefix, MAIN_SEPARATOR};
use crate::Absolutize;

impl Absolutize for Path {
    fn absolutize(&self) -> io::Result<PathBuf> {
        let mut size = self.as_os_str().len();

        let mut iter = self.components();

        let cwd = get_cwd!();

        if let Some(first_component) = iter.next() {
            let mut tokens = Vec::new();

            match first_component {
                Component::Prefix(prefix) => {
                    tokens.push(prefix.as_os_str());

                    if let Some(second_component) = iter.next() {
                        match second_component {
                            Component::RootDir => {
                                tokens.push(MAIN_SEPARATOR.as_os_str());
                            }
                            Component::CurDir => {
                                // may be unreachable

                                for token in cwd.iter().skip(1) {
                                    tokens.push(token);
                                    size += token.len();
                                }

                                size -= 1;
                            }
                            Component::ParentDir => {
                                match cwd.parent() {
                                    Some(cwd_parent) => {
                                        for token in cwd_parent.iter().skip(1) {
                                            tokens.push(token);
                                            size += token.len();
                                        }

                                        size -= 2;
                                    }
                                    None => {
                                        tokens.push(MAIN_SEPARATOR.as_os_str());

                                        size -= 1;
                                    }
                                }
                            }
                            _ => {
                                let path_str = self.as_os_str().to_str().ok_or_else(|| {
                                    io::Error::new(ErrorKind::Other, "The path is not valid UTF-8.")
                                })?;

                                if path_str[first_component.as_os_str().len()..].starts_with('.') {
                                    for token in cwd.iter().skip(1) {
                                        tokens.push(token);
                                        size += token.len();
                                    }

                                    size -= 1;

                                    tokens.push(second_component.as_os_str());
                                } else {
                                    for token in cwd.iter().skip(1) {
                                        tokens.push(token);
                                        size += token.len();
                                    }

                                    size += 1;

                                    tokens.push(second_component.as_os_str());
                                }
                            }
                        }
                    } else {
                        tokens.push(MAIN_SEPARATOR.as_os_str());

                        size += 1;
                    }
                }
                Component::RootDir => {
                    let prefix = cwd.get_path_prefix().unwrap().as_os_str();
                    tokens.push(prefix);
                    size += prefix.len();

                    tokens.push(MAIN_SEPARATOR.as_os_str());
                }
                Component::CurDir => {
                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    size += cwd.as_os_str().len() - 1;
                }
                Component::ParentDir => {
                    match cwd.parent() {
                        Some(cwd_parent) => {
                            for token in cwd_parent.iter() {
                                tokens.push(token);
                            }

                            size += cwd_parent.as_os_str().len() - 2;
                        }
                        None => {
                            let prefix = cwd.get_path_prefix().unwrap().as_os_str();
                            tokens.push(prefix);
                            size += prefix.len();

                            tokens.push(MAIN_SEPARATOR.as_os_str());
                            size -= 1;
                        }
                    }
                }
                Component::Normal(token) => {
                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    size += cwd.as_os_str().len() + 1;

                    tokens.push(token);
                }
            }

            for component in iter {
                match component {
                    Component::CurDir => {
                        // may be unreachable

                        size -= 2;
                    }
                    Component::ParentDir => {
                        let tokens_length = tokens.len();

                        if tokens_length > 2 {
                            let removed = tokens.remove(tokens_length - 1);
                            size -= removed.len() + 4; // xxx\..\
                        } else {
                            size -= 3; // ..\
                        }
                    }
                    _ => {
                        tokens.push(component.as_os_str());
                    }
                }
            }

            debug_assert!(!tokens.is_empty());

            let mut path_string = OsString::with_capacity(size);

            let mut iter = tokens.iter();

            path_string.push(iter.next().unwrap());
            path_string.push(iter.next().unwrap());

            let tokens_length = tokens.len();

            if tokens_length > 2 {
                for &token in iter.take(tokens_length - 3) {
                    path_string.push(token);

                    path_string.push(MAIN_SEPARATOR.as_os_str());
                }

                path_string.push(tokens[tokens_length - 1]);
            }

            debug_assert!(size + 1 >= path_string.len()); // + 1 is for `\\server\share` -> `\\server\share\`

            let path_buf = PathBuf::from(path_string);

            Ok(path_buf)
        } else {
            #[allow(clippy::identity_conversion)]
            Ok(cwd.into())
        }
    }

    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<PathBuf> {
        let mut virtual_root = virtual_root.as_ref().absolutize()?;

        if self.is_absolute() {
            let path = self.parse_dot()?;

            let path_lowercase = path
                .to_str()
                .ok_or_else(|| io::Error::new(ErrorKind::Other, "The path is not valid UTF-8."))?
                .to_lowercase();

            let virtual_root_lowercase = virtual_root
                .to_str()
                .ok_or_else(|| {
                    io::Error::new(ErrorKind::Other, "The virtual root is not valid UTF-8.")
                })?
                .to_lowercase();

            if !&path_lowercase.starts_with(&virtual_root_lowercase) {
                return Err(io::Error::from(ErrorKind::InvalidInput));
            }

            Ok(path)
        } else {
            let path = self.parse_dot()?;

            if path.is_absolute() {
                let path_lowercase = path
                    .to_str()
                    .ok_or_else(|| {
                        io::Error::new(ErrorKind::Other, "The path is not valid UTF-8.")
                    })?
                    .to_lowercase();

                let virtual_root_lowercase = virtual_root
                    .to_str()
                    .ok_or_else(|| {
                        io::Error::new(ErrorKind::Other, "The virtual root is not valid UTF-8.")
                    })?
                    .to_lowercase();

                if !&path_lowercase.starts_with(&virtual_root_lowercase) {
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

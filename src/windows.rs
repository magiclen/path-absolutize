use std::borrow::Cow;
use std::ffi::OsString;
use std::io::{self, ErrorKind};
use std::path::{Component, Path, PathBuf};

use crate::path_dedot::{ParseDot, ParsePrefix, MAIN_SEPARATOR};
use crate::Absolutize;

impl Absolutize for Path {
    fn absolutize(&self) -> io::Result<Cow<Path>> {
        let mut size = self.as_os_str().len();

        let mut iter = self.components();

        let mut has_change = false;

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

                                let mut cwd_iter = cwd.iter().skip(1);

                                if let Some(token) = cwd_iter.next() {
                                    tokens.push(token);
                                    size += token.len();

                                    for token in cwd_iter {
                                        tokens.push(token);
                                        size += token.len() + 1;
                                    }
                                }

                                size -= 1;

                                has_change = true;
                            }
                            Component::ParentDir => {
                                match cwd.parent() {
                                    Some(cwd_parent) => {
                                        let mut cwd_parent_iter = cwd_parent.iter().skip(1);

                                        if let Some(token) = cwd_parent_iter.next() {
                                            tokens.push(token);
                                            size += token.len();

                                            for token in cwd_parent_iter {
                                                tokens.push(token);
                                                size += token.len() + 1;
                                            }
                                        }

                                        size -= 2;
                                    }
                                    None => {
                                        tokens.push(MAIN_SEPARATOR.as_os_str());

                                        size -= 1;
                                    }
                                }

                                has_change = true;
                            }
                            _ => {
                                let path_str = self.as_os_str().to_str().ok_or_else(|| {
                                    io::Error::new(ErrorKind::Other, "The path is not valid UTF-8.")
                                })?;

                                if path_str[first_component.as_os_str().len()..].starts_with('.') {
                                    let mut cwd_iter = cwd.iter().skip(1);

                                    if let Some(token) = cwd_iter.next() {
                                        tokens.push(token);
                                        size += token.len();

                                        for token in cwd_iter {
                                            tokens.push(token);
                                            size += token.len() + 1;
                                        }
                                    }

                                    size -= 1;

                                    tokens.push(second_component.as_os_str());
                                } else {
                                    let mut cwd_iter = cwd.iter().skip(1);

                                    if let Some(token) = cwd_iter.next() {
                                        tokens.push(token);
                                        size += token.len();

                                        for token in cwd_iter {
                                            tokens.push(token);
                                            size += token.len() + 1;
                                        }
                                    }

                                    tokens.push(second_component.as_os_str());
                                }

                                has_change = true;
                            }
                        }
                    } else {
                        tokens.push(MAIN_SEPARATOR.as_os_str());

                        size += 1;

                        // has_change = true; // don't need this because `C:` -> `C:\` is like `\\server\share` -> `\\server\share\`
                    }
                }
                Component::RootDir => {
                    let prefix = cwd.get_path_prefix().unwrap().as_os_str();
                    tokens.push(prefix);
                    size += prefix.len();

                    tokens.push(MAIN_SEPARATOR.as_os_str());

                    has_change = true;
                }
                Component::CurDir => {
                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    size += cwd.as_os_str().len() - 1;

                    has_change = true;
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

                    has_change = true;
                }
                Component::Normal(token) => {
                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    size += cwd.as_os_str().len() + 1;

                    tokens.push(token);

                    has_change = true;
                }
            }

            for component in iter {
                match component {
                    Component::CurDir => {
                        // may be unreachable
                        size -= 2;

                        has_change = true;
                    }
                    Component::ParentDir => {
                        let tokens_length = tokens.len();

                        if tokens_length > 2 {
                            let removed = tokens.remove(tokens_length - 1);
                            size -= removed.len() + 4; // xxx\..\
                        } else {
                            size -= 3; // ..\
                        }

                        has_change = true;
                    }
                    _ => {
                        tokens.push(component.as_os_str());
                    }
                }
            }

            debug_assert!(!tokens.is_empty());

            let tokens_length = tokens.len();

            if has_change {
                let mut path_string = OsString::with_capacity(size);

                let mut iter = tokens.iter();

                path_string.push(iter.next().unwrap());
                path_string.push(iter.next().unwrap());

                if tokens_length > 2 {
                    for &token in iter.take(tokens_length - 3) {
                        path_string.push(token);

                        path_string.push(MAIN_SEPARATOR.as_os_str());
                    }

                    path_string.push(tokens[tokens_length - 1]);
                }

                debug_assert!(size >= path_string.len());

                let path_buf = PathBuf::from(path_string);

                Ok(Cow::from(path_buf))
            } else if tokens_length == 2 && tokens[0].len() == self.as_os_str().len() {
                let mut path_string = OsString::with_capacity(size);

                let mut iter = tokens.iter();

                path_string.push(iter.next().unwrap());
                path_string.push(iter.next().unwrap());

                debug_assert!(size + 1 >= path_string.len()); // + 1 is for `\\server\share` -> `\\server\share\`

                let path_buf = PathBuf::from(path_string);

                Ok(Cow::from(path_buf))
            } else {
                Ok(Cow::from(self))
            }
        } else {
            Ok(Cow::from(cwd))
        }
    }

    fn absolutize_virtually<P: AsRef<Path>>(&self, virtual_root: P) -> io::Result<Cow<Path>> {
        let virtual_root = virtual_root.as_ref().absolutize()?;

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
                let mut virtual_root = virtual_root.into_owned();

                virtual_root.push(path);

                Ok(Cow::from(virtual_root))
            }
        }
    }
}

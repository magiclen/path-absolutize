#![cfg(all(unix, not(feature = "unsafe_cache")))]

use std::{env, io::ErrorKind, path::Path};

use path_absolutize::Absolutize;

#[test]
fn absolutize_lv0_1() {
    let p = Path::new("/path/to/123/456");

    assert_eq!("/path/to/123/456", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv0_2() {
    let p = Path::new("/path/to/./123/456");

    assert_eq!("/path/to/123/456", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv0_3() {
    let p = Path::new("/path/to/./123/../456");

    assert_eq!("/path/to/456", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv0_4() {
    let p = Path::new("/../");

    assert_eq!("/", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv0_5() {
    let p = Path::new("/..");

    assert_eq!("/", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv1_1() {
    let p = Path::new(".");

    assert_eq!(
        env::current_dir().unwrap().to_str().unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv1_2() {
    let p = Path::new("./path/to/123/456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new("path/to/123/456"))
            .to_str()
            .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv1_3() {
    let p = Path::new("..");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(cwd_parent.to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
        },
        None => {
            assert_eq!("/", p.absolutize().unwrap().to_str().unwrap());
        },
    }
}

#[test]
fn absolutize_lv1_4() {
    let p = Path::new("../path/to/123/456");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(
                cwd_parent.join("path/to/123/456").to_str().unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        },
        None => {
            assert_eq!(
                Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        },
    }
}

#[test]
fn absolutize_lv2() {
    let p = Path::new("path/to/123/456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new("path/to/123/456"))
            .to_str()
            .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv3() {
    let p = Path::new("path/../../to/123/456");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(
                cwd_parent.join("to/123/456").to_str().unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        },
        None => {
            assert_eq!(
                Path::join(Path::new("/"), Path::new("to/123/456")).to_str().unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        },
    }
}

#[test]
fn virtually_absolutize_lv0_1() {
    let p = Path::new("/path/to/123/456");

    assert_eq!("/path/to/123/456", p.absolutize_virtually("/").unwrap().to_str().unwrap());
}

#[test]
fn virtually_absolutize_lv0_2() {
    let p = Path::new("/path/to/./123/../456");

    assert_eq!("/path/to/456", p.absolutize_virtually("/").unwrap().to_str().unwrap());
}

#[test]
fn virtually_absolutize_lv0_3() {
    let p = Path::new("/path/to/123/456");

    assert_eq!(
        ErrorKind::InvalidInput,
        p.absolutize_virtually("/virtual/root").unwrap_err().kind()
    );
}

#[test]
fn virtually_absolutize_lv1_1() {
    let p = Path::new("./path/to/123/456");

    assert_eq!(
        ErrorKind::InvalidInput,
        p.absolutize_virtually("/virtual/root").unwrap_err().kind()
    );
}

#[test]
fn virtually_absolutize_lv1_2() {
    let p = Path::new("../path/to/123/456");

    assert_eq!(
        ErrorKind::InvalidInput,
        p.absolutize_virtually("/virtual/root").unwrap_err().kind()
    );
}

#[test]
fn virtually_absolutize_lv2() {
    let p = Path::new("path/to/123/456");

    assert_eq!(
        "/virtual/root/path/to/123/456",
        p.absolutize_virtually("/virtual/root").unwrap().to_str().unwrap()
    );
}

#[test]
fn virtually_absolutize_lv3() {
    let p = Path::new("path/to/../../../../123/456");

    assert_eq!(
        "/virtual/root/123/456",
        p.absolutize_virtually("/virtual/root").unwrap().to_str().unwrap()
    );
}

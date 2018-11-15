#![cfg(windows)]

extern crate path_absolutize;
extern crate path_dedot;

use std::path::{Path, PathBuf};

use path_absolutize::{CWD, Absolutize};
use path_dedot::ParsePrefix;

#[test]
fn absolutize_lv0_1() {
    let p = Path::new(r"\path\to\123\456");

    assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"\path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv0_2() {
    let p = Path::new(r"\path\to\.\123\..\456");

    assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"\path\to\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv1_1() {
    let p = Path::new(r".\path\to\123\456");

    assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv1_2() {
    let p = Path::new(r"..\path\to\123\456");

    let cwd_parent = CWD.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
        }
        None => {
            assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
        }
    }
}

#[test]
fn absolutize_lv2() {
    let p = Path::new(r"path\to\123\456");

    assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn absolutize_lv3() {
    let p = Path::new(r"path\..\..\to\123\456");

    let cwd_parent = CWD.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(Path::join(&cwd_parent, Path::new(r"to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
        }
        None => {
            assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"to\123\456")).to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
        }
    }
}

#[test]
fn absolutize_lv4() {
    let cwd_prefix = CWD.get_path_prefix().unwrap();

    let target_prefix = if cwd_prefix.as_os_str().ne("C:") {
        "C:"
    } else {
        "D:"
    };

    let target = PathBuf::from(format!(r"{}123\567", target_prefix));

    let cwd = CWD.to_str().unwrap();

    let path = PathBuf::from(format!(r"{}{}\123\567", target_prefix, &cwd[cwd_prefix.as_os_str().to_str().unwrap().len()..]));

    assert_eq!(path.to_str().unwrap(), target.absolutize().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn absolutize_lv5() {
    let cwd_prefix = CWD.get_path_prefix().unwrap();

    let target_prefix = if cwd_prefix.as_os_str().ne("C:") {
        "C:"
    } else {
        "D:"
    };

    let target = PathBuf::from(format!(r"{}.\123\567", target_prefix));

    let cwd = CWD.to_str().unwrap();

    let path = PathBuf::from(format!(r"{}{}\123\567", target_prefix, &cwd[cwd_prefix.as_os_str().to_str().unwrap().len()..]));

    assert_eq!(path.to_str().unwrap(), target.absolutize().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn absolutize_lv6() {
    let cwd_prefix = CWD.get_path_prefix().unwrap();

    let target_prefix = if cwd_prefix.as_os_str().ne("C:") {
        "C:"
    } else {
        "D:"
    };

    let target = PathBuf::from(format!(r"{}..\123\567", target_prefix));

    let cwd_parent = CWD.parent();

    let path = match cwd_parent {
        Some(cwd_parent) => {
            let cwd_parent = cwd_parent.to_str().unwrap();
            PathBuf::from(format!(r"{}{}\123\567", target_prefix, &cwd_parent[cwd_prefix.as_os_str().to_str().unwrap().len()..]))
        }
        None => {
            PathBuf::from(format!(r"{}\123\567", target_prefix))
        }
    };

    assert_eq!(path.to_str().unwrap(), target.absolutize().unwrap().to_str().unwrap());
}
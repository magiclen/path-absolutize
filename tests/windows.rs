#![cfg(all(windows, not(feature = "unsafe_cache")))]

#[macro_use]
extern crate slash_formatter;

extern crate path_absolutize;

use std::env;
use std::path::{Path, PathBuf};

use path_absolutize::path_dedot::ParsePrefix;
use path_absolutize::Absolutize;

#[test]
fn absolutize_lv0_1() {
    let p = Path::new(r"\path\to\123\456");

    assert_eq!(
        Path::join(
            Path::new(env::current_dir().unwrap().get_path_prefix().unwrap().as_os_str()),
            Path::new(r"\path\to\123\456"),
        )
        .to_str()
        .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv0_2() {
    let p = Path::new(r"\path\to\.\123\456");

    assert_eq!(
        Path::join(
            Path::new(env::current_dir().unwrap().get_path_prefix().unwrap().as_os_str()),
            Path::new(r"\path\to\123\456"),
        )
        .to_str()
        .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv0_3() {
    let p = Path::new(r"\path\to\.\123\..\456");

    assert_eq!(
        Path::join(
            Path::new(env::current_dir().unwrap().get_path_prefix().unwrap().as_os_str()),
            Path::new(r"\path\to\456"),
        )
        .to_str()
        .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv0_4() {
    let p = Path::new(r"\..\");

    assert_eq!(
        Path::join(
            Path::new(env::current_dir().unwrap().get_path_prefix().unwrap().as_os_str()),
            Path::new(r"\"),
        )
        .to_str()
        .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv0_5() {
    let p = Path::new(r"\..");

    assert_eq!(
        Path::join(
            Path::new(env::current_dir().unwrap().get_path_prefix().unwrap().as_os_str()),
            Path::new(r"\"),
        )
        .to_str()
        .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv1_1() {
    let p = Path::new(r".");

    assert_eq!(
        env::current_dir().unwrap().to_str().unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv1_2() {
    let p = Path::new(r".\path\to\123\456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new(r"path\to\123\456"))
            .to_str()
            .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv1_3() {
    let p = Path::new(r"..");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(cwd_parent.to_str().unwrap(), p.absolutize().unwrap().to_str().unwrap());
        }
        None => {
            assert_eq!(
                Path::new(cwd.get_path_prefix().unwrap().as_os_str()).to_str().unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        }
    }
}

#[test]
fn absolutize_lv1_4() {
    let p = Path::new(r"..\path\to\123\456");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(
                Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        }
        None => {
            assert_eq!(
                Path::join(
                    Path::new(cwd.get_path_prefix().unwrap().as_os_str()),
                    Path::new(r"path\to\123\456"),
                )
                .to_str()
                .unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        }
    }
}

#[test]
fn absolutize_lv2() {
    let p = Path::new(r"path\to\123\456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new(r"path\to\123\456"))
            .to_str()
            .unwrap(),
        p.absolutize().unwrap().to_str().unwrap()
    );
}

#[test]
fn absolutize_lv3() {
    let p = Path::new(r"path\..\..\to\123\456");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(
                Path::join(&cwd_parent, Path::new(r"to\123\456")).to_str().unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        }
        None => {
            assert_eq!(
                Path::join(
                    Path::new(cwd.get_path_prefix().unwrap().as_os_str()),
                    Path::new(r"to\123\456"),
                )
                .to_str()
                .unwrap(),
                p.absolutize().unwrap().to_str().unwrap()
            );
        }
    }
}

#[test]
fn absolutize_lv4() {
    let cwd = env::current_dir().unwrap();

    let cwd_prefix = cwd.get_path_prefix().unwrap();

    let target_prefix = if cwd_prefix.as_os_str().ne("C:") {
        "C:"
    } else {
        "D:"
    };

    let target = PathBuf::from(format!(r"{}123\567", target_prefix));

    let cwd = cwd.to_str().unwrap();

    let path = PathBuf::from(backslash!(
        target_prefix,
        &cwd[cwd_prefix.as_os_str().to_str().unwrap().len()..],
        r"123\567"
    ));

    assert_eq!(path.to_str().unwrap(), target.absolutize().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn absolutize_lv5() {
    let cwd = env::current_dir().unwrap();

    let cwd_prefix = cwd.get_path_prefix().unwrap();

    let target_prefix = if cwd_prefix.as_os_str().ne("C:") {
        "C:"
    } else {
        "D:"
    };

    let target = PathBuf::from(format!(r"{}.\123\567", target_prefix));

    let cwd = cwd.to_str().unwrap();

    let path = PathBuf::from(backslash!(
        target_prefix,
        &cwd[cwd_prefix.as_os_str().to_str().unwrap().len()..],
        r"123\567"
    ));

    assert_eq!(path.to_str().unwrap(), target.absolutize().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn absolutize_lv6() {
    let cwd = env::current_dir().unwrap();

    let cwd_prefix = cwd.get_path_prefix().unwrap();

    let target_prefix = if cwd_prefix.as_os_str().ne("C:") {
        "C:"
    } else {
        "D:"
    };

    let target = PathBuf::from(format!(r"{}..\123\567", target_prefix));

    let cwd_parent = cwd.parent();

    let path = match cwd_parent {
        Some(cwd_parent) => {
            let cwd_parent = cwd_parent.to_str().unwrap();

            PathBuf::from(backslash!(
                target_prefix,
                &cwd_parent[cwd_prefix.as_os_str().to_str().unwrap().len()..],
                r"123\567"
            ))
        }
        None => PathBuf::from(backslash!(target_prefix, r"123\567")),
    };

    assert_eq!(path.to_str().unwrap(), target.absolutize().unwrap().to_str().unwrap());
}

#[test]
fn prefix_1() {
    let p = Path::new(r"C:\");

    assert_eq!(r"C:\", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn prefix_2() {
    let p = Path::new(r"C:");

    assert_eq!(r"C:\", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn prefix_3() {
    let p = Path::new(r"\\VBOXSRV\test");

    assert_eq!(r"\\VBOXSRV\test\", p.absolutize().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn prefix_4() {
    let p = Path::new(r"\\VBOXSRV\test\");

    assert_eq!(r"\\VBOXSRV\test\", p.absolutize().unwrap().to_str().unwrap());
}

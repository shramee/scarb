use assert_fs::prelude::*;
use indoc::indoc;
use predicates::prelude::*;
use snapbox::cmd::{cargo_bin, Command};
use std::fs;

use scarb::core::TomlManifest;

#[test]
fn new_simple() {
    let pt = assert_fs::TempDir::new().unwrap();

    Command::new(cargo_bin!("scarb"))
        .arg("new")
        .arg("hello")
        .current_dir(&pt)
        .assert()
        .success();

    let t = pt.child("hello");
    assert!(t.is_dir());
    assert!(t.child("Scarb.toml").is_file());
    assert!(t.child("src/lib.cairo").is_file());
    assert!(t.child(".gitignore").is_file());

    let toml_manifest = TomlManifest::read_from_path(t.child("Scarb.toml").path()).unwrap();
    assert_eq!(toml_manifest.package.unwrap().name, "hello");

    Command::new(cargo_bin!("scarb"))
        .arg("build")
        .current_dir(&t)
        .assert()
        .success();

    t.child("target/release/hello.sierra")
        .assert(predicates::str::is_empty().not());
}

#[test]
fn init_simple() {
    let pt = assert_fs::TempDir::new().unwrap();
    let t = pt.child("hello");
    t.create_dir_all().unwrap();

    Command::new(cargo_bin!("scarb"))
        .arg("init")
        .current_dir(&t)
        .assert()
        .success();

    assert!(t.is_dir());
    assert!(t.child("Scarb.toml").is_file());
    assert!(t.child("src/lib.cairo").is_file());
    assert!(t.child(".gitignore").is_file());

    let toml_manifest = TomlManifest::read_from_path(t.child("Scarb.toml").path()).unwrap();
    assert_eq!(toml_manifest.package.unwrap().name, "hello");

    Command::new(cargo_bin!("scarb"))
        .arg("build")
        .current_dir(&t)
        .assert()
        .success();

    t.child("target/release/hello.sierra")
        .assert(predicates::str::is_empty().not());
}

#[test]
fn new_no_path_arg() {
    Command::new(cargo_bin!("scarb"))
        .arg("new")
        .assert()
        .failure()
        .stderr_matches(indoc! {r#"
            error: the following required arguments were not provided:
              <PATH>

            Usage: scarb[..] new <PATH>

            For more information, try '--help'.
        "#});
}

#[test]
fn new_existing() {
    let pt = assert_fs::TempDir::new().unwrap();
    let t = pt.child("hello");
    t.create_dir_all().unwrap();

    Command::new(cargo_bin!("scarb"))
        .arg("new")
        .arg("hello")
        .current_dir(&pt)
        .assert()
        .failure()
        .stderr_eq(indoc! {r#"
            Error: destination `hello` already exists
            Use `scarb init` to initialize the directory.
        "#});
}

#[test]
fn invalid_package_name() {
    let pt = assert_fs::TempDir::new().unwrap();
    Command::new(cargo_bin!("scarb"))
        .arg("new")
        .arg("a-b")
        .current_dir(&pt)
        .assert()
        .failure()
        .stderr_eq(indoc! {r#"
            Error: invalid character `-` in package name: `a-b`, characters must be ASCII letter, ASCII numbers or underscore
        "#});
}

// TODO(mkaput): Test keyword as name.
// TODO(mkaput): Test corelib as name.

#[test]
fn new_explicit_project_name() {
    let pt = assert_fs::TempDir::new().unwrap();

    Command::new(cargo_bin!("scarb"))
        .arg("new")
        .arg("hello")
        .arg("--name")
        .arg("world")
        .current_dir(&pt)
        .assert()
        .success();

    let t = pt.child("hello");

    let toml_manifest = TomlManifest::read_from_path(t.child("Scarb.toml").path()).unwrap();
    assert_eq!(toml_manifest.package.unwrap().name, "world");
}

#[test]
fn init_does_not_overwrite_gitignore() {
    let pt = assert_fs::TempDir::new().unwrap();
    let t = pt.child("hello");
    t.create_dir_all().unwrap();
    t.child(".gitignore").write_str("examples\n").unwrap();

    Command::new(cargo_bin!("scarb"))
        .arg("init")
        .current_dir(&t)
        .assert()
        .success();

    assert_eq!(
        &fs::read_to_string(t.child(".gitignore").path()).unwrap(),
        "examples\n"
    );
}
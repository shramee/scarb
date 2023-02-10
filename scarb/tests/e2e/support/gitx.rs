use std::path::Path;
use std::{fmt, fs};

use assert_fs::fixture::{ChildPath, PathChild};
use assert_fs::TempDir;
use snapbox::cmd::Command;
use url::Url;

pub struct GitProject {
    pub name: String,
    pub t: TempDir,
    pub p: ChildPath,
}

impl GitProject {
    pub fn url(&self) -> String {
        Url::from_file_path(self.p.path()).unwrap().to_string()
    }

    pub fn git(&self, args: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>) {
        git(self, args)
    }

    pub fn commit(&self) {
        commit(&self.p)
    }

    pub fn child(&self, path: impl AsRef<Path>) -> ChildPath {
        self.p.child(path)
    }
}

impl fmt::Display for GitProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

pub fn new(name: impl Into<String>, f: impl FnOnce(ChildPath)) -> GitProject {
    let name = name.into();
    let t = TempDir::new().unwrap();
    let child = t.child(&name);
    init(child.path());
    f(child);
    let p = t.child(&name);
    commit(p.path());
    GitProject { name, t, p }
}

/// Initialize a Git new repository at the given path.
pub fn init(path: &Path) {
    fs::create_dir_all(path).unwrap();
    git(path, ["init", "-b", "main"]);
    git(path, ["config", "--local", "user.name", "Szczepan Czekan"]);
    git(
        path,
        ["config", "--local", "user.email", "szczekan@swmansion.com"],
    );
}

/// Commit staged changes to the Git repository.
pub fn commit(work_dir: &Path) {
    git(work_dir, ["add", "."]);
    git(work_dir, ["commit", "-m", "test"]);
}

pub fn git(cwd: impl GitContext, args: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>) {
    git_command()
        .args(args)
        .current_dir(cwd.git_path())
        .assert()
        .success();
}

pub fn git_command() -> Command {
    Command::new("git")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
}

pub trait GitContext {
    fn git_path(&self) -> &Path;
}

impl GitContext for &GitProject {
    fn git_path(&self) -> &Path {
        self.p.path()
    }
}

impl GitContext for &ChildPath {
    fn git_path(&self) -> &Path {
        self.path()
    }
}

impl GitContext for &Path {
    fn git_path(&self) -> &Path {
        self
    }
}
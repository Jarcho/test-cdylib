use cargo_metadata::Message;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::error::{Error, Result};
use crate::features;
use crate::run::Project;
use crate::rustflags;

#[derive(Deserialize)]
pub struct Metadata {
    pub target_directory: PathBuf,
    pub workspace_root: PathBuf,
}

fn raw_cargo() -> Command {
    Command::new(option_env!("CARGO").unwrap_or("cargo"))
}

fn cargo(project: &Project) -> Command {
    let mut cmd = raw_cargo();
    cmd.current_dir(&project.dir);
    cmd.env("CARGO_TARGET_DIR", path!(&project.dir / "target"));
    rustflags::set_env(&mut cmd);
    cmd
}

pub fn build_cdylib(project: &Project) -> Result<PathBuf> {
    let result = cargo(project)
        .arg("build")
        .arg("--message-format=json")
        .args(features(project))
        .stderr(Stdio::inherit())
        .output()
        .map_err(Error::Cargo)?;

    let mut artifact = None;
    for message in cargo_metadata::parse_messages(result.stdout.as_slice()) {
        match message? {
            Message::CompilerMessage(m) => eprintln!("{}", m),
            Message::CompilerArtifact(a) => artifact = Some(a),
            _ => (),
        }
    }

    if !result.status.success() {
        return Err(Error::CargoFail);
    }
    artifact
        .ok_or(Error::CargoFail)
        .map(|a| a.filenames[0].clone())
}

pub fn build_self_cdylib() -> Result<PathBuf> {
    let features = match features::find() {
        Some(features) => vec![
            "--no-default-features".to_owned(),
            "--features".to_owned(),
            features.join(","),
        ],
        None => vec![],
    };

    let mut cargo = raw_cargo();
    rustflags::set_env(&mut cargo);
    let result = cargo
        .arg("build")
        .arg("--lib")
        .arg("--message-format=json")
        .args(features)
        .stderr(Stdio::inherit())
        .output()
        .map_err(Error::Cargo)?;

    let mut artifact = None;
    for message in cargo_metadata::parse_messages(result.stdout.as_slice()) {
        match message? {
            Message::CompilerMessage(m) => eprintln!("{}", m),
            Message::CompilerArtifact(a) => artifact = Some(a),
            _ => (),
        }
    }

    if !result.status.success() {
        return Err(Error::CargoFail);
    }
    artifact
        .ok_or(Error::CargoFail)
        .map(|a| a.filenames[0].clone())
}

pub fn metadata() -> Result<Metadata> {
    let output = raw_cargo()
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .map_err(Error::Cargo)?;

    serde_json::from_slice(&output.stdout).map_err(Error::Metadata)
}

fn features(project: &Project) -> Vec<String> {
    match &project.features {
        Some(features) => vec![
            "--no-default-features".to_owned(),
            "--features".to_owned(),
            features.join(","),
        ],
        None => vec![],
    }
}

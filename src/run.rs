use std::collections::BTreeMap as Map;
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::cargo;
use crate::dependencies::{self, Dependency};
use crate::error::{Error, Result};
use crate::features;
use crate::manifest::{Build, Config, Lib, Manifest, Package, Workspace};
use crate::rustflags;

#[derive(Debug)]
pub struct Project {
    pub dir: PathBuf,
    source_dir: PathBuf,
    pub target_dir: PathBuf,
    pub name: String,
    pub features: Option<Vec<String>>,
    workspace: PathBuf,
}

pub(crate) fn run(path: &Path) -> Result<PathBuf> {
    check_exists(path)?;
    let project = prepare(path)?;
    cargo::build_cdylib(&project)
}

fn prepare(path: &Path) -> Result<Project> {
    let metadata = cargo::metadata()?;
    let target_dir = metadata.target_directory;
    let workspace = metadata.workspace_root;

    let crate_name = env::var("CARGO_PKG_NAME").map_err(Error::PkgName)?;
    let test_name = path.file_stem().unwrap();

    let source_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok_or(Error::ProjectDir)?;

    let features = features::find();
    let mut project = Project {
        dir: path!(target_dir / "cdylibs" / crate_name / test_name),
        source_dir,
        target_dir,
        name: format!("{}-cdylib-{}", crate_name, test_name.to_string_lossy()),
        features,
        workspace,
    };

    let manifest = make_manifest(&crate_name, &project, path)?;
    let manifest_toml = toml::to_string(&manifest)?;

    let config = make_config();
    let config_toml = toml::to_string(&config)?;

    if let Some(enabled_features) = &mut project.features {
        enabled_features.retain(|feature| manifest.features.contains_key(feature));
    }

    fs::create_dir_all(path!(project.dir / ".cargo"))?;
    fs::write(path!(project.dir / ".cargo" / "config"), config_toml)?;
    fs::write(path!(project.dir / "Cargo.toml"), manifest_toml)?;

    Ok(project)
}

fn make_manifest(crate_name: &str, project: &Project, cdylib_path: &Path) -> Result<Manifest> {
    let source_manifest = dependencies::get_manifest(&project.source_dir);
    let workspace_manifest = dependencies::get_workspace_manifest(&project.workspace);

    let features = source_manifest
        .features
        .keys()
        .map(|feature| {
            let enable = format!("{}/{}", crate_name, feature);
            (feature.clone(), vec![enable])
        })
        .collect();

    let mut manifest = Manifest {
        package: Package {
            name: project.name.clone(),
            version: "0.0.0".to_owned(),
            edition: source_manifest.package.edition,
            publish: false,
        },
        lib: Lib::new(project.source_dir.join(cdylib_path)),
        features,
        dependencies: Map::new(),
        workspace: Some(Workspace {}),
        // Within a workspace, only the [patch] and [replace] sections in
        // the workspace root's Cargo.toml are applied by Cargo.
        patch: workspace_manifest.patch,
        replace: workspace_manifest.replace,
    };

    manifest.dependencies.extend(source_manifest.dependencies);
    manifest
        .dependencies
        .extend(source_manifest.dev_dependencies);
    manifest.dependencies.insert(
        crate_name.to_owned(),
        Dependency {
            version: None,
            path: Some(project.source_dir.clone()),
            default_features: false,
            features: Vec::new(),
            rest: Map::new(),
        },
    );

    Ok(manifest)
}

fn make_config() -> Config {
    Config {
        build: Build {
            rustflags: rustflags::make_vec(),
        },
    }
}

fn check_exists(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }
    match File::open(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::Open(path.to_owned(), err)),
    }
}

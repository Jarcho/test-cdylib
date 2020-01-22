use std::env;
use std::fmt::{self, Display};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Cargo(io::Error),
    CargoFail,
    Io(io::Error),
    Metadata(serde_json::Error),
    Open(PathBuf, io::Error),
    PkgName(env::VarError),
    ProjectDir,
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    Json(serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            Cargo(e) => write!(f, "failed to execute cargo: {}", e),
            CargoFail => write!(f, "cargo reported an error"),
            Io(e) => e.fmt(f),
            Metadata(e) => write!(f, "failed to read cargo metadata: {}", e),
            Open(path, e) => write!(f, "{}: {}", path.display(), e),
            PkgName(e) => write!(f, "failed to detect CARGO_PKG_NAME: {}", e),
            ProjectDir => write!(f, "failed to determine name of project dir"),
            TomlDe(e) => e.fmt(f),
            TomlSer(e) => e.fmt(f),
            Json(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlDe(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlSer(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

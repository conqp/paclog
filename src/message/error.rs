use std::fmt::Display;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MissingField {
    PackageAndVersion,
    Package,
    Version,
    OldVersion,
    NewVersion,
}

impl Display for MissingField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PackageAndVersion => write!(f, "missing package and version"),
            Self::Package => write!(f, "missing package"),
            Self::Version => write!(f, "missing version"),
            Self::OldVersion => write!(f, "missing old version"),
            Self::NewVersion => write!(f, "missing new version"),
        }
    }
}

impl std::error::Error for MissingField {}

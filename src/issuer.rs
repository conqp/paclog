/// Issuer of the log entry.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Issuer {
    /// Entry was issued by `ALPM`.
    Alpm,
    /// Entry was issued by `ALPM Scriptlet`.
    AlpmScriptlet,
    /// Entry was issued by `pacman`.
    Pacman,
    /// Entry was issued by something else.
    Other(String),
}

impl From<String> for Issuer {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ALPM" => Self::Alpm,
            "ALPM-SCRIPTLET" => Self::AlpmScriptlet,
            "PACMAN" => Self::Pacman,
            _ => Self::Other(s),
        }
    }
}

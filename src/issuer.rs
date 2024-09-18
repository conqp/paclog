#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Issuer {
    Alpm,
    AlpmScriptlet,
    Pacman,
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

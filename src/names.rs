use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug)]
pub enum ErrorReason {
    Scope,
    Name,
}

impl Display for ErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorReason::Scope => write!(f, "scope"),
            ErrorReason::Name => write!(f, "name"),
        }
    }
}

#[derive(
    Debug, DeserializeFromStr, SerializeDisplay, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct PackageName(String, String);

impl FromStr for PackageName {
    type Err = errors::PackageNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (scope, name) = s
            .split_once('/')
            .ok_or(Self::Err::InvalidFormat(s.to_string()))?;

        for (reason, part) in [(ErrorReason::Scope, scope), (ErrorReason::Name, name)] {
            if part.len() < 3 || part.len() > 32 {
                return Err(Self::Err::InvalidLength(reason, part.to_string()));
            }

            if part.chars().all(|c| c.is_ascii_digit()) {
                return Err(Self::Err::OnlyDigits(reason, part.to_string()));
            }

            if part.starts_with('_') || part.ends_with('_') {
                return Err(Self::Err::PrePostfixUnderscore(reason, part.to_string()));
            }

            if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(Self::Err::InvalidCharacters(reason, part.to_string()));
            }
        }

        Ok(Self(scope.to_string(), name.to_string()))
    }
}

impl Display for PackageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl PackageName {
    pub fn as_str(&self) -> (&str, &str) {
        (&self.0, &self.1)
    }

    pub fn escaped(&self) -> String {
        format!("{}+{}", self.0, self.1)
    }
}

#[derive(
    Debug, DeserializeFromStr, SerializeDisplay, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum PackageNames {
    Pesde(PackageName),
}

impl PackageNames {
    pub fn as_str(&self) -> (&str, &str) {
        match self {
            PackageNames::Pesde(name) => name.as_str(),
        }
    }

    pub fn escaped(&self) -> String {
        match self {
            PackageNames::Pesde(name) => name.escaped(),
        }
    }
}

impl Display for PackageNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageNames::Pesde(name) => write!(f, "{name}"),
        }
    }
}

impl FromStr for PackageNames {
    type Err = errors::PackageNamesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(name) = PackageName::from_str(s) {
            Ok(PackageNames::Pesde(name))
        } else {
            Err(errors::PackageNamesError::InvalidPackageName(s.to_string()))
        }
    }
}

pub mod errors {
    use thiserror::Error;

    use crate::names::ErrorReason;

    #[derive(Debug, Error)]
    pub enum PackageNameError {
        #[error("package name `{0}` is not in the format `scope/name`")]
        InvalidFormat(String),

        #[error("package {0} `{1}` contains characters outside a-z, 0-9, and _")]
        InvalidCharacters(ErrorReason, String),

        #[error("package {0} `{1}` contains only digits")]
        OnlyDigits(ErrorReason, String),

        #[error("package {0} `{1}` starts or ends with an underscore")]
        PrePostfixUnderscore(ErrorReason, String),

        #[error("package {0} `{1}` is not within 3-32 characters long")]
        InvalidLength(ErrorReason, String),
    }

    #[derive(Debug, Error)]
    #[non_exhaustive]
    pub enum PackageNamesError {
        #[error("invalid package name {0}")]
        InvalidPackageName(String),
    }
}
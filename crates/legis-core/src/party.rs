use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Party {
    Democratic,
    Republican,
    Independent,
    Other(String),
}

impl FromStr for Party {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Democratic" => Self::Democratic,
            "Republican" => Self::Republican,
            "Independent" => Self::Independent,
            other => Self::Other(other.to_owned()),
        })
    }
}

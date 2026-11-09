use std::fmt;
use std::str::FromStr;

use jiff::civil::Date;

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("invalid bioguide ID {0:?}: expected one uppercase letter followed by six digits")]
    Bioguide(String),
    #[error("invalid bill type {0:?}")]
    BillType(String),
    #[error("invalid bill ID {0:?}: expected <congress>-<type>-<number>")]
    BillId(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BioguideId(String);

impl fmt::Display for BioguideId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for BioguideId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        let valid =
            b.len() == 7 && b[0].is_ascii_uppercase() && b[1..].iter().all(u8::is_ascii_digit);
        if valid {
            Ok(Self(s.to_owned()))
        } else {
            Err(IdError::Bioguide(s.to_owned()))
        }
    }
}

impl AsRef<str> for BioguideId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Congress(u16);

impl fmt::Display for Congress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Congress {
    pub fn new(n: u16) -> Option<Self> {
        (n >= 1).then_some(Self(n))
    }

    pub fn get(self) -> u16 {
        self.0
    }

    pub fn for_date(date: Date) -> Self {
        // A Congress starts January 3 of each odd year. Only correct from 1935
        // on: before the 20th Amendment, Congresses began March 4. Dates before
        // the first Congress (1789) clamp to 1.
        let year = i32::from(date.year());
        let start_year = if year % 2 == 0 {
            year - 1
        } else if date.month() == 1 && date.day() < 3 {
            year - 2
        } else {
            year
        };
        let n = ((start_year - 1789) / 2 + 1).max(1);
        Self(u16::try_from(n).unwrap_or(u16::MAX))
    }

    pub fn current(today: Date) -> Self {
        Self::for_date(today)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BillType {
    Hr,
    Hjres,
    Hconres,
    Hres,
    S,
    Sjres,
    Sconres,
    Sres,
}

impl fmt::Display for BillType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Hr => "H.R.",
            Self::Hjres => "H.J.Res.",
            Self::Hconres => "H.Con.Res.",
            Self::Hres => "H.Res.",
            Self::S => "S.",
            Self::Sjres => "S.J.Res.",
            Self::Sconres => "S.Con.Res.",
            Self::Sres => "S.Res.",
        })
    }
}

impl BillType {
    pub const ALL: [Self; 8] = [
        Self::Hr,
        Self::Hjres,
        Self::Hconres,
        Self::Hres,
        Self::S,
        Self::Sjres,
        Self::Sconres,
        Self::Sres,
    ];

    pub fn as_path(self) -> &'static str {
        match self {
            Self::Hr => "hr",
            Self::Hjres => "hjres",
            Self::Hconres => "hconres",
            Self::Hres => "hres",
            Self::S => "s",
            Self::Sjres => "sjres",
            Self::Sconres => "sconres",
            Self::Sres => "sres",
        }
    }
}

impl FromStr for BillType {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|t| t.as_path().eq_ignore_ascii_case(s))
            .ok_or_else(|| IdError::BillType(s.to_owned()))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BillId {
    pub congress: Congress,
    pub kind: BillType,
    pub number: u32,
}

impl fmt::Display for BillId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            self.congress,
            self.kind.as_path(),
            self.number
        )
    }
}

impl FromStr for BillId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = || {
            let (congress, rest) = s.split_once('-')?;
            let (kind, number) = rest.split_once('-')?;
            Some(Self {
                congress: Congress::new(congress.parse().ok()?)?,
                kind: kind.parse().ok()?,
                number: number.parse().ok()?,
            })
        };
        parse().ok_or_else(|| IdError::BillId(s.to_owned()))
    }
}

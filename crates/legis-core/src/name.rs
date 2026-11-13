use std::str::FromStr;

const SUFFIXES: [&str; 5] = ["Jr.", "Sr.", "II", "III", "IV"];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    pub last: String,
    pub first: String,
    pub suffix: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid name {0:?}: expected \"Last, First\"")]
pub struct NameError(String);

impl FromStr for Name {
    type Err = NameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (last, rest) = s.split_once(',').ok_or_else(|| NameError(s.to_owned()))?;
        let (first, suffix) = match rest.split_once(',') {
            Some((first, suffix)) => (first, Some(squash(suffix))),
            None => (rest, None),
        };

        let last = squash(last);
        let mut first = squash(first);
        let mut suffix = suffix.filter(|s| !s.is_empty());

        if suffix.is_none()
            && let Some((head, tail)) = first.rsplit_once(' ')
            && SUFFIXES.contains(&tail)
        {
            suffix = Some(tail.to_owned());
            first = head.to_owned();
        }

        if last.is_empty() || first.is_empty() {
            return Err(NameError(s.to_owned()));
        }
        Ok(Self {
            last,
            first,
            suffix,
        })
    }
}

fn squash(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

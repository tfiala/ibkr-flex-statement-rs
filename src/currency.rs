use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Currency {
    BASE,
    CAD,
    USD,
}

impl<'a> TryFrom<&'a str> for Currency {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        match s {
            "BASE_SUMMARY" => Ok(Currency::BASE),
            "CAD" => Ok(Currency::CAD),
            "USD" => Ok(Currency::USD),
            _ => Err(anyhow::Error::msg(format!("unknown currency {}", s))),
        }
    }
}

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum AssetCategory {
    Stock,
    // Bond,
    // MutualFund,
    // Option,
    // Future,
    // Forex,
    // Crypto,
}

impl<'a> TryFrom<&'a str> for AssetCategory {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        match s {
            "STK" => Ok(Self::Stock),
            _ => Err(anyhow::Error::msg(format!(
                "unsupported asset category {}",
                s
            ))),
        }
    }
}

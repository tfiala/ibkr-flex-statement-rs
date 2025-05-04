use crate::node_utils::NodeWrapper;
use crate::statement_section::StatementSection;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum PositionSide {
    Long,
    Short,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountInfo {
    pub account_id: String,
}

impl<'a> TryFrom<&'a str> for PositionSide {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        match s {
            "Long" => Ok(Self::Long),
            "Short" => Ok(Self::Short),
            _ => Err(anyhow::Error::msg(format!("unknown position side {}", s))),
        }
    }
}

impl StatementSection for AccountInfo {
    fn from_node(node: &NodeWrapper) -> Result<AccountInfo> {
        Ok(AccountInfo {
            account_id: node.get_attribute("accountId")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;
    use anyhow::Result;

    const PARTIAL_STATEMENT_EXAMPLE: &str = r##"
        <FlexQueryResponse queryName="example-query" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-04-25" toDate="2025-04-25" period="LastBusinessDay" whenGenerated="2025-04-26;13:34:28 EDT">
                    <AccountInformation 
                        accountId="U1234567"
                        accountType="Individual"
                        customerType="Individual"
                        accountCapabilities="Portfolio Margin"
                        tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
                    <AccountSummary accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" accountBaseCurrency="USD" accountBaseCurrencySymbol="$" accountBaseCurrencyRate="1.0" accountBaseCurrencyRateDateTime="2025-04-26;13:34:28 EDT" accountBaseCurrencyRateSource="IBKR" accountBaseCurrencyRateSourceDescription="IBKR" accountBaseCurrencyRateSourceDateTime="2025-04-26;13:34:28 EDT" />
                </FlexStatement>
            </FlexStatements>
        </FlexQueryResponse>
        "##;

    #[test]
    fn account_info_parses() -> Result<()> {
        let statements = Parser::new()?.parse_flex_query_response(PARTIAL_STATEMENT_EXAMPLE)?;
        assert_eq!(statements.len(), 1);
        let result = &statements[0];

        assert_eq!(
            result.account_info,
            AccountInfo {
                account_id: "U1234567".to_string(),
            }
        );
        Ok(())
    }
}

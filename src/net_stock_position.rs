use crate::asset_category::AssetCategory;
use crate::currency::Currency;
use crate::node_utils::NodeWrapper;
use crate::statement_section::StatementSection;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct NetStockPosition {
    pub account_id: String,
    pub asset_category: AssetCategory,
    pub conid: u32,
    pub currency: Currency,
    pub listing_exchange: String,
    pub net_shares: f64,
    pub ticker: String,
}

impl StatementSection for NetStockPosition {
    fn from_node(node: &NodeWrapper) -> Result<NetStockPosition> {
        Ok(NetStockPosition {
            account_id: node.get_attribute("accountId")?,
            asset_category: AssetCategory::try_from(node.node.attribute("assetCategory").unwrap())?,
            conid: node.parse_attribute("conid")?,
            currency: Currency::try_from(node.node.attribute("currency").unwrap())?,
            net_shares: node.parse_attribute("netShares")?,
            listing_exchange: node.get_attribute("listingExchange")?,
            ticker: node.get_attribute("symbol")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;
    use crate::currency::Currency;
    use anyhow::Result;

    const FULL_STATEMENT_EXAMPLE: &str = r##"
        <FlexQueryResponse queryName="example-query" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-04-25" toDate="2025-04-25" period="LastBusinessDay" whenGenerated="2025-04-26;13:34:28 EDT">
                    <AccountInformation accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
                    <NetStockPositionSummary>
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="GRPN" conid="426480582" listingExchange="NASDAQ" netShares="3000" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="META" conid="107113386" listingExchange="NASDAQ" netShares="800" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="NFLX" conid="15124833" listingExchange="NASDAQ" netShares="400" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="PLTR" conid="444857009" listingExchange="NASDAQ" netShares="3100" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TQQQ" conid="72539702" listingExchange="NASDAQ" netShares="34100" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TSLA" conid="76792991" listingExchange="NASDAQ" netShares="1500" />
                        <NetStockPosition 
                            accountId="U1234567"
                            currency="USD"
                            assetCategory="STK"
                            symbol="TTWO"
                            conid="6478131"
                            listingExchange="NASDAQ"
                            netShares="500" />
                    </NetStockPositionSummary>
                </FlexStatement>
            </FlexStatements>
         </FlexQueryResponse>
        "##;

    #[test]
    fn net_stock_positions_parse() -> Result<()> {
        let result = Parser::new()?.parse_statement_contents(FULL_STATEMENT_EXAMPLE)?;

        // Ensure we got two equity summaries.
        assert_eq!(result.net_stock_positions.len(), 7);

        assert_eq!(
            result.net_stock_positions[6],
            NetStockPosition {
                account_id: "U1234567".to_string(),
                asset_category: AssetCategory::Stock,
                conid: 6478131,
                currency: Currency::USD,
                listing_exchange: "NASDAQ".to_string(),
                net_shares: 500.0,
                ticker: "TTWO".to_string(),
            }
        );
        Ok(())
    }
}

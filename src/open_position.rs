use super::currency::Currency;
use super::time_utils;
use crate::asset_category::AssetCategory;
use crate::node_utils::NodeWrapper;
use crate::statement_section::StatementSection;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum PositionSide {
    Long,
    Short,
}

#[derive(Debug, PartialEq)]
pub struct OpenPosition {
    pub account_id: String,
    pub asset_category: AssetCategory,
    pub conid: u32,
    pub cost_basis_price: f64,
    pub fifo_pnl_unrealized: f64,
    pub currency: Currency,
    pub listing_exchange: String,
    pub mark_price: f64,
    pub open_quantity: f64,
    pub position_value: f64,
    pub timestamp_eod_ms: i64,
    pub ticker: String,
    pub side: PositionSide,
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

impl StatementSection for OpenPosition {
    fn from_node(node: &NodeWrapper) -> Result<OpenPosition> {
        Ok(OpenPosition {
            account_id: node.get_attribute("accountId")?,
            asset_category: AssetCategory::try_from(node.node.attribute("assetCategory").unwrap())?,
            conid: node.parse_attribute("conid")?,
            cost_basis_price: node.parse_attribute("costBasisPrice")?,
            currency: Currency::try_from(node.node.attribute("currency").unwrap())?,
            fifo_pnl_unrealized: node.parse_attribute("fifoPnlUnrealized")?,
            listing_exchange: node.get_attribute("listingExchange")?,
            mark_price: node.parse_attribute("markPrice")?,
            open_quantity: node.parse_attribute("position")?,
            position_value: node.parse_attribute("positionValue")?,
            side: PositionSide::try_from(node.node.attribute("side").unwrap())?,
            ticker: node.get_attribute("symbol")?,
            timestamp_eod_ms: time_utils::trading_eod_after_hours_timestamp_ms(
                node.node.attribute("reportDate").unwrap(),
            )?,
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
                <FlexStatement accountId="U2418904" fromDate="2025-04-25" toDate="2025-04-25" period="LastBusinessDay" whenGenerated="2025-04-26;13:34:28 EDT">
                    <AccountInformation accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
                    <OpenPositions>
                        <OpenPosition accountId="U2418904" currency="USD" assetCategory="STK" symbol="GRPN" conid="426480582" listingExchange="NASDAQ" reportDate="2025-04-25" position="3000" markPrice="19.89" positionValue="59670" openPrice="20.153441225" costBasisPrice="20.153441225" percentOfNAV="1.63" fifoPnlUnrealized="-790.323674" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U2418904" currency="USD" assetCategory="STK" symbol="META" conid="107113386" listingExchange="NASDAQ" reportDate="2025-04-25" position="800" markPrice="547.27" positionValue="437816" openPrice="542.020354354" costBasisPrice="542.020354354" percentOfNAV="11.95" fifoPnlUnrealized="4199.716517" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U2418904" currency="USD" assetCategory="STK" symbol="NFLX" conid="15124833" listingExchange="NASDAQ" reportDate="2025-04-25" position="400" markPrice="1101.53" positionValue="440612" openPrice="1056.32548211" costBasisPrice="1056.32548211" percentOfNAV="12.02" fifoPnlUnrealized="18081.807156" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U2418904" currency="USD" assetCategory="STK" symbol="PLTR" conid="444857009" listingExchange="NASDAQ" reportDate="2025-04-25" position="3100" markPrice="112.78" positionValue="349618" openPrice="104.761973398" costBasisPrice="104.761973398" percentOfNAV="9.54" fifoPnlUnrealized="24855.882465" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U2418904" currency="USD" assetCategory="STK" symbol="TQQQ" conid="72539702" listingExchange="NASDAQ" reportDate="2025-04-25" position="34100" markPrice="53.86" positionValue="1836626" openPrice="53.776784497" costBasisPrice="53.776784497" percentOfNAV="50.12" fifoPnlUnrealized="2837.648645" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U2418904" currency="USD" assetCategory="STK" symbol="TSLA" conid="76792991" listingExchange="NASDAQ" reportDate="2025-04-25" position="1500" markPrice="284.95" positionValue="427425" openPrice="262.984320092" costBasisPrice="262.984320092" percentOfNAV="11.66" fifoPnlUnrealized="32948.519862" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition 
                            accountId="U2418904"
                            currency="USD"
                            assetCategory="STK"
                            symbol="TTWO"
                            conid="6478131"
                            listingExchange="NASDAQ"
                            reportDate="2025-04-25"
                            position="500"
                            markPrice="225.38"
                            positionValue="112690"
                            openPrice="217.200032892"
                            costBasisPrice="217.200032892"
                            percentOfNAV="3.08"
                            fifoPnlUnrealized="4089.983554"
                            side="Long"
                            openDateTime=""
                            holdingPeriodDateTime=""
                            accruedInt=""
                            commodityType="" />
                    </OpenPositions>
                </FlexStatement>
            </FlexStatements>
         </FlexQueryResponse>
        "##;

    #[test]
    fn open_positions_parse() -> Result<()> {
        let result = Parser::new()?.parse_statement_contents(FULL_STATEMENT_EXAMPLE)?;

        // Ensure we got two equity summaries.
        assert_eq!(result.open_positions.len(), 7);

        assert_eq!(
            result.open_positions[6],
            OpenPosition {
                account_id: "U2418904".to_string(),
                asset_category: AssetCategory::Stock,
                conid: 6478131,
                cost_basis_price: 217.200032892,
                fifo_pnl_unrealized: 4089.983554,
                currency: Currency::USD,
                listing_exchange: "NASDAQ".to_string(),
                mark_price: 225.38,
                open_quantity: 500.0,
                position_value: 112690.0,
                timestamp_eod_ms: result.open_positions[6].timestamp_eod_ms,
                ticker: "TTWO".to_string(),
                side: PositionSide::Long
            }
        );
        Ok(())
    }
}

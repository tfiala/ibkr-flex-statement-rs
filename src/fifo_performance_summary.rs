use crate::statement_section::StatementSection;

use super::node_utils::NodeWrapper;
use super::time_utils;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct FIFOPerformanceSummary {
    pub account_id: String,
    pub timestamp_eod_ms: i64,

    pub ticker: Option<String>,
    pub conid: Option<u32>,
    pub listing_exchange: Option<String>,

    pub realized_st_profit: f64,
    pub realized_st_loss: f64,
    pub unrealized_st_profit: f64,
    pub unrealized_st_loss: f64,

    pub realized_lt_profit: f64,
    pub realized_lt_loss: f64,
    pub unrealized_lt_profit: f64,
    pub unrealized_lt_loss: f64,

    pub total_realized_pnl: f64,
    pub total_fifo_pnl: f64,
}

impl StatementSection for FIFOPerformanceSummary {
    fn from_node(node: &NodeWrapper) -> Result<FIFOPerformanceSummary> {
        Ok(FIFOPerformanceSummary {
            account_id: node.get_attribute("accountId")?,
            timestamp_eod_ms: time_utils::trading_eod_after_hours_timestamp_ms(
                node.node.attribute("reportDate").unwrap(),
            )?,

            ticker: node.get_attribute_opt("symbol"),
            conid: node.parse_attribute_opt("conid")?,
            listing_exchange: node.get_attribute_opt("listingExchange"),

            realized_st_profit: node.parse_attribute("realizedSTProfit")?,
            realized_st_loss: node.parse_attribute("realizedSTLoss")?,
            unrealized_st_profit: node.parse_attribute("unrealizedSTProfit")?,
            unrealized_st_loss: node.parse_attribute("unrealizedSTLoss")?,

            realized_lt_profit: node.parse_attribute("realizedLTProfit")?,
            realized_lt_loss: node.parse_attribute("realizedLTLoss")?,
            unrealized_lt_profit: node.parse_attribute("unrealizedLTProfit")?,
            unrealized_lt_loss: node.parse_attribute("unrealizedLTLoss")?,

            total_realized_pnl: node.parse_attribute("totalRealizedPnl")?,
            total_fifo_pnl: node.parse_attribute("totalFifoPnl")?,
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
                    <AccountInformation accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
                    <FIFOPerformanceSummaryInBase>
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="ARGX" conid="276343981" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="-636.4413056" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="-636.4413056" unrealizedProfit="0" unrealizedLoss="0" unrealizedSTProfit="0" unrealizedSTLoss="0" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="-636.4413056" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="GEO" conid="158655765" listingExchange="NYSE" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="-1375.50915991" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="-1375.50915991" unrealizedProfit="0" unrealizedLoss="0" unrealizedSTProfit="0" unrealizedSTLoss="0" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="-1375.50915991" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="GRPN" conid="426480582" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="1607.307677" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="1607.307677" unrealizedProfit="77.771818" unrealizedLoss="-868.095492" unrealizedSTProfit="77.771818" unrealizedSTLoss="-868.095492" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="816.984003" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="META" conid="107113386" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="0" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="0" unrealizedProfit="8598.681628" unrealizedLoss="-4398.965111" unrealizedSTProfit="8598.681628" unrealizedSTLoss="-4398.965111" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="4199.716517" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="NFLX" conid="15124833" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="0" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="0" unrealizedProfit="18081.807156" unrealizedLoss="0" unrealizedSTProfit="18081.807156" unrealizedSTLoss="0" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="18081.807156" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="PLTR" conid="444857009" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="0" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="0" unrealizedProfit="24855.882465" unrealizedLoss="0" unrealizedSTProfit="24855.882465" unrealizedSTLoss="0" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="24855.882465" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="TQQQ" conid="72539702" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="0" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="0" unrealizedProfit="42178.328575" unrealizedLoss="-39340.67993" unrealizedSTProfit="42178.328575" unrealizedSTLoss="-39340.67993" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="2837.648645" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="TSLA" conid="76792991" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="199.59291494" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="199.59291494" unrealizedProfit="33175.116277" unrealizedLoss="-226.596415" unrealizedSTProfit="33175.116277" unrealizedSTLoss="-226.596415" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="33148.11277694" />
                        <FIFOPerformanceSummaryUnderlying
                            accountId="U1234567"
                            assetCategory="STK"
                            symbol="TTWO"
                            conid="6478131"
                            listingExchange="NASDAQ"
                            reportDate="2025-04-25"
                            realizedSTProfit="0"
                            realizedSTLoss="0"
                            realizedLTProfit="0"
                            realizedLTLoss="0"
                            totalRealizedPnl="0"
                            unrealizedProfit="4089.983554"
                            unrealizedLoss="0"
                            unrealizedSTProfit="4089.983554"
                            unrealizedSTLoss="0"
                            unrealizedLTProfit="0"
                            unrealizedLTLoss="0"
                            totalFifoPnl="4089.983554" />

                        <FIFOPerformanceSummaryUnderlying
                            accountId="U1234567"
                            assetCategory=""
                            symbol=""
                            conid=""
                            listingExchange=""
                            reportDate="2025-04-25"
                            realizedSTProfit="0"
                            realizedSTLoss="-205.04987357"
                            realizedLTProfit="0"
                            realizedLTLoss="0"
                            totalRealizedPnl="-205.04987357"
                            unrealizedProfit="131057.571473"
                            unrealizedLoss="-44834.337024864"
                            unrealizedSTProfit="131057.571473"
                            unrealizedSTLoss="-44834.337024864"
                            unrealizedLTProfit="0"
                            unrealizedLTLoss="0"
                            totalFifoPnl="86018.184574566" />
                    </FIFOPerformanceSummaryInBase>
                </FlexStatement>
            </FlexStatements>
         </FlexQueryResponse>
    "##;

    #[test]
    fn fifo_performance_summaries_parse() -> Result<()> {
        let statements = Parser::new()?.parse_flex_query_response(PARTIAL_STATEMENT_EXAMPLE)?;
        assert_eq!(statements.len(), 1);
        let result = &statements[0];

        // Ensure we got two equity summaries.
        assert_eq!(result.fifo_performance_summaries.len(), 10);

        assert_eq!(
            result.fifo_performance_summaries[8],
            FIFOPerformanceSummary {
                account_id: "U1234567".to_string(),
                ticker: Some("TTWO".to_string()),
                conid: Some(6478131),
                listing_exchange: Some("NASDAQ".to_string()),
                timestamp_eod_ms: result.fifo_performance_summaries[8].timestamp_eod_ms,
                realized_st_profit: 0.0,
                realized_st_loss: 0.0,
                unrealized_st_profit: 4089.983554,
                unrealized_st_loss: 0.0,
                realized_lt_profit: 0.0,
                realized_lt_loss: 0.0,
                unrealized_lt_profit: 0.0,
                unrealized_lt_loss: 0.0,
                total_realized_pnl: 0.0,
                total_fifo_pnl: 4089.983554,
            }
        );

        assert_eq!(
            result.fifo_performance_summaries[9],
            FIFOPerformanceSummary {
                account_id: "U1234567".to_string(),
                ticker: None,
                conid: None,
                listing_exchange: None,
                timestamp_eod_ms: result.fifo_performance_summaries[9].timestamp_eod_ms,
                realized_st_profit: 0.0,
                realized_st_loss: -205.04987357,
                unrealized_st_profit: 131057.571473,
                unrealized_st_loss: -44834.337024864,
                realized_lt_profit: 0.0,
                realized_lt_loss: 0.0,
                unrealized_lt_profit: 0.0,
                unrealized_lt_loss: 0.0,
                total_realized_pnl: -205.04987357,
                total_fifo_pnl: 86018.184574566,
            }
        );
        Ok(())
    }
}

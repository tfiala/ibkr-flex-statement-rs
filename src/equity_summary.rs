use super::currency::Currency;
use super::time_utils;
use crate::node_utils::NodeWrapper;
use crate::statement_section::StatementSection;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct EquitySummary {
    pub account_id: String,
    pub cash_balance: f64,
    pub cash_balance_long: f64,
    pub cash_balance_short: f64,
    pub currency: Currency,
    pub interest_accrual_mtd: f64,
    pub interest_accrual_mtd_long: f64,
    pub interest_accrual_mtd_short: f64,
    pub stock_balance: f64,
    pub stock_balance_long: f64,
    pub stock_balance_short: f64,
    pub timestamp_eod_ms: i64,
}

impl StatementSection for EquitySummary {
    fn from_node(node: &NodeWrapper) -> Result<EquitySummary> {
        Ok(EquitySummary {
            account_id: node.get_attribute("accountId")?,
            cash_balance: node.parse_attribute("cash")?,
            cash_balance_long: node.parse_attribute("cashLong")?,
            cash_balance_short: node.parse_attribute("cashShort")?,
            currency: Currency::try_from(node.node.attribute("currency").unwrap())?,
            interest_accrual_mtd: node.parse_attribute("interestAccruals")?,
            interest_accrual_mtd_long: node.parse_attribute("interestAccrualsLong")?,
            interest_accrual_mtd_short: node.parse_attribute("interestAccrualsShort")?,
            stock_balance: node.parse_attribute("stock")?,
            stock_balance_long: node.parse_attribute("stockLong")?,
            stock_balance_short: node.parse_attribute("stockShort")?,
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

    const PARTIAL_STATEMENT_EXAMPLE: &str = r##"
        <FlexQueryResponse queryName="example-query" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-04-25" toDate="2025-04-25" period="LastBusinessDay" whenGenerated="2025-04-26;13:34:28 EDT">
                    <AccountInformation accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
                    <EquitySummaryInBase>
                        <EquitySummaryByReportDateInBase accountId="U1234567" currency="USD" cash="-1755658.753685009" cashLong="0.000832235" cashShort="-1755658.754517244" commodities="0" commoditiesLong="0" commoditiesShort="0" dividendAccruals="0" dividendAccrualsLong="0" dividendAccrualsShort="0" interestAccruals="1292.18" interestAccrualsLong="1591.34" interestAccrualsShort="-299.16" stock="3441241" stockLong="3441241" stockShort="0" funds="0" fundsLong="0" fundsShort="0" brokerInterestAccrualsComponent="186.32" brokerInterestAccrualsComponentLong="485.48" brokerInterestAccrualsComponentShort="-299.16" brokerFeesAccrualsComponent="0" brokerFeesAccrualsComponentLong="0" brokerFeesAccrualsComponentShort="0" total="1686874.426314991" totalLong="3442832.340832235" totalShort="-1755957.914517244" reportDate="2025-04-24" />
                        <EquitySummaryByReportDateInBase
                            accountId="U1234567"
                            currency="USD"
                            cash="-1856140.99825062"
                            cashLong="0.000832132"
                            cashShort="-1856140.999082752"
                            commodities="0"
                            commoditiesLong="0"
                            commoditiesShort="0"
                            dividendAccruals="0"
                            dividendAccrualsLong="0"
                            dividendAccrualsShort="0"
                            interestAccruals="1051.42"
                            interestAccrualsLong="1591.34"
                            interestAccrualsShort="-539.92"
                            stock="3664457"
                            stockLong="3664457"
                            stockShort="0"
                            funds="0"
                            fundsLong="0"
                            fundsShort="0"
                            brokerInterestAccrualsComponent="-54.44"
                            brokerInterestAccrualsComponentLong="485.48"
                            brokerInterestAccrualsComponentShort="-539.92"
                            brokerFeesAccrualsComponent="0"
                            brokerFeesAccrualsComponentLong="0"
                            brokerFeesAccrualsComponentShort="0"
                            total="1809367.421749379"
                            totalLong="3666048.340832131"
                            totalShort="-1856680.919082752"
                            reportDate="2025-04-25" />
                    </EquitySummaryInBase>
                </FlexStatement>
            </FlexStatements>
         </FlexQueryResponse>
        "##;

    #[test]
    fn equity_summaries_parse() -> Result<()> {
        let statements = Parser::new()?.parse_flex_query_response(PARTIAL_STATEMENT_EXAMPLE)?;
        assert_eq!(statements.len(), 1);
        let result = &statements[0];

        // Ensure we got two equity summaries.
        assert_eq!(result.equity_summaries.len(), 2);

        assert_eq!(
            result.equity_summaries[1],
            EquitySummary {
                account_id: "U1234567".to_string(),
                cash_balance: -1856140.99825062,
                cash_balance_long: 0.000832132,
                cash_balance_short: -1856140.999082752,
                currency: Currency::USD,
                interest_accrual_mtd: 1051.42,
                interest_accrual_mtd_long: 1591.34,
                interest_accrual_mtd_short: -539.92,
                stock_balance: 3664457.0,
                stock_balance_long: 3664457.0,
                stock_balance_short: 0.0,
                timestamp_eod_ms: result.equity_summaries[1].timestamp_eod_ms,
            }
        );
        Ok(())
    }
}

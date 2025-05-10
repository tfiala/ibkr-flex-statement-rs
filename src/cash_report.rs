use crate::currency::Currency;
use crate::node_utils::NodeWrapper;
use crate::statement_section::StatementSection;
use crate::time_utils;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct CashReport {
    pub account_id: String,
    pub currency: Currency,
    pub start_timestamp_ms: i64,
    pub end_timestamp_ms: i64,

    pub starting_cash: f64,
    pub ending_cash: f64,
    pub ending_settled_cash: f64,

    pub net_trade_purchases: f64,
    pub net_trade_sales: f64,

    pub commissions: f64,
    pub commissions_mtd: Option<f64>,
    pub commissions_ytd: Option<f64>,

    pub other_fees: f64,
    pub other_fees_mtd: Option<f64>,
    pub other_fees_ytd: Option<f64>,

    pub dividends: f64,
    pub dividends_mtd: Option<f64>,
    pub dividends_ytd: Option<f64>,

    pub interest: f64,
    pub interest_mtd: Option<f64>,
    pub interest_ytd: Option<f64>,

    pub deposits: f64,
    pub deposits_mtd: Option<f64>,
    pub deposits_ytd: Option<f64>,

    pub withdrawals: f64,
    pub withdrawals_mtd: Option<f64>,
    pub withdrawals_ytd: Option<f64>,
}

impl StatementSection for CashReport {
    fn from_node(node: &NodeWrapper) -> Result<CashReport> {
        let start_date_eod_ms_plus_one = time_utils::trading_eod_after_hours_timestamp_ms(
            node.node.attribute("fromDate").unwrap(),
        )?;
        let start_timestamp_ms = start_date_eod_ms_plus_one - (60 * 60 * 24 * 1000) + 1;

        Ok(CashReport {
            account_id: node.get_attribute("accountId")?,
            currency: Currency::try_from(node.node.attribute("currency").unwrap())?,

            starting_cash: node.parse_attribute("startingCash")?,
            ending_cash: node.parse_attribute("endingCash")?,
            ending_settled_cash: node.parse_attribute("endingSettledCash")?,

            net_trade_purchases: node.parse_attribute("netTradesPurchases")?,
            net_trade_sales: node.parse_attribute("netTradesSales")?,

            commissions: node.parse_attribute("commissions")?,
            commissions_mtd: node.parse_attribute_opt("commissionsMTD")?,
            commissions_ytd: node.parse_attribute_opt("commissionsYTD")?,

            other_fees: node.parse_attribute("otherFees")?,
            other_fees_mtd: node.parse_attribute_opt("otherFeesMTD")?,
            other_fees_ytd: node.parse_attribute_opt("otherFeesYTD")?,

            dividends: node.parse_attribute("dividends")?,
            dividends_mtd: node.parse_attribute_opt("dividendsMTD")?,
            dividends_ytd: node.parse_attribute_opt("dividendsYTD")?,

            interest: node.parse_attribute("brokerInterest")?,
            interest_mtd: node.parse_attribute_opt("brokerInterestMTD")?,
            interest_ytd: node.parse_attribute_opt("brokerInterestYTD")?,

            deposits: node.parse_attribute("deposits")?,
            deposits_mtd: node.parse_attribute_opt("depositsMTD")?,
            deposits_ytd: node.parse_attribute_opt("depositsYTD")?,

            withdrawals: node.parse_attribute("withdrawals")?,
            withdrawals_mtd: node.parse_attribute_opt("withdrawalsMTD")?,
            withdrawals_ytd: node.parse_attribute_opt("withdrawalsYTD")?,

            start_timestamp_ms,
            end_timestamp_ms: time_utils::trading_eod_after_hours_timestamp_ms(
                node.node.attribute("toDate").unwrap(),
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
                    <CashReport>
                        <CashReportCurrency
                            accountId="U1234567"
                            currency="USD"
                            fromDate="2025-04-25"
                            toDate="2025-04-25"
                            startingCash="-1755658.754517244"
                            startingCashSec="-1755658.754517244"
                            startingCashCom="0"
                            commissions="-56.26956551"
                            commissionsSec="-56.26956551"
                            commissionsCom="0"
                            commissionsMTD="-11167.4772929"
                            commissionsYTD="-25339.56064716"
                            depositWithdrawals="0"
                            depositWithdrawalsSec="0"
                            depositWithdrawalsCom="0"
                            depositWithdrawalsMTD="0"
                            depositWithdrawalsYTD="1650000"
                            debitCardActivity="0"
                            debitCardActivitySec="0"
                            debitCardActivityCom="0"
                            debitCardActivityMTD="0"
                            debitCardActivityYTD="0"
                            dividends="0"
                            dividendsSec="0"
                            dividendsCom="0"
                            dividendsMTD="0"
                            dividendsYTD="110.7"
                            otherFees="-19.77"
                            otherFeesSec="-19.77"
                            otherFeesCom="0"
                            otherFeesMTD="-121.27"
                            otherFeesYTD="-486.9"
                            otherIncome="0"
                            otherIncomeSec="0"
                            otherIncomeCom="0"
                            otherIncomeMTD="0"
                            otherIncomeYTD="0"
                            netTradesSales="0"
                            netTradesSalesSec="0"
                            netTradesSalesCom="0"
                            netTradesPurchases="0"
                            netTradesPurchasesSec="0"
                            netTradesPurchasesCom="0"
                            endingCash="-1856140.999082752"
                            endingCashSec="-1856140.999082752"
                            endingCashCom="0"
                            endingSettledCash="-1755734.794082752"
                            endingSettledCashSec="-1755734.794082752"
                            endingSettledCashCom="0"
                            brokerInterest="0"
                            brokerInterestSec="0"
                            brokerInterestCom="0"
                            brokerInterestMTD="-545.49"
                            brokerInterestYTD="-1341.59"
                            brokerFees="0"
                            brokerFeesSec="0"
                            brokerFeesCom="0"
                            brokerFeesMTD="0"
                            brokerFeesYTD="0"
                            deposits="0"
                            depositsSec="0"
                            depositsCom="0"
                            depositsMTD="0"
                            depositsYTD="1650000"
                            withdrawals="0"
                            withdrawalsSec="0"
                            withdrawalsCom="0"
                            withdrawalsMTD="0"
                            withdrawalsYTD="0" />
                    </CashReport>

                </FlexStatement>
            </FlexStatements>
         </FlexQueryResponse>
        "##;

    const PARTIAL_STATEMENT_EXAMPLE_NO_MTD_YTD: &str = r##"
        <FlexQueryResponse queryName="some-query" type="AF">
        <FlexStatements count="1">
        <FlexStatement accountId="U1234567" fromDate="2024-01-01" toDate="2024-01-01" period="" whenGenerated="2025-05-04;01:26:45 EDT">
        <AccountInformation accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
        <CashReport>
            <CashReportCurrency accountId="U1234567" currency="BASE_SUMMARY" fromDate="2024-01-01" toDate="2024-01-01" startingCash="1308.407281684" startingCashSec="1308.407281684" startingCashCom="0" commissions="0" commissionsSec="0" commissionsCom="0" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" dividends="0" dividendsSec="0" dividendsCom="0" otherFees="0" otherFeesSec="0" otherFeesCom="0" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" endingCash="1308.407282342" endingCashSec="1308.407282342" endingCashCom="0" endingSettledCash="1308.407282342" endingSettledCashSec="1308.407282342" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" deposits="0" depositsSec="0" depositsCom="0" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" netTradesSales="0" netTradesSalesSec="0" netTradesSalesCom="0" netTradesPurchases="0" netTradesPurchasesSec="0" netTradesPurchasesCom="0" transactionTax="0" transactionTaxSec="0" transactionTaxCom="0" withholdingTax="0" withholdingTaxSec="0" withholdingTaxCom="0" withholdingCollectedTax="0" withholdingCollectedTaxSec="0" withholdingCollectedTaxCom="0" slbNetSecuritiesLentActivity="0" slbNetSecuritiesLentActivitySec="0" slbNetSecuritiesLentActivityCom="0" />
            <CashReportCurrency accountId="U1234567" currency="CAD" fromDate="2024-01-01" toDate="2024-01-01" startingCash="0.001153" startingCashSec="0.001153" startingCashCom="0" commissions="0" commissionsSec="0" commissionsCom="0" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" dividends="0" dividendsSec="0" dividendsCom="0" otherFees="0" otherFeesSec="0" otherFeesCom="0" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" endingCash="0.001153" endingCashSec="0.001153" endingCashCom="0" endingSettledCash="0.001153" endingSettledCashSec="0.001153" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" deposits="0" depositsSec="0" depositsCom="0" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" netTradesSales="0" netTradesSalesSec="0" netTradesSalesCom="0" netTradesPurchases="0" netTradesPurchasesSec="0" netTradesPurchasesCom="0" transactionTax="0" transactionTaxSec="0" transactionTaxCom="0" withholdingTax="0" withholdingTaxSec="0" withholdingTaxCom="0" withholdingCollectedTax="0" withholdingCollectedTaxSec="0" withholdingCollectedTaxCom="0" slbNetSecuritiesLentActivity="0" slbNetSecuritiesLentActivitySec="0" slbNetSecuritiesLentActivityCom="0" />
            <CashReportCurrency accountId="U1234567" currency="USD" fromDate="2024-01-01" toDate="2024-01-01" startingCash="1308.406411423" startingCashSec="1308.406411423" startingCashCom="0" commissions="0" commissionsSec="0" commissionsCom="0" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" dividends="0" dividendsSec="0" dividendsCom="0" otherFees="0" otherFeesSec="0" otherFeesCom="0" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" endingCash="1308.406411423" endingCashSec="1308.406411423" endingCashCom="0" endingSettledCash="1308.406411423" endingSettledCashSec="1308.406411423" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" deposits="0" depositsSec="0" depositsCom="0" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" netTradesSales="0" netTradesSalesSec="0" netTradesSalesCom="0" netTradesPurchases="0" netTradesPurchasesSec="0" netTradesPurchasesCom="0" transactionTax="0" transactionTaxSec="0" transactionTaxCom="0" withholdingTax="0" withholdingTaxSec="0" withholdingTaxCom="0" withholdingCollectedTax="0" withholdingCollectedTaxSec="0" withholdingCollectedTaxCom="0" slbNetSecuritiesLentActivity="0" slbNetSecuritiesLentActivitySec="0" slbNetSecuritiesLentActivityCom="0" />
        </CashReport>
        </FlexStatement>
        </FlexStatements>
        </FlexQueryResponse>
        "##;

    #[test]
    fn cash_reports_parse() -> Result<()> {
        let statements = Parser::new()?.parse_flex_query_response(PARTIAL_STATEMENT_EXAMPLE)?;
        assert_eq!(statements.len(), 1);
        let result = &statements[0];

        // Ensure we got two equity summaries.
        assert_eq!(result.cash_reports.len(), 1);

        assert_eq!(
            result.cash_reports[0],
            CashReport {
                account_id: "U1234567".to_string(),
                currency: Currency::USD,

                starting_cash: -1755658.754517244,
                ending_cash: -1856140.999082752,
                ending_settled_cash: -1755734.794082752,

                commissions: -56.26956551,
                commissions_mtd: Some(-11167.4772929),
                commissions_ytd: Some(-25339.56064716),

                dividends: 0.0,
                dividends_mtd: Some(0.0),
                dividends_ytd: Some(110.7),

                other_fees: -19.77,
                other_fees_mtd: Some(-121.27),
                other_fees_ytd: Some(-486.9),

                net_trade_purchases: 0.0,
                net_trade_sales: 0.0,

                interest: 0.0,
                interest_mtd: Some(-545.49),
                interest_ytd: Some(-1341.59),

                deposits: 0.0,
                deposits_mtd: Some(0.0),
                deposits_ytd: Some(1650000.0),

                withdrawals: 0.0,
                withdrawals_mtd: Some(0.0),
                withdrawals_ytd: Some(0.0),

                start_timestamp_ms: result.cash_reports[0].start_timestamp_ms,
                end_timestamp_ms: result.cash_reports[0].end_timestamp_ms,
            }
        );

        Ok(())
    }

    #[test]
    fn cash_reports_parse_no_mtd_ytd() -> Result<()> {
        let statements =
            Parser::new()?.parse_flex_query_response(PARTIAL_STATEMENT_EXAMPLE_NO_MTD_YTD)?;
        assert_eq!(statements.len(), 1);
        let result = &statements[0];

        // Ensure we got two equity summaries.
        assert_eq!(result.cash_reports.len(), 3);

        assert_eq!(
            result.cash_reports[2],
            CashReport {
                account_id: "U1234567".to_string(),
                currency: Currency::USD,

                starting_cash: 1308.406411423,
                ending_cash: 1308.406411423,
                ending_settled_cash: 1308.406411423,

                commissions: 0.0,
                commissions_mtd: None,
                commissions_ytd: None,

                dividends: 0.0,
                dividends_mtd: None,
                dividends_ytd: None,

                other_fees: 0.0,
                other_fees_mtd: None,
                other_fees_ytd: None,

                net_trade_purchases: 0.0,
                net_trade_sales: 0.0,

                interest: 0.0,
                interest_mtd: None,
                interest_ytd: None,

                deposits: 0.0,
                deposits_mtd: None,
                deposits_ytd: None,

                withdrawals: 0.0,
                withdrawals_mtd: None,
                withdrawals_ytd: None,

                start_timestamp_ms: result.cash_reports[2].start_timestamp_ms,
                end_timestamp_ms: result.cash_reports[2].end_timestamp_ms,
            }
        );

        Ok(())
    }
}

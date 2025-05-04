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

    pub net_trades: Option<f64>,
    pub net_trade_purchases: Option<f64>,
    pub net_trade_sales: Option<f64>,

    pub commissions: f64,
    pub commissions_mtd: f64,
    pub commissions_ytd: f64,

    pub other_fees: f64,
    pub other_fees_mtd: f64,
    pub other_fees_ytd: f64,

    pub dividends: f64,
    pub dividends_mtd: f64,
    pub dividends_ytd: f64,

    pub interest: f64,
    pub interest_mtd: f64,
    pub interest_ytd: f64,

    pub deposits: f64,
    pub deposits_mtd: f64,
    pub deposits_ytd: f64,

    pub withdrawals: f64,
    pub withdrawals_mtd: f64,
    pub withdrawals_ytd: f64,
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

            net_trades: node.parse_attribute_opt("netTrades")?,
            net_trade_purchases: node.parse_attribute_opt("netTradesPurchases")?,
            net_trade_sales: node.parse_attribute_opt("netTradesSales")?,

            commissions: node.parse_attribute("commissions")?,
            commissions_mtd: node.parse_attribute("commissionsMTD")?,
            commissions_ytd: node.parse_attribute("commissionsYTD")?,

            other_fees: node.parse_attribute("otherFees")?,
            other_fees_mtd: node.parse_attribute("otherFeesMTD")?,
            other_fees_ytd: node.parse_attribute("otherFeesYTD")?,

            dividends: node.parse_attribute("dividends")?,
            dividends_mtd: node.parse_attribute("dividendsMTD")?,
            dividends_ytd: node.parse_attribute("dividendsYTD")?,

            interest: node.parse_attribute("brokerInterest")?,
            interest_mtd: node.parse_attribute("brokerInterestMTD")?,
            interest_ytd: node.parse_attribute("brokerInterestYTD")?,

            deposits: node.parse_attribute("deposits")?,
            deposits_mtd: node.parse_attribute("depositsMTD")?,
            deposits_ytd: node.parse_attribute("depositsYTD")?,

            withdrawals: node.parse_attribute("withdrawals")?,
            withdrawals_mtd: node.parse_attribute("withdrawalsMTD")?,
            withdrawals_ytd: node.parse_attribute("withdrawalsYTD")?,

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

    const FULL_STATEMENT_EXAMPLE: &str = r##"
        <FlexQueryResponse queryName="example-query" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-04-25" toDate="2025-04-25" period="LastBusinessDay" whenGenerated="2025-04-26;13:34:28 EDT">
                    <AccountInformation accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
                    <CashReport>
                        <CashReportCurrency accountId="U1234567" currency="BASE_SUMMARY" fromDate="2025-04-25" toDate="2025-04-25" startingCash="-1755658.753685008" startingCashSec="-1755658.753685008" startingCashCom="0" commissions="-56.26956551" commissionsSec="-56.26956551" commissionsCom="0" commissionsMTD="-11167.4772929" commissionsYTD="-25339.56064716" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" depositWithdrawalsMTD="0" depositWithdrawalsYTD="1650000" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" debitCardActivityMTD="0" debitCardActivityYTD="0" dividends="0" dividendsSec="0" dividendsCom="0" dividendsMTD="0" dividendsYTD="110.7" otherFees="-19.77" otherFeesSec="-19.77" otherFeesCom="0" otherFeesMTD="-121.27" otherFeesYTD="-486.9" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" otherIncomeMTD="0" otherIncomeYTD="0" endingCash="-1856140.99825062" endingCashSec="-1856140.99825062" endingCashCom="0" endingSettledCash="-1755734.79325062" endingSettledCashSec="-1755734.79325062" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerInterestMTD="-545.49" brokerInterestYTD="-1341.59" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" brokerFeesMTD="0" brokerFeesYTD="0" deposits="0" depositsSec="0" depositsCom="0" depositsMTD="0" depositsYTD="1650000" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" withdrawalsMTD="0" withdrawalsYTD="0" />
                        <CashReportCurrency accountId="U1234567" currency="CAD" fromDate="2025-04-25" toDate="2025-04-25" startingCash="0.001153" startingCashSec="0.001153" startingCashCom="0" commissions="0" commissionsSec="0" commissionsCom="0" commissionsMTD="0" commissionsYTD="0" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" depositWithdrawalsMTD="0" depositWithdrawalsYTD="0" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" debitCardActivityMTD="0" debitCardActivityYTD="0" dividends="0" dividendsSec="0" dividendsCom="0" dividendsMTD="0" dividendsYTD="0" otherFees="0" otherFeesSec="0" otherFeesCom="0" otherFeesMTD="0" otherFeesYTD="0" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" otherIncomeMTD="0" otherIncomeYTD="0" endingCash="0.001153" endingCashSec="0.001153" endingCashCom="0" endingSettledCash="0.001153" endingSettledCashSec="0.001153" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerInterestMTD="0" brokerInterestYTD="0" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" brokerFeesMTD="0" brokerFeesYTD="0" deposits="0" depositsSec="0" depositsCom="0" depositsMTD="0" depositsYTD="0" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" withdrawalsMTD="0" withdrawalsYTD="0" />
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

    #[test]
    fn cash_reports_parse() -> Result<()> {
        let result = Parser::new()?.parse_statement_contents(FULL_STATEMENT_EXAMPLE)?;

        // Ensure we got two equity summaries.
        assert_eq!(result.cash_reports.len(), 3);

        assert_eq!(
            result.cash_reports[2],
            CashReport {
                account_id: "U1234567".to_string(),
                currency: Currency::USD,

                starting_cash: -1755658.754517244,
                ending_cash: -1856140.999082752,
                ending_settled_cash: -1755734.794082752,

                commissions: -56.26956551,
                commissions_mtd: -11167.4772929,
                commissions_ytd: -25339.56064716,

                dividends: 0.0,
                dividends_mtd: 0.0,
                dividends_ytd: 110.7,

                other_fees: -19.77,
                other_fees_mtd: -121.27,
                other_fees_ytd: -486.9,

                net_trades: None,
                net_trade_purchases: None,
                net_trade_sales: None,

                interest: 0.0,
                interest_mtd: -545.49,
                interest_ytd: -1341.59,

                deposits: 0.0,
                deposits_mtd: 0.0,
                deposits_ytd: 1650000.0,

                withdrawals: 0.0,
                withdrawals_mtd: 0.0,
                withdrawals_ytd: 0.0,

                start_timestamp_ms: result.cash_reports[2].start_timestamp_ms,
                end_timestamp_ms: result.cash_reports[2].end_timestamp_ms,
            }
        );

        Ok(())
    }
}

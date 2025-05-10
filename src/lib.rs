pub mod account_info;
pub mod asset_category;
pub mod cash_report;
pub mod currency;
pub mod equity_summary;
pub mod fifo_performance_summary;
pub mod net_stock_position;
mod node_utils;
pub mod open_position;
pub mod statement_section;
mod time_utils;
pub mod trade;

use account_info::AccountInfo;
use anyhow::Result;
use cash_report::CashReport;
use chrono_tz::Tz;
use equity_summary::EquitySummary;
use fifo_performance_summary::FIFOPerformanceSummary;
use net_stock_position::NetStockPosition;
use node_utils::NodeWrapper;
use open_position::OpenPosition;
use roxmltree::{Document, Node};
use statement_section::{StatementSection, StatementSectionWithTimezone};
use std::collections::HashMap;
use std::fmt::Debug;
use trade::Trade;

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub account_info: AccountInfo,
    pub cash_reports: Vec<CashReport>,
    pub equity_summaries: Vec<EquitySummary>,
    pub fifo_performance_summaries: Vec<FIFOPerformanceSummary>,
    pub net_stock_positions: Vec<NetStockPosition>,
    pub open_positions: Vec<OpenPosition>,
    pub trades: Vec<Trade>,
}

/// Parser for interpreting the content of an InteractiveBrokers Flex-based XML statement.
///
/// # Examples
///
/// ```
/// use anyhow::Result;
/// use ibkr_flex_statement::{Parser, Statement};
///
/// let statement_xml: &str = "<FlexQueryResponse>...</FlexQueryResponse>";
///
/// let parser = Parser::new().unwrap();
/// let statements: Vec<Statement> = parser.parse_flex_query_response(statement_xml).unwrap();
/// ```
pub struct Parser {
    pub timezone_map: HashMap<String, Tz>,
}

impl Parser {
    pub fn new() -> Result<Self> {
        let new_york_tz: Tz = "America/New_York".parse().unwrap();
        let timezone_map = HashMap::from([
            ("EST".to_string(), new_york_tz),
            ("EDT".to_string(), new_york_tz),
        ]);

        Ok(Parser { timezone_map })
    }

    fn parse_section<T: StatementSection>(
        &self,
        node: &Node,
        section_name: &str,
    ) -> Result<Vec<T>> {
        node.descendants()
            .filter(|n| n.tag_name().name() == section_name)
            .map(|n| T::from_node(&NodeWrapper { node: n }).map_err(anyhow::Error::msg))
            .collect::<Result<Vec<T>>>()
    }

    fn parse_section_with_timezone<T: StatementSectionWithTimezone>(
        &self,
        node: &Node,
        section_name: &str,
    ) -> Result<Vec<T>> {
        node.descendants()
            .filter(|n| n.tag_name().name() == section_name)
            .map(|n| {
                T::from_node(&NodeWrapper { node: n }, &self.timezone_map)
                    .map_err(anyhow::Error::msg)
            })
            .collect::<Result<Vec<T>>>()
    }

    fn parse_flex_statement(&self, node: &Node) -> Result<Statement> {
        let account_infos = self.parse_section::<AccountInfo>(node, "AccountInformation")?;
        if account_infos.len() > 1 {
            return Err(anyhow::Error::msg(
                "multiple account information sections found",
            ));
        } else if account_infos.is_empty() {
            return Err(anyhow::Error::msg("no account information sections found"));
        }
        let account_info = account_infos[0].clone();

        let cash_reports = self.parse_section(node, "CashReportCurrency")?;
        let equity_summaries = self.parse_section(node, "EquitySummaryByReportDateInBase")?;
        let fifo_performance_summaries =
            self.parse_section(node, "FIFOPerformanceSummaryUnderlying")?;
        let net_stock_positions = self.parse_section(node, "NetStockPosition")?;
        let open_positions = self.parse_section(node, "OpenPosition")?;
        let trades = self.parse_section_with_timezone(node, "Trade")?;

        Ok(Statement {
            account_info,
            cash_reports,
            equity_summaries,
            fifo_performance_summaries,
            net_stock_positions,
            open_positions,
            trades,
        })
    }

    pub fn parse_flex_query_response(&self, flex_query_response: &str) -> Result<Vec<Statement>> {
        let doc = Document::parse(flex_query_response)?;
        doc.descendants()
            .filter(|n| n.tag_name().name() == "FlexStatement")
            .map(|n| self.parse_flex_statement(&n).map_err(anyhow::Error::msg))
            .collect::<Result<Vec<Statement>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono_tz::Tz;

    const FULL_STATEMENT_EXAMPLE: &str = r##"
        <FlexQueryResponse queryName="example-query" type="AF">
            <FlexStatements count="1">
                <FlexStatement accountId="U1234567" fromDate="2025-04-25" toDate="2025-04-25" period="LastBusinessDay" whenGenerated="2025-04-26;13:34:28 EDT">
                    <AccountInformation accountId="U1234567" accountType="Individual" customerType="Individual" accountCapabilities="Portfolio Margin" tradingPermissions="Stocks,Options,Warrants,Forex,Futures,Crypto Currencies,Mutual Funds,Fully Paid Stock Loan" />
                    <EquitySummaryInBase>
                        <EquitySummaryByReportDateInBase accountId="U1234567" currency="USD" cash="-1755658.753685009" cashLong="0.000832235" cashShort="-1755658.754517244" commodities="0" commoditiesLong="0" commoditiesShort="0" dividendAccruals="0" dividendAccrualsLong="0" dividendAccrualsShort="0" interestAccruals="1292.18" interestAccrualsLong="1591.34" interestAccrualsShort="-299.16" stock="3441241" stockLong="3441241" stockShort="0" funds="0" fundsLong="0" fundsShort="0" brokerInterestAccrualsComponent="186.32" brokerInterestAccrualsComponentLong="485.48" brokerInterestAccrualsComponentShort="-299.16" brokerFeesAccrualsComponent="0" brokerFeesAccrualsComponentLong="0" brokerFeesAccrualsComponentShort="0" total="1686874.426314991" totalLong="3442832.340832235" totalShort="-1755957.914517244" reportDate="2025-04-24" />
                        <EquitySummaryByReportDateInBase accountId="U1234567" currency="USD" cash="-1856140.99825062" cashLong="0.000832132" cashShort="-1856140.999082752" commodities="0" commoditiesLong="0" commoditiesShort="0" dividendAccruals="0" dividendAccrualsLong="0" dividendAccrualsShort="0" interestAccruals="1051.42" interestAccrualsLong="1591.34" interestAccrualsShort="-539.92" stock="3664457" stockLong="3664457" stockShort="0" funds="0" fundsLong="0" fundsShort="0" brokerInterestAccrualsComponent="-54.44" brokerInterestAccrualsComponentLong="485.48" brokerInterestAccrualsComponentShort="-539.92" brokerFeesAccrualsComponent="0" brokerFeesAccrualsComponentLong="0" brokerFeesAccrualsComponentShort="0" total="1809367.421749379" totalLong="3666048.340832131" totalShort="-1856680.919082752" reportDate="2025-04-25" />
                    </EquitySummaryInBase>
                    <FIFOPerformanceSummaryInBase>
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="STK" symbol="TTWO" conid="6478131" listingExchange="NASDAQ" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="0" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="0" unrealizedProfit="4089.983554" unrealizedLoss="0" unrealizedSTProfit="4089.983554" unrealizedSTLoss="0" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="4089.983554" />
                        <FIFOPerformanceSummaryUnderlying accountId="U1234567" assetCategory="" symbol="" conid="" listingExchange="" reportDate="2025-04-25" realizedSTProfit="0" realizedSTLoss="-205.04987357" realizedLTProfit="0" realizedLTLoss="0" totalRealizedPnl="-205.04987357" unrealizedProfit="131057.571473" unrealizedLoss="-44834.337024864" unrealizedSTProfit="131057.571473" unrealizedSTLoss="-44834.337024864" unrealizedLTProfit="0" unrealizedLTLoss="0" totalFifoPnl="86018.184574566" />
                    </FIFOPerformanceSummaryInBase>
                    <CashReport>
                        <CashReportCurrency accountId="U1234567" currency="BASE_SUMMARY" fromDate="2025-04-25" toDate="2025-04-25" netTradesSales="0" netTradesPurchases="0" startingCash="-1755658.753685008" startingCashSec="-1755658.753685008" startingCashCom="0" commissions="-56.26956551" commissionsSec="-56.26956551" commissionsCom="0" commissionsMTD="-11167.4772929" commissionsYTD="-25339.56064716" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" depositWithdrawalsMTD="0" depositWithdrawalsYTD="1650000" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" debitCardActivityMTD="0" debitCardActivityYTD="0" dividends="0" dividendsSec="0" dividendsCom="0" dividendsMTD="0" dividendsYTD="110.7" otherFees="-19.77" otherFeesSec="-19.77" otherFeesCom="0" otherFeesMTD="-121.27" otherFeesYTD="-486.9" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" otherIncomeMTD="0" otherIncomeYTD="0" endingCash="-1856140.99825062" endingCashSec="-1856140.99825062" endingCashCom="0" endingSettledCash="-1755734.79325062" endingSettledCashSec="-1755734.79325062" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerInterestMTD="-545.49" brokerInterestYTD="-1341.59" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" brokerFeesMTD="0" brokerFeesYTD="0" deposits="0" depositsSec="0" depositsCom="0" depositsMTD="0" depositsYTD="1650000" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" withdrawalsMTD="0" withdrawalsYTD="0" />
                        <CashReportCurrency accountId="U1234567" currency="CAD" fromDate="2025-04-25" toDate="2025-04-25" netTradesSales="0" netTradesPurchases="0" startingCash="0.001153" startingCashSec="0.001153" startingCashCom="0" commissions="0" commissionsSec="0" commissionsCom="0" commissionsMTD="0" commissionsYTD="0" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" depositWithdrawalsMTD="0" depositWithdrawalsYTD="0" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" debitCardActivityMTD="0" debitCardActivityYTD="0" dividends="0" dividendsSec="0" dividendsCom="0" dividendsMTD="0" dividendsYTD="0" otherFees="0" otherFeesSec="0" otherFeesCom="0" otherFeesMTD="0" otherFeesYTD="0" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" otherIncomeMTD="0" otherIncomeYTD="0" endingCash="0.001153" endingCashSec="0.001153" endingCashCom="0" endingSettledCash="0.001153" endingSettledCashSec="0.001153" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerInterestMTD="0" brokerInterestYTD="0" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" brokerFeesMTD="0" brokerFeesYTD="0" deposits="0" depositsSec="0" depositsCom="0" depositsMTD="0" depositsYTD="0" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" withdrawalsMTD="0" withdrawalsYTD="0" />
                        <CashReportCurrency accountId="U1234567" currency="USD" fromDate="2025-04-25" toDate="2025-04-25" netTradesSales="0" netTradesPurchases="0" startingCash="-1755658.754517244" startingCashSec="-1755658.754517244" startingCashCom="0" commissions="-56.26956551" commissionsSec="-56.26956551" commissionsCom="0" commissionsMTD="-11167.4772929" commissionsYTD="-25339.56064716" depositWithdrawals="0" depositWithdrawalsSec="0" depositWithdrawalsCom="0" depositWithdrawalsMTD="0" depositWithdrawalsYTD="1650000" debitCardActivity="0" debitCardActivitySec="0" debitCardActivityCom="0" debitCardActivityMTD="0" debitCardActivityYTD="0" dividends="0" dividendsSec="0" dividendsCom="0" dividendsMTD="0" dividendsYTD="110.7" otherFees="-19.77" otherFeesSec="-19.77" otherFeesCom="0" otherFeesMTD="-121.27" otherFeesYTD="-486.9" otherIncome="0" otherIncomeSec="0" otherIncomeCom="0" otherIncomeMTD="0" otherIncomeYTD="0" endingCash="-1856140.999082752" endingCashSec="-1856140.999082752" endingCashCom="0" endingSettledCash="-1755734.794082752" endingSettledCashSec="-1755734.794082752" endingSettledCashCom="0" brokerInterest="0" brokerInterestSec="0" brokerInterestCom="0" brokerInterestMTD="-545.49" brokerInterestYTD="-1341.59" brokerFees="0" brokerFeesSec="0" brokerFeesCom="0" brokerFeesMTD="0" brokerFeesYTD="0" deposits="0" depositsSec="0" depositsCom="0" depositsMTD="0" depositsYTD="1650000" withdrawals="0" withdrawalsSec="0" withdrawalsCom="0" withdrawalsMTD="0" withdrawalsYTD="0" />
                    </CashReport>
                    <OpenPositions>
                        <OpenPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="GRPN" conid="426480582" listingExchange="NASDAQ" reportDate="2025-04-25" position="3000" markPrice="19.89" positionValue="59670" openPrice="20.153441225" costBasisPrice="20.153441225" percentOfNAV="1.63" fifoPnlUnrealized="-790.323674" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="META" conid="107113386" listingExchange="NASDAQ" reportDate="2025-04-25" position="800" markPrice="547.27" positionValue="437816" openPrice="542.020354354" costBasisPrice="542.020354354" percentOfNAV="11.95" fifoPnlUnrealized="4199.716517" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="NFLX" conid="15124833" listingExchange="NASDAQ" reportDate="2025-04-25" position="400" markPrice="1101.53" positionValue="440612" openPrice="1056.32548211" costBasisPrice="1056.32548211" percentOfNAV="12.02" fifoPnlUnrealized="18081.807156" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="PLTR" conid="444857009" listingExchange="NASDAQ" reportDate="2025-04-25" position="3100" markPrice="112.78" positionValue="349618" openPrice="104.761973398" costBasisPrice="104.761973398" percentOfNAV="9.54" fifoPnlUnrealized="24855.882465" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TQQQ" conid="72539702" listingExchange="NASDAQ" reportDate="2025-04-25" position="34100" markPrice="53.86" positionValue="1836626" openPrice="53.776784497" costBasisPrice="53.776784497" percentOfNAV="50.12" fifoPnlUnrealized="2837.648645" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TSLA" conid="76792991" listingExchange="NASDAQ" reportDate="2025-04-25" position="1500" markPrice="284.95" positionValue="427425" openPrice="262.984320092" costBasisPrice="262.984320092" percentOfNAV="11.66" fifoPnlUnrealized="32948.519862" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                        <OpenPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TTWO" conid="6478131" listingExchange="NASDAQ" reportDate="2025-04-25" position="500" markPrice="225.38" positionValue="112690" openPrice="217.200032892" costBasisPrice="217.200032892" percentOfNAV="3.08" fifoPnlUnrealized="4089.983554" side="Long" openDateTime="" holdingPeriodDateTime="" accruedInt="" commodityType="" />
                    </OpenPositions>
                    <NetStockPositionSummary>
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="GRPN" conid="426480582" listingExchange="NASDAQ" netShares="3000" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="META" conid="107113386" listingExchange="NASDAQ" netShares="800" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="NFLX" conid="15124833" listingExchange="NASDAQ" netShares="400" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="PLTR" conid="444857009" listingExchange="NASDAQ" netShares="3100" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TQQQ" conid="72539702" listingExchange="NASDAQ" netShares="34100" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TSLA" conid="76792991" listingExchange="NASDAQ" netShares="1500" />
                        <NetStockPosition accountId="U1234567" currency="USD" assetCategory="STK" symbol="TTWO" conid="6478131" listingExchange="NASDAQ" netShares="500" />
                    </NetStockPositionSummary>
                    <Trades>
                        <Trade accountId="U1234567" currency="USD" symbol="ARGX" conid="276343981" listingExchange="NASDAQ" tradeID="7587063231" reportDate="2025-04-25" dateTime="2025-04-25;10:19:55 EDT" tradeDate="2025-04-25" transactionType="ExchTrade" exchange="BYX" quantity="1" tradePrice="606.57" tradeMoney="606.57" proceeds="-606.57" ibCommission="-1.000035" ibCommissionCurrency="USD" netCash="-607.570035" closePrice="614.76" openCloseIndicator="O" cost="607.570035" fifoPnlRealized="0" mtmPnl="8.19" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" origTransactionID="0" buySell="BUY" ibOrderID="4015030800" transactionID="32580112485" ibExecID="0000edae.680b59d1.01.01" orderTime="2025-04-25;10:19:55 EDT" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" orderType="LMT" accruedInt="0" assetCategory="STK" brokerageOrderID="002ce642.00014b44.680b0ed6.0001" orderReference="" isAPIOrder="N" initialInvestment="" />
                        <Trade accountId="U1234567" currency="USD" symbol="GEO" conid="158655765" listingExchange="NYSE" tradeID="7587946875" reportDate="2025-04-25" dateTime="2025-04-25;11:24:28 EDT" tradeDate="2025-04-25" transactionType="ExchTrade" exchange="NYSE" quantity="1000" tradePrice="30.85" tradeMoney="30850" proceeds="-30850" ibCommission="-5.035" ibCommissionCurrency="USD" netCash="-30855.035" closePrice="30.58" openCloseIndicator="O" cost="30855.035" fifoPnlRealized="0" mtmPnl="-270" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" origTransactionID="0" buySell="BUY" ibOrderID="4015577648" transactionID="32582764875" ibExecID="00012e0e.680b7717.01.01" orderTime="2025-04-25;11:24:26 EDT" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" orderType="LMT" accruedInt="0" assetCategory="STK" brokerageOrderID="002ce642.00014b44.680b0fbf.0001" orderReference="" isAPIOrder="N" initialInvestment="" />
                    </Trades>
                </FlexStatement>
            </FlexStatements>
         </FlexQueryResponse>
        "##;

    #[test]
    fn long_timezone_name_parses() -> Result<()> {
        let _tz: Tz = "America/New_York".parse().unwrap();
        Ok(())
    }

    #[test]
    fn parsing_succeeds() -> Result<()> {
        let statements = Parser::new()?.parse_flex_query_response(FULL_STATEMENT_EXAMPLE)?;
        assert_eq!(statements.len(), 1);
        let result = &statements[0];

        // Ensure we got two equity summaries.
        assert_eq!(result.cash_reports.len(), 3);
        assert_eq!(result.equity_summaries.len(), 2);
        assert_eq!(result.fifo_performance_summaries.len(), 2);
        assert_eq!(result.net_stock_positions.len(), 7);
        assert_eq!(result.open_positions.len(), 7);
        assert_eq!(result.trades.len(), 2);

        Ok(())
    }
}

use crate::{node_utils::NodeWrapper, statement_section::StatementSectionWithTimezone};

use super::currency::Currency;
use anyhow::Result;
use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq)]
pub enum OpenCloseIndicator {
    Close,
    CloseOpen,
    Open,
}

#[derive(Debug, PartialEq)]
pub enum OrderType {
    Limit,
}

#[derive(Debug, PartialEq)]
pub struct Trade {
    pub account_id: String,
    pub conid: u32,
    pub currency: Currency,
    pub execution_exchange: String,
    pub execution_id: String,
    pub execution_timestamp_ms: i64,
    pub commission: f64,
    pub listing_exchange: String,
    pub open_close_indicator: OpenCloseIndicator,
    pub order_id: String,
    pub order_type: OrderType,
    pub price: f64,
    pub quantity: f64,
    pub side: TradeSide,
    pub ticker: String,
}

impl<'a> TryFrom<&'a str> for OpenCloseIndicator {
    type Error = anyhow::Error;
    fn try_from(s: &'a str) -> Result<Self> {
        match s {
            "C" => Ok(Self::Close),
            "C;O" => Ok(Self::CloseOpen),
            "O" => Ok(OpenCloseIndicator::Open),
            _ => Err(anyhow::Error::msg(format!(
                "unknown openClose indicator {}",
                s
            ))),
        }
    }
}

impl<'a> TryFrom<&'a str> for OrderType {
    type Error = anyhow::Error;
    fn try_from(s: &'a str) -> Result<Self> {
        match s {
            "LMT" => Ok(Self::Limit),
            _ => Err(anyhow::Error::msg(format!("unknown order type {}", s))),
        }
    }
}

impl<'a> TryFrom<&'a str> for TradeSide {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        match s {
            "BUY" => Ok(Self::Buy),
            "SELL" => Ok(Self::Sell),
            _ => Err(anyhow::Error::msg(format!("unknown trade side {}", s))),
        }
    }
}

fn try_parse_trade_execution_time_ms(tz_map: &HashMap<String, Tz>, s: &str) -> Result<i64> {
    let mut dt_parts = s.split(" ");
    let datetime_str = dt_parts.next().unwrap();

    let short_timezone = dt_parts.next().unwrap();
    let timezone = tz_map.get(short_timezone).unwrap();

    let naive_dt = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d;%H:%M:%S %Z")?;
    let tz_aware_dt = timezone.from_local_datetime(&naive_dt).unwrap();

    // println!("tz_aware_dt: {:?}, timestamp: {}", tz_aware_dt, tz_aware_dt.timestamp());

    Ok(tz_aware_dt.timestamp() * 1000)
}

impl StatementSectionWithTimezone for Trade {
    fn from_node(node: &NodeWrapper, tz_map: &HashMap<String, Tz>) -> Result<Trade> {
        Ok(Trade {
            account_id: node.get_attribute("accountId")?,
            commission: node.parse_attribute("ibCommission")?,
            conid: node.parse_attribute("conid")?,
            currency: Currency::try_from(node.node.attribute("currency").unwrap())?,
            execution_exchange: node.get_attribute("exchange")?,
            execution_id: node.get_attribute("ibExecID")?,
            execution_timestamp_ms: try_parse_trade_execution_time_ms(
                tz_map,
                node.node.attribute("dateTime").unwrap(),
            )?,
            listing_exchange: node.get_attribute("listingExchange")?,
            open_close_indicator: OpenCloseIndicator::try_from(
                node.node.attribute("openCloseIndicator").unwrap(),
            )?,
            order_id: node.get_attribute("brokerageOrderID")?,
            order_type: OrderType::try_from(node.node.attribute("orderType").unwrap())?,
            price: node.parse_attribute("tradePrice")?,
            quantity: node.parse_attribute("quantity")?,
            side: TradeSide::try_from(node.node.attribute("buySell").unwrap())?,
            ticker: node.get_attribute("symbol")?,
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
                    <Trades>
                        <Trade accountId="U1234567" 
                               currency="USD"
                               symbol="ARGX"
                               conid="276343981"
                               listingExchange="NASDAQ"
                               tradeID="7587063231"
                               reportDate="2025-04-25"
                               dateTime="2025-04-25;10:19:55 EDT"
                               tradeDate="2025-04-25"
                               transactionType="ExchTrade"
                               exchange="BYX"
                               quantity="1"
                               tradePrice="606.57"
                               tradeMoney="606.57"
                               proceeds="-606.57"
                               ibCommission="-1.000035"
                               ibCommissionCurrency="USD"
                               netCash="-607.570035"
                               closePrice="614.76"
                               openCloseIndicator="O"
                               cost="607.570035"
                               fifoPnlRealized="0"
                               mtmPnl="8.19"
                               origTradePrice="0"
                               origTradeDate=""
                               origTradeID=""
                               origOrderID="0"
                               origTransactionID="0"
                               buySell="BUY"
                               ibOrderID="4015030800"
                               transactionID="32580112485"
                               ibExecID="0000edae.680b59d1.01.01"
                               orderTime="2025-04-25;10:19:55 EDT"
                               openDateTime=""
                               holdingPeriodDateTime=""
                               whenRealized=""
                               whenReopened=""
                               orderType="LMT"
                               accruedInt="0"
                               assetCategory="STK"
                               brokerageOrderID="002ce642.00014b44.680b0ed6.0001"
                               orderReference=""
                               isAPIOrder="N"
                               initialInvestment="" />
                        <Trade accountId="U1234567" 
                               currency="USD"
                               symbol="GEO"
                               conid="158655765"
                               listingExchange="NYSE"
                               tradeID="7587946875"
                               reportDate="2025-04-25"
                               dateTime="2025-04-25;11:24:28 EDT"
                               tradeDate="2025-04-25"
                               transactionType="ExchTrade"
                               exchange="NYSE"
                               quantity="1000"
                               tradePrice="30.85"
                               tradeMoney="30850"
                               proceeds="-30850"
                               ibCommission="-5.035"
                               ibCommissionCurrency="USD"
                               netCash="-30855.035"
                               closePrice="30.58"
                               openCloseIndicator="O"
                               cost="30855.035"
                               fifoPnlRealized="0"
                               mtmPnl="-270"
                               origTradePrice="0"
                               origTradeDate=""
                               origTradeID=""
                               origOrderID="0"
                               origTransactionID="0"
                               buySell="BUY"
                               ibOrderID="4015577648"
                               transactionID="32582764875"
                               ibExecID="00012e0e.680b7717.01.01"
                               orderTime="2025-04-25;11:24:26 EDT"
                               openDateTime=""
                               holdingPeriodDateTime=""
                               whenRealized=""
                               whenReopened=""
                               orderType="LMT"
                               accruedInt="0"
                               assetCategory="STK"
                               brokerageOrderID="002ce642.00014b44.680b0fbf.0001"
                               orderReference=""
                               isAPIOrder="N"
                               initialInvestment="" />
                    </Trades>
                </FlexStatement>
            </FlexStatements>
         </FlexQueryResponse>
        "##;

    #[test]
    fn trades_parse() -> Result<()> {
        let statements = Parser::new()?.parse_flex_query_response(PARTIAL_STATEMENT_EXAMPLE)?;
        assert_eq!(statements.len(), 1);
        let result = &statements[0];

        // Ensure we got two trades.
        assert_eq!(result.trades.len(), 2);

        // Ensure the first trade matches.
        assert_eq!(
            result.trades[0],
            Trade {
                account_id: "U1234567".to_string(),
                commission: -1.000035,
                conid: 276343981,
                currency: Currency::USD,
                execution_exchange: "BYX".to_string(),
                execution_id: "0000edae.680b59d1.01.01".to_string(),
                execution_timestamp_ms: result.trades[0].execution_timestamp_ms,
                open_close_indicator: OpenCloseIndicator::Open,
                order_id: "002ce642.00014b44.680b0ed6.0001".to_string(),
                order_type: OrderType::Limit,
                price: 606.57,
                quantity: 1.0,
                side: TradeSide::Buy,
                ticker: "ARGX".to_string(),
                listing_exchange: "NASDAQ".to_string(),
            }
        );

        // Ensure the first trade matches.
        assert_eq!(
            result.trades[1],
            Trade {
                account_id: "U1234567".to_string(),
                commission: -5.035,
                conid: 158655765,
                currency: Currency::USD,
                execution_exchange: "NYSE".to_string(),
                execution_id: "00012e0e.680b7717.01.01".to_string(),
                execution_timestamp_ms: result.trades[1].execution_timestamp_ms,
                open_close_indicator: OpenCloseIndicator::Open,
                order_id: "002ce642.00014b44.680b0fbf.0001".to_string(),
                order_type: OrderType::Limit,
                price: 30.85,
                quantity: 1000.0,
                side: TradeSide::Buy,
                ticker: "GEO".to_string(),
                listing_exchange: "NYSE".to_string(),
            }
        );
        Ok(())
    }
}

[<img alt="github" src="https://img.shields.io/badge/github-tfiala?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/tfiala/ibkr-flex-statement-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ibkr-flex-statement.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ibkr-flex-statement)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=docs.rs" height="20">](https://docs.rs/ibkr-flex-statement/latest/ibkr-flex-statement)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/tfiala/ibkr-flex-statement-rs/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/tfiala/ibkr-flex-statement-rs/actions/workflows/rust.yml)
[<img alt="codecov.io" src="https://img.shields.io/codecov/c/github/tfiala/ibkr-flex-statement-rs?style=for-the-badge" height="20">](https://codecov.io/gh/tfiala/ibkr-flex-statement-rs)

Rust-based InteractiveBrokers (IBKR) flex query result parser for broker statement data.

## Flex Query Configuration

The Flex query result parser has been developed and tested to parse Flex-based queries
that contain the following sections:

### Account Information
1. ClientAccountID
2. AccountType
3. CustomerType
4. AccountCapabilities
5. TradingPermissions

### Cash Report
1. ClientAccountID
2. CurrencyPrimary
3. FromDate
4. ToDate
5. StartingCash
6. Commissions
7. Deposit/Withdrawals
8. DebitCardActivity
9. Dividends
10. OtherFees
11. OtherIncome
12. EndingCash
13. EndingSettledCash
14. BrokerInterest
15. BrokerFees
16. Deposits
17. Withdrawals
18. NetTradesSales
19. NetTradesPurchases
20. TransactionTax
21. WithholdingTax
22. WithholdingTaxCollected
23. NetSecuritiesLentActivitySLB

### Interest Accruals
1. ClientAccountID
2. FromDate
3. ToDate
4. StartingAccrualBalance
5. InterestAccrued
6. AccrualReversal
7. EndingAccrualBalance
8. CurrencyPrimary

### Net Asset Value (NAV) in Base
1. ClientAccountID
2. CurrencyPrimary
3. Cash
4. Commodities
5. DividendAccruals
6. InterestAccruals
7. Stock
8. Funds
9. BrokerInterestAccrualsComponent
10. BrokerFeesAccrualsComponent
11. Total
12. ReportDate

### Net Stock Position Summary
1. ClientAccountID
2. CurrencyPrimary
3. AssetClass
4. Symbol
5. Conid
6. ListingExchange
7. NetShares

### Open Positions
Options: Summary
1. ClientAccountID
2. CurrencyPrimary
3. AssetClass
4. Symbol
5. Conid
6. ListingExchange
7. ReportDate
8. Quantity
9. MarkPrice
10. PositionValue
11. OpenPrice
12. CostBasisPrice
13. PercentOfNAV
14. FifoPnlUnrealized
15. Side
16. OpenDateTime
17. HoldingPeriodDateTime
18. AccruedInterest
19. CommodityType

### Realized and Unrealized Performance Summary in Base
1. ClientAccountID
2. AssetClass
3. Symbol
4. Conid
5. ListingExchange
6. ReportDate
7. RealizedShortTermProfit
8. RealizedShortTermLoss
9. RealizedLongTermProfit
10. RealizedLongTermLoss
11. TotalRealizedPnl
12. UnrealizedProfit
13. UnrealizedLoss
14. UnrealizedSTProfit
15. UnrealizedSTLoss
16. UnrealizedLTProfit
17. UnrealizedLTLoss
18. TotalFifoPnl

### Trades
Options: Execution
1. ClientAccountID
2. CurrencyPrimary
3. Symbol
4. Conid
5. ListingExchange
6. TradeID
7. ReportDate
8. DateTime
9. TradeDate
10. TransactionType
11. Exchange
12. Quantity
13. TradePrice
14. TradeMoney
15. Proceeds
16. IBCommission
17. IBCommissionCurrency
18. NetCash
19. ClosePrice
20. Open/CloseIndicator
21. CostBasis
22. FifoPnlRealized
23. MtmPnl
24. OrigTradePrice
25. OrigTradeDate
26. OrigTradeID
27. OrigOrderID
28. OrigTransactionID
29. Buy/Sell
30. IBOrderID
31. TransactionID
32. IBExecID
33. OrderTime
34. OpenDateTime
35. HoldingPeriodDateTime
36. WhenRealized
37. WhenReopened
38. OrderType
39. AccruedInterest
40. AssetClass
41. BrokerageOrderID
42. OrderReference
43. IsAPIOrder
44. InitialInvestment

### Transaction Fees
Options: Summary, Execution
1. ClientAccountID
2. CurrencyPrimary
3. Symbol
4. Description
5. Conid
6. ListingExchange
7. Date
8. ReportDate
9. SettleDate
10. Quantity
11. OrderID
12. TradePrice

### Delivery Configuration

Accounts

Format: XML

Period: (Any period works -- I am generally doing "Business Day" but also use yearly and monthly periods).

General Configuration

Profit and Loss: Default

Include Canceled Trades? Yes

Include Currency Rates? No

Include Audit Trail Fields? No

Display Account Alias in Place of Account ID? No

Breakout by Day? No

Date Format: yyyy-MM-dd

Time Format: HH:mm:ss TimeZone

Date/Time Separator: ; (semi-colon)

## Setup

```toml
[dependencies]
ibkr-flex-statement = "0.3.3"
```

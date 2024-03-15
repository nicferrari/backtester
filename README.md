# rs-backtester
rs-backtester is a financial backtesting library written entirely in Rust with the purpose of being
easy-to-use yet flexible enough to allow a quick implementation of different strategies

## Install
- To install simply add rs-backtester to your cargo.toml
```rust
[dependencies]
rs-backtester = "0.0.1"
```
## Get started

To get started:
- Import the necessary modules:
```rust
use std::error::Error;
use backtester::backtester::Backtest;
use backtester::datas::Data;
use backtester::strategies::{sma_cross};
```
- Define an instance of the Data class. Market data can be retrieve either through yahoo-finance or read from
a CSV file (OHLC format, Volume is not uploaded)
```rust
let quotes = Data::load("GOOG.csv","GOOG")?;
```
- As an alternative, you can retrieve data directly from yahoo finance with the following
which makes use of the crate yahoo-finance-api
```rust
let quotes = Data::new_from_yahoo("GOOG".to_string())?;
```
- Create a function which return a Strategy or use one provided by the library.<BR>
A Strategy is simply a vector of Choices (e.g. BUY, SHORTSELL, ...)
and the indicator used
```rust
let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
```
- Create an instance of the Backtest class
```rust
let mut sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64);
```
- Calculate it
```rust
sma_cross_tester.calculate();
```
- After you calculate:
  - you can read a report of the backtest
    ```rust
    report(&sma_cross_tester);
    ```
  - you can produce a log period by period with the requested parameter
    ```rust
    sma_cross_tester.log(&["date","open","high","low","close","position","account","indicator"]);
    ```
  - you can chart it (with indicators)
    ```rust
    plot(sma_cross_tester.clone())?;
    ``` 
  - you can CSV it
    ```rust
    sma_cross_tester.to_csv()?;
    ```

## Further enhancements (to be made)
- include user-defined commissions
- report of the backtesting with flexible user-decided fields (e.g. maximum drawdown, #trades, ...)
    this can also be done after the calculate steps on the log data...
  https://github.com/kernc/backtesting.py as an example
- report and chart multiple strategies at once and compare them
- (TBC) rework the strategy definition to use only a function, if possible
- implement error checking and lifetime of a Backtest object (linked to quotes?)
- (TBC) GUI with parsing user input
- diffent types of orders (partial, stop loss, profit taking, ...)<BR>
<BR>
- check vs backtrader
- when order stays for different period, position should be incremented,
or otherwise implement backtrader-style single orders
## Dependencies
```rust
[dependencies]
yahoo_finance_api = "2.1.0"
tokio-test = "0.4.3"
chrono = { version = "0.4.33", features = [] }
plotters = "0.3.5"
serde = "1.0.196"
csv = "1.3.0"
```
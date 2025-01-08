# rs-backtester
[![Latest version](https://img.shields.io/crates/v/rs-backtester.svg)](https://crates.io/crates/rs-backtester)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/nicferrari/rs-backtester/blob/master/LICENSE-APACHE-2.0)

rs-backtester is a financial backtesting library written entirely in Rust with the purpose of being
easy-to-use yet flexible enough to allow a quick implementation of different strategies

## Get started

To get started:
- Import the necessary modules:
```rust
use std::error::Error;
use rs_backtester::backtester::{Backtest, Commission};
use rs_backtester::datas::Data;
use rs_backtester::strategies::{sma_cross};
use rs_backtester::report::{report};
```
- Define an instance of the Data class. Market data can be retrieved either through yahoo-finance or read from
a CSV file (OHLC format, Volume is not uploaded)
```rust
let quotes = Data::load("GOOG.csv","GOOG")?;
```
- As an alternative, you can retrieve data directly from yahoo finance with the following
which makes use of the crate yahoo-finance-api
```rust
let quotes = Data::new_from_yahoo("GOOG","1d","6mo")?;
```
- Create a function which returns a Strategy or use one provided by the library.<BR>
A Strategy is basically a vector of Choices (e.g. BUY, SHORTSELL, ...)
and the indicator used
```rust
let sma_cross_strategy = sma_cross(quotes.clone(), 10,20);
```
- Create an instance of the Backtest class
```rust
let sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64, Commission::default());
```
- Now:
  - you can read a report of the backtest
    ```rust
    report(sma_cross_tester);
    ```
  - you can produce a log period by period with the requested parameters
    ```rust
    sma_cross_tester.log(&["date","open","high","low","close","position","account","indicator"]);
    ```
  - you can chart it (with indicators)
    ```rust
    plot(sma_cross_tester,plot_config)?;
    ``` 
  <img src="https://github.com/nicferrari/rs-backtester/blob/master/plot.png" width="400"><BR>
  - you can save it to CSV for inspection
    ```rust
    sma_cross_tester.to_csv("sma_cross.csv")?;
    ```
  - you can also compare multiple strategies at once
  - and you can also play with your strategy modifying it by inverting it or transform it in long or short-only
<HR>
Check the examples folder for more!

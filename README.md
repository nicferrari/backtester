Financial backtester

## Get started

To get started:
- define an instance of the Data class. Market data can be retrieve either through yahoo-finance or read from
a CSV file (OHLC format) (*** to be implemented***)
- create a function which return a Strategy. A Strategy is simply a vector of Choices (e.g. BUY, SHORTSELL, ...)
and the indicator used (for charting purposes) (*** makes little sense***)
- create an instance of the Backtest class
- calculate it (*** to be automatically called ***)
- you can produce a log period by period with the requested parameter
- you can chart it (with indicators)
- you can CSV it (*** to be implemented ***)
```rust
let mut sma_cross_tester = Backtest::new(...)?;
```
## Further enhancements (to be made)
- CSV created by Backtest class
- report of the backtesting with flexible user-decided fields (e.g. maximum drawdown, #trades, ...)
    this can also be done after the calculate steps on the log data...
- report and chart multiple strategies at once and compare them
- (TBC) rework the strategy definition to use only a function, if possible
- implement error checking and lifetime of a Backtest object (linked to quotes?)
- (TBC) GUI with parsing user input
<BR>
- check vs backtrader
- correct buy at close
- when order stays for different period, position should be incremented,
or otherwise implement backtrader-style single orders
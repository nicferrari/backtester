## [0.1.4]
- added volumes to Data
- added max drawdown and sharpe ratio to report
- added new strategy modifier skip_first if you want to avoid entering as soon as a signal calculation is available
- moved test data in test_data folder
- simplification of csv generation (with the exception of backtesting under rework) with possibility to combine in a single csv more granular csv output
- rework of engines (now calculations are made at backtest initialization and stored in Metrics and not in multiple places)
- added computation timing printout
- improved performance (more than 4x on backtesting)
- rework of backtesting and strategies
- rework of reports
- rework of examples
- rework of tests

## [0.1.3]
- Possibility to display the list of trades produced by a strategy
- Minor bugs fixes
use backtester::backtester::Backtest;
use backtester::charts::plot;
use backtester::datas::Data;
use backtester::strategies::{buy_n_hold, do_nothing, short_n_hold, simple_sma};
use backtester::Result;

fn main()->Result<()>{
    let quotes = Data::new_from_yahoo("AAPL".to_string())?;
    /*let bnh_strategy = buy_n_hold(quotes.clone());
    let mut bnh_tester = Backtest::new(quotes.clone(),bnh_strategy.clone(),100000f64)?;
    //print_report(quotes,strategy);
    bnh_tester.calculate();
    bnh_tester.print_report();
    let snh_strategy = short_n_hold(quotes.clone());
    let mut snh_tester = Backtest::new(quotes.clone(),snh_strategy.clone(),100000f64)?;
    snh_tester.calculate();
    snh_tester.print_report();
    let nothing_strategy = do_nothing(quotes.clone());
    let mut nothing_tester = Backtest::new(quotes.clone(),nothing_strategy.clone(),100000f64)?;
    nothing_tester.calculate();
    nothing_tester.print_report();
    let revert_bnh_strategy = bnh_strategy.revert();
    let mut revert_bnh_tester = Backtest::new(quotes.clone(),revert_bnh_strategy.clone(),100000f64)?;
    revert_bnh_tester.calculate();
    revert_bnh_tester.print_report_arg2(&["date","open","close","position","account"]);
*/
    let sma_cross_strategy = simple_sma(quotes.clone(), 5);
    let mut sma_cross_tester = Backtest::new(quotes.clone(),sma_cross_strategy.clone(),100000f64)?;
    sma_cross_tester.calculate();   //da togliere perchè superfluo va chiamata all'inizializzazione in automatico
    sma_cross_tester.log(&["date","open","high","low","close","position","account"]);
    //_ = plot(&quotes, &sma_cross_tester.position(), &sma_cross_tester.account(), &sma_cross_strategy.choices());
    _ = plot(sma_cross_tester);

    Ok(())
}
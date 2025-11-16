use criterion::{criterion_group, criterion_main, Criterion};
use rs_backtester::backtester::Backtest;
use rs_backtester::data::Data;
use rs_backtester::strategies::sma_cross;

fn backtesting_calc(c: &mut Criterion) {
    let filename = "test_data//NVDA.csv";
    let quotes = Data::load(filename, "NVDA").unwrap();
    let sma_cross_strategy_arc = sma_cross(quotes.clone(), 10, 20);
    c.bench_function("backtesting calculation", |b| {
        b.iter(|| Backtest::new(sma_cross_strategy_arc.clone(), 100_000.))
    });
}

criterion_group!(benches, backtesting_calc);
criterion_main!(benches);

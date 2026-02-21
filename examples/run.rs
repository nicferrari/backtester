use rs_backtester::bt_run::*;
fn main() -> Result<(),Box<dyn std::error::Error>>{
    let cfg = load_config("test_data//bt_runs.toml");
    run(cfg)?;
    Ok(())
}

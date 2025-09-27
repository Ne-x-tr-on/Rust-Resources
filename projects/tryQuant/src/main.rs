use serde::Deserialize;
use std::error::Error;

/// Represents daily stock price data
#[derive(Debug, Deserialize)]
struct PriceData {
    date: String,
    close: f64,
}

/// Calculate simple moving average
fn moving_average(prices: &[f64], window: usize) -> Vec<f64> {
    prices
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect()
}

/// Backtest a moving average crossover strategy
fn backtest(prices: &[PriceData], short_window: usize, long_window: usize) {
    let close_prices: Vec<f64> = prices.iter().map(|p| p.close).collect();

    let short_ma = moving_average(&close_prices, short_window);
    let long_ma = moving_average(&close_prices, long_window);

    // Align both series to the same length
    let offset = long_window - 1;
    let mut position = false;
    let mut pnl = 0.0;

    for i in 0..short_ma.len() {
    let idx = i + offset;
    if idx >= close_prices.len() {
        break; // ✅ avoid out of bounds
    }

    let price = close_prices[idx];




        if short_ma[i] > long_ma[i] && !position {
            println!("{}: BUY at {:.2}", prices[idx].date, price);
            position = true;
        } else if short_ma[i] < long_ma[i] && position {
            println!("{}: SELL at {:.2}", prices[idx].date, price);
            pnl += price - close_prices[idx - 1];
            position = false;
        }
    }

    println!("Final PnL: {:.2}", pnl);
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load stock data from CSV
    // let mut rdr = csv::Reader::from_path("data.csv")?;
    let mut rdr = csv::Reader::from_path("data.csv")?;

    let mut prices: Vec<PriceData> = Vec::new();

    for result in rdr.deserialize() {
        let record: PriceData = result?;
        prices.push(record);
    }

    println!("Loaded {} price records", prices.len());

    // Run backtest with 5-day and 20-day moving averages
    backtest(&prices, 5, 20);

    Ok(())
}

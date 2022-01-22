//use reqwest::Error;
use chrono::prelude::*;
use reqwest::blocking::Client;

mod kucoin;

pub struct PricePoint {
    // This struct represents the parsed financial data
    datetime: DateTime<Utc>,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
    turnover: f64,
}
impl PricePoint {
    fn print_point(&self) {
        println!(
            "{:?}, {}, {}, {}, {}, {}, {}",
            self.datetime, self.open, self.close, self.high, self.low, self.turnover, self.volume
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_period = kucoin::SamplePeriod::Minute3;
    let url = kucoin::build_url(30, &sample_period);
    let client = Client::new(); // Create a new Client
    let response = client.get(url).send()?; // GET response from API endpoint at URL
    let mut parsed_data: Vec<PricePoint> = Vec::new(); // Make a new Vec to hold the data
    kucoin::process_price_history(response, &mut parsed_data);

    println!(
        "\n>> Received {} samples, spaced every {} seconds:\n",
        parsed_data.len(),
        sample_period.get_seconds().0
    );
    for pp in parsed_data {
        pp.print_point()
    }
    Ok(()) // main return
}

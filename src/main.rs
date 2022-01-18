//use reqwest::Error;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    code: String,
    data: Vec<[String; 7]>,
}
pub enum SamplePeriods {
    min1 { code: String, period: i64 },
    min3,
    min5,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Time inputs
    let sample_period = "1min"; // use enum...   1min, 3min, 5min, 15min, 30min, 1hour, 2hour, 4hour, 6hour, 8hour, 12hour, 1day, 1week
    let sample_period_s = 60;
    let n_samples = 100;

    // Time calculations
    let t_duration_s = n_samples * sample_period_s;
    let t_now_s = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let end_time_s = t_now_s;
    let start_time_s = end_time_s - t_duration_s;

    // Build URL string
    let coin_symbol = "ETH";
    let base_currency = "USDT";
    let trading_pair = format!("{coin_symbol}-{base_currency}");

    let end_time = end_time_s.to_string(); //"1642450614";
    let start_time = start_time_s.to_string(); // "1642449114";

    let url = format!("https://api.kucoin.com/api/v1/market/candles?type={sample_period}&symbol={trading_pair}&startAt={start_time}&endAt={end_time}");

    let client = Client::new();
    let response: ResponseData = client.get(url).send()?.json()?;

    let mut x: f64;
    for (i, point) in response.data.iter().enumerate() {
        print!("{i}, ");
        for value in point {
            x = value.parse()?;
            print!("{:?}, ", x);
        }
        print!("\n");
    }
    Ok(())
}

// Example kucoin price query. Max 1500 data points per response
// https://api.kucoin.com/api/v1/market/candles?type=1min&symbol=BTC-USDT&startAt=1566703297&endAt=1566789757
// json_response =
//  {
//      "code":"200000",
//      "data":[
//          [time_seconds, open, close, high, low, volume, turnover],
//          ...
//      ]
//  }
//

fn _signal_change(v: Vec<f64>) -> Vec<f64> {
    let mut result_vec: Vec<f64> = Vec::new();
    let n = v.len();
    let last_element = v[n - 1];
    for element in v {
        result_vec.push(1.0 - element / last_element);
    }
    result_vec // Return value
}

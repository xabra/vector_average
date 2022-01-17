//use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct KucoinResponse {
    code: String,
    data: Vec<[String; 7]>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.kucoin.com/api/v1/market/candles?type=1min&symbol=BTC-USDT&startAt=1566703297&endAt=1566789757";
    let resp_string: String = reqwest::blocking::get(url)?.text()?;
    let deserialized: KucoinResponse = serde_json::from_str(&resp_string).unwrap();
    println!("deserialized = {:?}", deserialized.code);
    println!("deserialized = {:?}", deserialized.data[0][2]);
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

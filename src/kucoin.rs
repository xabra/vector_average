use chrono::prelude::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    // This struct holds the JSON response from GET to KuCoin URL
    code: String,
    data: Vec<[String; 7]>,
}

pub fn process_price_history(
    response: reqwest::blocking::Response,
    parsed_data: &mut Vec<super::PricePoint>,
) {
    let response_json: ResponseData = response.json().unwrap();
    for point in response_json.data.iter() {
        let pp = build_price_point(point);
        parsed_data.push(pp);
    }
}
fn build_price_point(point: &[String; 7]) -> super::PricePoint {
    let timestamp: i64 = point[0].parse().unwrap();
    let data = super::PricePoint {
        datetime: Utc.timestamp(timestamp, 0),
        open: point[1].parse().unwrap(),
        close: point[2].parse().unwrap(),
        high: point[3].parse().unwrap(),
        low: point[4].parse().unwrap(),
        volume: point[5].parse().unwrap(),
        turnover: point[6].parse().unwrap(),
    };
    data // Return value
}
pub fn build_url(n_samples: i64) -> String {
    // Time inputs
    let sample_period = "1min"; // use enum...   1min, 3min, 5min, 15min, 30min, 1hour, 2hour, 4hour, 6hour, 8hour, 12hour, 1day, 1week
    let sample_period_s = 60;

    // Time calculations
    let t_duration_s = chrono::Duration::seconds(n_samples * sample_period_s);
    let t_now_s: DateTime<Utc> = Utc::now();

    let end_time_s = t_now_s;
    let start_time_s = end_time_s - t_duration_s;

    // Build URL string
    let coin_symbol = "ETH";
    let base_currency = "USDT";
    let trading_pair = format!("{coin_symbol}-{base_currency}");

    let end_time = end_time_s.timestamp().to_string(); //"1642450614";
    let start_time = start_time_s.timestamp().to_string(); // "1642449114";

    let url = format!("https://api.kucoin.com/api/v1/market/candles?type={sample_period}&symbol={trading_pair}&startAt={start_time}&endAt={end_time}");
    url // return value
}
// ----------------------------------------------------------------
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

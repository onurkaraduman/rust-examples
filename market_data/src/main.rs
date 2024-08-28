use futures::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Deserialize)]
pub struct MarkPriceUpdate {
    pub stream: String,
    pub data: MarkPriceUpdateData,
}

#[derive(Deserialize)]
pub struct MarkPriceUpdateData {
    pub e: String,
    #[serde(rename = "E")]
    pub e2: i64,
    pub s: String,
    pub p: String,
    #[serde(rename = "P")]
    pub p2: String,
    pub i: String,
    pub r: String,
    #[serde(rename = "T")]
    pub t: i64,
}

#[derive(Deserialize)]
pub struct AggTrade {
    pub stream: String,
    pub data: AggTradeData,
}

#[derive(Deserialize)]
pub struct AggTradeData {
    pub e: String,
    #[serde(rename = "E")]
    pub e2: i64,
    pub a: i64,
    pub s: String,
    pub p: String,
    pub q: String,
    pub f: i64,
    pub l: i64,
    #[serde(rename = "T")]
    pub t: i64,
    pub m: bool,
}

#[tokio::main]
async fn main() {
    println!("Market data client is starting....");
    let url_str =
        String::from("wss://fstream.binance.com/stream?streams=bnbusdt@aggTrade/btcusdt@markPrice");
    // let url = Url::parse(&url_str).unwrap();
    println!("Connecting to binance websocket...");
    let (ws_stream, _) = connect_async(url_str).await.expect("Failed to connect");
    let (_write, mut read) = ws_stream.split();
    println!("Connected to binance websocket");

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let tick_data: Value = serde_json::from_str(&text).unwrap();
                let msg_type = &tick_data["data"]["e"];

                if msg_type == "aggTrade" {
                    let agg_trade: AggTrade = serde_json::from_str(&text).unwrap();
                    println!(
                        "Msg: {}, Symbol: {}, Price: {}, Quantity: {}",
                        msg_type, agg_trade.data.s, agg_trade.data.p, agg_trade.data.p
                    );
                } else if msg_type == "markPriceUpdate" {
                    let mark_price: MarkPriceUpdate = serde_json::from_str(&text).unwrap();
                    println!(
                        "Msg: {}, Symbol: {}, Price: {}, Quantity: {}",
                        msg_type, mark_price.data.s, mark_price.data.p, mark_price.data.p
                    );
                }
            }
            Ok(Message::Ping(_))
            | Ok(Message::Pong(_))
            | Ok(Message::Binary(_))
            | Ok(Message::Frame(_)) => {}
            Ok(Message::Close(_)) => {
                println!("Connection closed.");
                return;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

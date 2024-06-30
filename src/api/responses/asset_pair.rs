/// The expected trading asset pair response.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetPair {
    pub altname: String,
    pub wsname: String,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub lot: String,
    pub pair_decimals: i32,
    pub lot_decimals: i32,
    pub lot_multiplier: i32,
    pub leverage_buy: Vec<i32>,
    pub leverage_sell: Vec<i32>,
    pub fees: Vec<Vec<f64>>,
    pub fees_maker: Vec<Vec<f64>>,
    pub fee_volume_currency: String,
    pub margin_call: i32,
    pub margin_stop: i32,
    pub cost_decimals: i32,
    pub ordermin: String,
    pub costmin: String,
    pub tick_size: String,
    pub status: String,
    pub long_position_limit: i32,
    pub short_position_limit: i32,
}

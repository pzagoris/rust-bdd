use std::collections::HashMap;
/** The response from the Account Open Orders
 *
 *  Instead of structs for deserialization, we can handle the response as a
 *  string or json value (anad use jsonpath for checking for specific values.
 *
 *
 */
#[derive(Debug, serde::Deserialize)]
pub struct AccountOpenOrders {
    pub open: HashMap<String, OpenOrder>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenOrder {
    refid: String,
    userref: u32,
    status: Status,
    opentm: f64,
    starttm: u64,
    expiretm: u64,
    descr: Description,
    vol: String,
    vol_exec: String,
    cost: String,
    fee: String,
    price: String,
    stopprice: String,
    limitprice: String,
    misc: String,
    oflags: String,
    trades: Option<Vec<String>>,
    cl_ord_id: Option<String>,
    trigger: Option<String>,
    margin: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pending,
    Open,
    Closed,
    Canceled,
    Expired,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Description {
    pair: String,
    #[serde(rename = "type")]
    kind: Type,
    ordertype: OrderType,
    price: String,
    price2: String,
    leverage: String,
    order: String,
    close: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Buy,
    Sell,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
}

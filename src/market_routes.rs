use crate::*;
use actix_web::get;
use actix_web::Responder;

#[get("/market")]
pub(crate) async fn market() -> impl Responder {
    match fetch_market_data().await {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}
#[get("/market/{id}")]
async fn market_by_id(id: web::Path<String>) -> impl Responder {
    match fetch_market_data_with_id(id.to_string()).await {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

async fn fetch_market_data() -> Result<String, reqwest::Error> {
    let method_name = "get_trait_market_data";
    let args_json = &serde_json::json!({});
    let body_string = helper::create_query_body(CONTRACT_ID, method_name, args_json);
    //body to base64
    let res = helper::http_get(NEAR_URL, body_string).await?;
    //convert to json
    let v: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&res);
    match v {
        Ok(v) => Ok(helper::parse_result(v, "[]".to_string())),
        Err(e) => Ok(e.to_string()),
    }
}

async fn fetch_market_data_with_id(id: String) -> Result<String, reqwest::Error> {
    let method_name = "get_trait_market_data_by_id";
    let args_json = &serde_json::json!({ "trait_id": id });

    let body_string = helper::create_query_body(CONTRACT_ID, method_name, args_json);

    let res = helper::http_get(NEAR_URL, body_string).await?;

    let v: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&res);
    match v {
        Ok(v) => Ok(helper::parse_result(v, "{}".to_string())),
        Err(e) => Ok(e.to_string()),
    }
}

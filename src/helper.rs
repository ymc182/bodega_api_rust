pub async fn http_get(url: &str, body_string: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .body(body_string)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
pub fn parse_result(result: serde_json::Value, empty_value: String) -> String {
    if result["result"]["result"].is_null() {
        return empty_value;
    }
    let result = result["result"]["result"].to_string();
    let result: Vec<u8> = serde_json::from_str(&result.to_string()).unwrap();
    let result = String::from_utf8(result).unwrap();
    result
}

pub fn create_query_body(contract_id: &str, method_name: &str, args: &serde_json::Value) -> String {
    let args_base64 = base64::encode(&serde_json::to_vec(args).unwrap());
    let body = &serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": "dontcare",
                    "method": "query",
                    "params": {
                                    "request_type": "call_function",
                                    "finality": "final",
                                    "account_id": contract_id,
                                    "method_name": method_name,
                                    "args_base64": args_base64
                    }
    });
    //convert body to string
    let body_string = serde_json::to_string(body).unwrap();
    body_string
}

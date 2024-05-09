use std::vec;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext, TransformFunc,
}; 

use ic_cdk_macros::{self, query, update}; 
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)] 
struct Context {
    bucket_start_time_index: usize, 
    closing_price_index: usize, 
} 

#[ic_cdk::update] 
async fn get_btc_usd_price() -> String {
    let url = format!("https://api.coinbase.com/v2/exchange-rates?currency=BTC"); 
    // let host = format!("https://api.coinbase.com/"); 

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(), 
            value: "api.coinbase.com".to_string(),
        }, 
        HttpHeader {
            name: "User-Agent".to_string(), 
            value: "rust_http_get_backend".to_string(), 
        }, 
    ]; 

    // Create the request 
    let request = CanisterHttpRequestArgument {
        url: url.to_string(), 
        method: HttpMethod::GET, 
        headers: request_headers, 
        body: None, 
        max_response_bytes: None, 
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(), 
                method: "transform".to_string()
            }), 
            context: vec![]
        }),  
        // transform: Some(TransformContext::new(transform, sercer_json::to_vec(&context).unwrap())), 
    }; 

    // Send the request 
    let cycles = 230_949_972_000; 

    let response = http_request(request, cycles).await; 

    // Handle the response 
    match response {
        Ok((HttpResponse { body, .. },)) => {
            let body_str = String::from_utf8(body).unwrap(); 

            // Parse the JSON string into serde_json::Value
            let parsed: Value = serde_json::from_str(&body_str).unwrap(); 

            // Access the fields you're interested in 
            let kes = parsed["data"]["rates"]["KES"].as_str().unwrap(); 
            let usd = parsed["data"]["rates"]["KES"].as_str().unwrap();   

            format!("KES: {}, USD: {} ", kes, usd)
        }
        // Ok(HttpResponse::Fail { body, .. }) => {
        //     format!("Request failed with body: {:?}", body)
        // } 
        Err(err) => {
            // Handling the error 
            format!("Request error: {:?}", err)
        }
    }
    
}

#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse { 
    let headers = vec![
        HttpHeader {
            name:  "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        }, 
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        }, 
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ]; 

    let mut res = HttpResponse {
        status: raw.response.status.clone(), 
        body: raw.response.body.clone(), 
        headers
    }; 

    if res.status == 200u64 {
        res.body = raw.response.body; 
    } else {
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
    }

    res 
}

ic_cdk::export_candid!();

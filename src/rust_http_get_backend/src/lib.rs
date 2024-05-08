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
    let host = format!("https://api.coinbase.com/"); 

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(), 
            value: format!("{host}:433")
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
    }; 

    // Send the request 
    let cycles = 230_949_972_000; 

    let response = http_request(request, cycles).await; 

    // Handle the response 
    match response {
        Ok((HttpResponse { body, .. },)) => {
            let body_str = String::from_utf8(body).unwrap(); 
            // return the body string 
            body_str 
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

ic_cdk::export_candid!();

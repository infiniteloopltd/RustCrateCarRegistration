extern crate curl;
extern crate base64;
extern crate serde_json;

use curl::easy::{Easy, List};
use base64::{encode};
use serde_json::Value;

pub fn european_lookup(endpoint: &str, registration_number: &str, username: &str, password: &str ) -> Value {
    let url = format!("https://www.regcheck.org.uk/api/json.aspx/{}/{}",endpoint, registration_number);
    return general_lookup(&url, username, password);
}

pub fn usa_lookup(registration_number: &str, state: &str, username: &str, password: &str ) -> Value {
    let url = format!("https://www.regcheck.org.uk/api/json.aspx/CheckUSA/{}/{}", registration_number, state);
    return general_lookup(&url, username, password);
}

pub fn australian_lookup(registration_number: &str, state: &str, username: &str, password: &str ) -> Value {
    let url = format!("https://www.regcheck.org.uk/api/json.aspx/CheckAustralia/{}/{}", registration_number, state);
    return general_lookup(&url, username, password);
}

fn general_lookup(api: &str, username: &str, password: &str ) -> Value {
    let mut easy = Easy::new();

    easy.url(api).unwrap();

    let auth = format!("{}:{}",username,password);
    let b64 = encode(&auth);
    let basic_auth = format!("Authorization: Basic {}", b64);
    let mut list = List::new();
    list.append(&basic_auth).unwrap();
    easy.http_headers(list).unwrap();


    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html = String::from_utf8(Vec::from(data)).unwrap();
            Ok(data.len())
        }).unwrap();

        transfer.perform().unwrap();
    };

    let json: Value = serde_json::from_str(&html).unwrap_or_else(|e| {
       panic!("Failed to parse json; error is {}", e);
    });
    //println!("{}",html);
    return json;
}

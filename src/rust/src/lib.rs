use extendr_api::prelude::*;
// use std::collections::HashMap;
use url::Url;

/// @export
#[extendr]
#[derive(Debug, IntoDataFrameRow)]
struct ParsedUrl {
    url: String,
    scheme: String,
    host: String,
    port: String,
    path: String,
    query: String,
    fragment: String,
    username: String,
    password: String
}

/// Parse a vector of URLs and return a DataFrame with parsed components
#[extendr]
fn rs_url_parse(urls: Vec<String>) -> Dataframe<ParsedUrl> {
    let mut parsed_urls = Vec::new();

    for url in urls {
        match Url::parse(&url) {
            Ok(parsed_url) => {
                parsed_urls.push(ParsedUrl {
                    url: url.to_string(),
                    scheme: parsed_url.scheme().to_string(),
                    host: parsed_url.host_str().unwrap_or("").to_string(),
                    port: parsed_url.port().map_or("".to_string(), |p| p.to_string()),
                    path: parsed_url.path().to_string(),
                    query: parsed_url.query().unwrap_or("").to_string(),
                    fragment: parsed_url.fragment().unwrap_or("").to_string(),
                    username: parsed_url.username().to_string(),
                    password: parsed_url.password().unwrap_or("").to_string()
                });
            }
            Err(_) => {
                parsed_urls.push(ParsedUrl {
                    url: url.to_string(),
                    scheme: "".to_string(),
                    host: "".to_string(),
                    port: "".to_string(),
                    path: "".to_string(),
                    query: "".to_string(),
                    fragment: "".to_string(),
                    username: "".to_string(),
                    password: "".to_string()
                });
            }
        }
    }
    parsed_urls.into_dataframe().unwrap()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod urlparser;
    fn rs_url_parse;
}

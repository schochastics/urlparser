use extendr_api::prelude::*;
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
  password: String,
}

#[extendr]
fn url_parse(urls: Vec<String>) -> Dataframe<ParsedUrl> {
  urls.into_iter()
    .map(|url| {
      let parsed_url = Url::parse(&url);
      let (scheme, host, port, path, query, fragment, username, password) =
        parsed_url.as_ref().ok().map_or_else(
          || Default::default(),
          |p| (
            p.scheme().to_string(),
            p.host_str().unwrap_or("").to_string(),
            p.port().map_or_else(String::new, |p| p.to_string()),
            p.path().to_string(),
            p.query().unwrap_or("").to_string(),
            p.fragment().unwrap_or("").to_string(),
            p.username().to_string(),
            p.password().unwrap_or("").to_string(),
          ),
        );

        ParsedUrl {
            url,
            scheme,
            host,
            port,
            path,
            query,
            fragment,
            username,
            password,
        }
        })
        .collect::<Vec<_>>() 
        .into_dataframe() 
        .unwrap()
}

extendr_module! {
    mod urlparser;
    fn url_parse;
}

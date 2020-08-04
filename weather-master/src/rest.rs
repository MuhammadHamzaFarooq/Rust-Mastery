use command::Query;
use error::*;
use model::*;
use reqwest::Client;
use serde_json as json;

pub fn query(query: &Query) -> Result<Weather> {
    use API_KEY;
    use std::io::Read;

    let url = ApiUrl::new(API_KEY, query);
    let response = {
        let mut response = Client::new().get(&url.into_url()).send()?;
        let mut buf = String::new();

        // This operation can only fail if we have received something other than ASCII or
        // UTF-8 from the server.
        response.read_to_string(&mut buf).expect("Received invalid data");

        buf
    };

    json::from_str(&response).map_err(|e| {

        // If we fail to deserialize a weather response, we will attempt to deserialize an API
        // error instead--but, in the event that fails, we will return the original serialization
        // error instead of the new one.
        match json::from_str::<ApiError>(&response) {
            Ok(e) => Error::api(e),
            _ => Error::serialization(e),
        }

    })
}

#[derive(Debug)]
struct ApiUrl<'a> {
    key: &'a str,
    query: &'a Query,
}

impl<'a> ApiUrl<'a> {
    fn new(key: &'a str, query: &'a Query) -> Self {
        Self { key, query }
    }

    fn into_url(self) -> String {
        let Self { key, query } = self;
        format!("http://api.openweathermap.org/data/2.5/weather?APPID={}&{}", key, query)
    }
}

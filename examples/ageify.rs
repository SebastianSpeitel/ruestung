#![allow(dead_code)]
use ruestung::prelude::*;

#[derive(Debug, serde::Deserialize)]
struct Response {
    name: String,
    age: u8,
    count: u32,
}

struct Name(String);
impl Query for Name {
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        builder
            .request_mut()
            .url_mut()
            .query_pairs_mut()
            .append_pair("name", self.0.as_str());
    }
}

struct Agify;
impl Endpoint for Agify {
    const URL: &'static str = "https://api.agify.io";
    type Query = (Required<Name>,);
    type Response = Response;
}

#[tokio::main]
async fn main() {
    let api = Api::<Agify, _>::new();

    // Removing the following line will result in a compile error
    let sebastian = api.set(Name("sebastian".to_string()));

    let sebastian = sebastian.into_future().await.unwrap();

    dbg!(sebastian);
}

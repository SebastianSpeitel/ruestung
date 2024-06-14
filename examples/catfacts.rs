#![allow(dead_code)]
use ruestung::prelude::*;

#[derive(Debug, serde::Deserialize)]
struct CatFact {
    fact: String,
    length: usize,
}

struct Fact;
impl Endpoint for Fact {
    const URL: &'static str = "https://catfact.ninja/fact";
    type Query = ();
    type Response = CatFact;
}

#[tokio::main]
async fn main() {
    let api = Api::<Fact, _>::new();

    let fact = api.into_future().await.unwrap();

    dbg!(fact);
}

#![allow(dead_code)]
use ruestung::prelude::*;
use ruestung::reqwest::Client;

#[derive(Debug, serde::Deserialize)]
struct Resources {
    films: String,
    people: String,
    planets: String,
    starships: String,
}

#[derive(Debug, serde::Deserialize)]
struct Starship {
    name: String,
    model: String,
}

#[derive(Debug, serde::Deserialize)]
struct Planet {
    name: String,
    climate: String,
}

#[derive(Debug, Clone, Copy)]
struct Id(usize);

impl Query for Id {
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        builder
            .request_mut()
            .url_mut()
            .path_segments_mut()
            .unwrap()
            .push(self.0.to_string().as_str());
    }
}

#[derive(Debug, Clone)]
struct Root;
impl Endpoint for Root {
    const URL: &'static str = "https://swapi.dev/api";
    type Query = ();
    type Response = Resources;
}

#[derive(Debug, Clone)]
struct Starships;
impl Endpoint for Starships {
    const URL: &'static str = "https://swapi.dev/api/starships/";
    type Query = (Required<Id>,);
    type Response = Starship;
}

#[derive(Debug, Clone)]
struct Planets;
impl Endpoint for Planets {
    const URL: &'static str = "https://swapi.dev/api/planets/";
    type Query = (Required<Id>,);
    type Response = Planet;
}

#[tokio::main]
async fn main() {
    // Set credentials and headers on client
    let client = Client::builder().build().unwrap();
    let swapi = Api::<Root, _>::new().set_client(client);

    let planets = swapi.as_new::<Planets>();
    let starships = swapi.as_new::<Starships>();

    let tatooine = planets.set(Id(1)).into_future().await.unwrap();
    let deathstar = starships.set(Id(9)).into_future().await.unwrap();

    dbg!(tatooine);
    dbg!(deathstar);
}

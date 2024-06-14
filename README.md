# Ruestung

Typesafe REST client builder.

## Example

```rust
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
```

More complex examples can be found in the `examples` directory.

## Name

`Rust` + `REST` = `ruest` (already taken) => `ruestung` (german word for armor)

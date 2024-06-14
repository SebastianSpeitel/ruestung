pub mod api;
pub mod endpoint;
pub mod join;
pub mod query;
pub mod requestbuilder;
pub mod takes;

#[doc(inline)]
pub use api::Api;
#[doc(inline)]
pub use endpoint::Endpoint;
#[doc(inline)]
pub use query::Query;
pub use reqwest;

pub mod prelude {
    pub use crate::api::Api;
    pub use crate::endpoint::{Endpoint, EndpointExt};
    pub use crate::query::{Optional, Query, Required};
    pub use crate::requestbuilder::RequestBuilder;
    pub use reqwest::{Method, StatusCode};
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn basic() {
        struct Id(String);
        impl Query for Id {
            #[inline]
            fn build(&self, builder: &mut RequestBuilder) {
                builder
                    .request_mut()
                    .url_mut()
                    .query_pairs_mut()
                    .append_pair("id", self.0.as_str());
            }
        }

        struct MyEndpoint;
        impl Endpoint for MyEndpoint {
            const URL: &'static str = "https://example.com/foo";
            type Query = (Required<Id>,);
            type Response = ();
        }

        let api = Api::<MyEndpoint, _>::new();

        let api = api.set(Id("foo".to_string()));

        let _fut = api.into_future();
    }
}

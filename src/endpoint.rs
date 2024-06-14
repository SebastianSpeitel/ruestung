use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::{query::Query, requestbuilder::RequestBuilder, takes::Takes};

/// Definition of an endpoint
///
/// # Example
///
/// ```rust
/// use ruestung::prelude::*;
///
/// struct MyEndpoint;
/// impl Endpoint for MyEndpoint {
///     const URL: &'static str = "https://example.com";
///     const METHOD: Method = Method::POST;
///     type Query = ();
///     type Response = ();
/// }
///
/// let api = Api::<MyEndpoint, _>::new();
/// ```
pub trait Endpoint {
    /// The URL of the endpoint
    ///
    /// # Panics
    /// Panics if the URL is not valid
    const URL: &'static str;
    /// The HTTP method of the endpoint
    const METHOD: Method = Method::GET;
    /// The query of the endpoint
    ///
    /// # Example
    ///
    /// ```rust
    /// use ruestung::prelude::*;
    /// # struct Foo;
    /// # impl Query for Foo {};
    /// # struct Bar;
    /// # impl Query for Bar {};
    ///
    /// type FooBarQuery = (Required<Foo>, Optional<Bar>);
    /// ```
    type Query;
    /// Response type of the endpoint
    ///
    /// Should implement `serde::de::DeserializeOwned`
    type Response;
}

pub trait EndpointExt: Endpoint {
    #[inline]
    fn set<T, const SLOT: u8>(self, param: T) -> <Self as Takes<T, SLOT>>::Taken
    where
        Self: Takes<T, SLOT> + Sized,
    {
        self.take(param)
    }

    #[inline]
    fn into_future(
        self,
    ) -> impl std::future::Future<Output = reqwest::Result<Self::Response>> + Send + Sync
    where
        Self: Query + Send + Sync,
        Self::Response: DeserializeOwned,
    {
        async move {
            // Raname self to query because that's how it's used
            let query = self;

            let mut builder = RequestBuilder::new::<Self>();
            query.build(&mut builder);

            // First try
            let client = builder.client.unwrap_or_default();
            let mut resp = client.execute(builder.request).await?;

            if builder.retry_on.contains(&resp.status()) {
                // Reset builder
                builder = RequestBuilder::new::<Self>();
                // Provide client to builder
                builder.client_mut().replace(client);
                query.build(&mut builder);

                // Second try
                let client = builder.client.unwrap_or_default();
                resp = client.execute(builder.request).await?;
            }

            resp.json().await
        }
    }
}

impl<E: Endpoint> EndpointExt for E {}

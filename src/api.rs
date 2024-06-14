use std::marker::PhantomData;

use reqwest::{Client, Method};

use crate::{
    endpoint::Endpoint, join::Join, query::Query, requestbuilder::RequestBuilder, takes::Takes,
};

/// Representation an endpoint and its query
///
/// # Example
///
/// ```rust
/// use ruestung::prelude::*;
///
/// struct Foo(String);
/// impl Query for Foo {
///     fn build(&self, builder: &mut RequestBuilder) {
///         builder.request_mut().url_mut().query_pairs_mut().append_pair("foo", &self.0);
///     }
/// }
///
/// struct MyEndpoint;
/// impl Endpoint for MyEndpoint {
///     const URL: &'static str = "https://example.com";
///     type Query = (Required<Foo>,);
///     type Response = ();   
/// }
///
/// let api = Api::<MyEndpoint, _>::new();
///
/// let with_foo = api.set(Foo("bar".to_string()));
///
/// ```
#[derive(Debug, Clone)]
pub struct Api<E, Q> {
    endpoint: PhantomData<E>,
    query: Q,
}

impl<E, Q> Api<E, Q> {
    /// Construct a new API instance for the given endpoint
    ///
    /// # Example
    ///
    /// ```rust
    /// use ruestung::prelude::*;
    ///
    /// struct MyEndpoint;
    /// impl Endpoint for MyEndpoint {
    ///     const URL: &'static str = "https://example.com";
    ///     type Query = ();
    ///     type Response = ();
    /// }
    ///
    /// let api = Api::<MyEndpoint, _>::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self
    where
        E: Endpoint<Query = Q>,
        Q: Default,
    {
        Self {
            endpoint: PhantomData,
            query: Q::default(),
        }
    }

    /// Construct a new API instance for the given endpoint,
    /// but keep already passed query parameters
    ///
    /// # Example
    ///
    /// ```rust
    /// use ruestung::prelude::*;
    /// use ruestung::reqwest::Client;
    ///
    /// struct Root;
    /// impl Endpoint for Root {
    ///     const URL: &'static str = "https://example.com";
    ///     type Query = ();
    ///     type Response = ();
    /// }
    ///
    /// struct Sub;
    /// impl Endpoint for Sub {
    ///     const URL: &'static str = "https://example.com/sub";
    ///     type Query = ();
    ///     type Response = ();
    /// }
    ///
    /// let client = Client::new();
    /// let api = Api::<Root, _>::new().set_client(client);
    ///
    /// // `sub` will have the client already set
    /// let sub = api.into_new::<Sub>();
    /// ```
    pub fn into_new<E2>(self) -> Api<E, <E2::Query as Join>::Left<Q>>
    where
        E2: Endpoint,
        E2::Query: Join + Default,
    {
        Api {
            endpoint: PhantomData,
            query: E2::Query::default().left(self.query),
        }
    }

    #[inline]
    pub fn set_client(self, client: Client) -> Api<E, Q::Left<Client>>
    where
        Q: Join,
    {
        Api {
            endpoint: self.endpoint,
            query: self.query.left(client),
        }
    }
}

impl<E, Q> Default for Api<E, Q>
where
    E: Endpoint<Query = Q>,
    Q: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<E, Q> Endpoint for Api<E, Q>
where
    E: Endpoint,
{
    const URL: &'static str = E::URL;
    const METHOD: Method = E::METHOD;
    type Query = E::Query;
    type Response = E::Response;
}

impl<E, Q> Query for Api<E, Q>
where
    E: Endpoint,
    Q: Query,
{
    #[inline]
    fn build(&self, request: &mut RequestBuilder) {
        self.query.build(request);
    }
}

impl<T, const S: u8, E, Q: Takes<T, S>> Takes<T, S> for Api<E, Q> {
    type Taken = Api<E, Q::Taken>;
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        Api {
            endpoint: PhantomData,
            query: self.query.take(param),
        }
    }
}

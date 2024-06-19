use std::marker::PhantomData;

use reqwest::{header::HeaderMap, Client, Method};

use crate::{requestbuilder::RequestBuilder, takes::Takes};

/// Maybe rename to RequestLike or PartialRequest?
#[diagnostic::on_unimplemented(
    // message = "My Message for `ImportantTrait` is not implemented for `{Self}`",
    // label = "My Label",
    note = "Maybe you are still missing a required argument?"
)]
pub trait Query: Sized {
    #[allow(unused_variables)]
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {}
}

impl Query for () {}
impl Query for Client {
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        builder.client_mut().replace(self.clone());
    }
}
impl Query for Method {
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.clone_into(builder.request_mut().method_mut());
    }
}
impl Query for HeaderMap {
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        builder
            .request_mut()
            .headers_mut()
            .extend(self.iter().map(|(k, v)| (k.to_owned(), v.to_owned())));
    }
}

/// A required query parameter
#[derive(Debug)]
pub struct Required<T>(PhantomData<T>);

impl<T> Default for Required<T> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for Required<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Takes<T> for Required<T> {
    type Taken = T;
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        param
    }
}

/// An optional query parameter
#[derive(Debug)]
pub struct Optional<T>(PhantomData<T>);

impl<T> Default for Optional<T> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for Optional<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Query for Optional<T> {}

impl<T> Takes<T> for Optional<T> {
    type Taken = T;
    #[inline]
    fn take(self, param: T) -> Self::Taken {
        param
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Form<T>(pub T);

impl<T> Query for Form<T>
where
    T: serde::Serialize,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        use reqwest::header::{HeaderValue, CONTENT_TYPE};
        const HEADER_VALUE: HeaderValue =
            HeaderValue::from_static("application/x-www-form-urlencoded");

        let request = builder.request_mut();
        if let Ok(body) = serde_urlencoded::to_string(&self.0) {
            request.body_mut().replace(body.into());
            request.headers_mut().insert(CONTENT_TYPE, HEADER_VALUE);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Json<T>(pub T);

impl<T> Query for Json<T>
where
    T: serde::Serialize,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        use reqwest::header::{HeaderValue, CONTENT_TYPE};
        const HEADER_VALUE: HeaderValue = HeaderValue::from_static("application/json");

        let request = builder.request_mut();
        if let Ok(body) = serde_json::to_vec(&self.0) {
            request.body_mut().replace(body.into());
            request.headers_mut().insert(CONTENT_TYPE, HEADER_VALUE);
        }
    }
}

impl<Q0> Query for (Q0,)
where
    Q0: Query,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.0.build(builder);
    }
}
impl<Q0, Q1> Query for (Q0, Q1)
where
    Q0: Query,
    Q1: Query,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.0.build(builder);
        self.1.build(builder);
    }
}
impl<Q0, Q1, Q2> Query for (Q0, Q1, Q2)
where
    Q0: Query,
    Q1: Query,
    Q2: Query,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
    }
}
impl<Q0, Q1, Q2, Q3> Query for (Q0, Q1, Q2, Q3)
where
    Q0: Query,
    Q1: Query,
    Q2: Query,
    Q3: Query,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
    }
}
impl<Q0, Q1, Q2, Q3, Q4> Query for (Q0, Q1, Q2, Q3, Q4)
where
    Q0: Query,
    Q1: Query,
    Q2: Query,
    Q3: Query,
    Q4: Query,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
    }
}
impl<Q0, Q1, Q2, Q3, Q4, Q5> Query for (Q0, Q1, Q2, Q3, Q4, Q5)
where
    Q0: Query,
    Q1: Query,
    Q2: Query,
    Q3: Query,
    Q4: Query,
    Q5: Query,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
    }
}
impl<Q0, Q1, Q2, Q3, Q4, Q5, Q6> Query for (Q0, Q1, Q2, Q3, Q4, Q5, Q6)
where
    Q0: Query,
    Q1: Query,
    Q2: Query,
    Q3: Query,
    Q4: Query,
    Q5: Query,
    Q6: Query,
{
    #[inline]
    fn build(&self, builder: &mut RequestBuilder) {
        self.0.build(builder);
        self.1.build(builder);
        self.2.build(builder);
        self.3.build(builder);
        self.4.build(builder);
        self.5.build(builder);
        self.6.build(builder);
    }
}

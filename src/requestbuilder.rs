use reqwest::{Client, Request, Response, StatusCode};

use crate::endpoint::Endpoint;

pub struct RequestBuilder {
    pub(crate) client: Option<Client>,
    pub(crate) request: Request,
    pub(crate) retry_on: Vec<StatusCode>,
    // Maybe Box this to reduce size of RequestBuilder?
    // because it is rarely used
    pub(crate) last_response: Option<Response>,
}

impl RequestBuilder {
    #[must_use]
    pub(crate) fn new<E: Endpoint>() -> Self {
        Self {
            client: None,
            request: Request::new(E::METHOD, E::URL.parse().unwrap()),
            retry_on: Vec::new(),
            last_response: None,
        }
    }

    /// Get a mutable reference of the optional client
    #[inline]
    pub fn client_mut(&mut self) -> &mut Option<Client> {
        &mut self.client
    }

    /// Get a mutable reference of the request
    ///
    /// # Example
    ///
    /// ```rust
    /// use ruestung::prelude::*;
    ///
    /// struct Id(String);
    /// impl Query for Id {
    ///     fn build(&self, builder: &mut RequestBuilder) {
    ///         builder.request_mut().url_mut().query_pairs_mut().append_pair("id", self.0.as_str());
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn request_mut(&mut self) -> &mut Request {
        &mut self.request
    }

    /// Add a status code to retry on
    #[inline]
    pub fn retry_on(&mut self, status: StatusCode) {
        self.retry_on.push(status);
    }

    /// Get the last response
    #[inline]
    pub fn last_response(&self) -> Option<&Response> {
        self.last_response.as_ref()
    }
}

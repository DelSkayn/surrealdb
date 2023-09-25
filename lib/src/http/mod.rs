//! Http client implementation

#![allow(dead_code)]

pub use lib_http::{
	header, method, status,
	uri::{self, Uri},
	version,
};
use lib_http::{
	header::{InvalidHeaderName, InvalidHeaderValue},
	method::InvalidMethod,
	Method, StatusCode,
};
use std::{convert::Infallible, string::FromUtf8Error};
use thiserror::Error;
use tokio::time::error::Elapsed;

mod url;
pub use url::{IntoUrl, Url};
mod builder;
pub use builder::{ClientBuilder, RedirectAction, RedirectPolicy};
mod request;
pub use request::Request;

#[cfg(not(target_arch = "wasm32"))]
pub mod hyper;
#[cfg(not(target_arch = "wasm32"))]
pub use hyper as backend;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm as backend;

use backend::{Backend, BackendError};
pub use backend::{BackendBody as Body, Response};

/// Errors from serializing and deserializing values.
#[derive(Error, Debug)]
pub enum SerializeError {
	#[error("{0}")]
	Json(#[from] serde_json::Error),
	#[error("{0}")]
	UrlDe(#[from] serde_urlencoded::de::Error),
	#[error("{0}")]
	UrlSer(#[from] serde_urlencoded::ser::Error),
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("{0}")]
	Url(#[from] url::UrlParseError),
	#[error("{0}")]
	Backend(#[from] BackendError),
	#[error("Failed to parse bytes to string: {0}")]
	FromUtf8(#[from] FromUtf8Error),
	#[error(
		"Invalid authorization token, authorization token could not be used as a header value"
	)]
	InvalidToken,
	#[error("Request returned error statuscode: {0}")]
	StatusCode(StatusCode),
	#[error("{0}")]
	InvalidHeaderName(#[from] InvalidHeaderName),
	#[error("{0}")]
	InvalidHeaderValue(#[from] InvalidHeaderValue),
	#[error("{0}")]
	InvalidMethod(#[from] InvalidMethod),
	#[error("Decoding error {0}")]
	Decode(SerializeError),
	#[error("Encoding error {0}")]
	Encode(SerializeError),
	#[error("Request timed out: {0}")]
	Timeout(#[from] Elapsed),
}

impl From<Infallible> for Error {
	fn from(_: Infallible) -> Self {
		panic!("Infallible error was created")
	}
}

/// A http client.
#[derive(Clone)]
pub struct Client {
	inner: Backend,
}

impl Client {
	/// Create a new http client with an all default config.
	pub fn new() -> Self {
		Self {
			inner: Backend::new(),
		}
	}

	/// Return a builder for configuring the http client.
	pub fn builder() -> ClientBuilder {
		ClientBuilder::new()
	}

	/// Create a new get request.
	///
	/// Returns an error if the url is invalid.
	pub fn get<U: IntoUrl>(&self, url: U) -> Result<Request, Error> {
		let url = url.to_url()?;
		Request::new(Method::GET, url, self.clone())
	}

	/// Create a new post request.
	///
	/// Returns an error if the url is invalid.
	pub fn post<U: IntoUrl>(&self, url: U) -> Result<Request, Error> {
		let url = url.to_url()?;
		Request::new(Method::POST, url, self.clone())
	}

	/// Create a new head request.
	///
	/// Returns an error if the url is invalid.
	pub fn head<U: IntoUrl>(&self, url: U) -> Result<Request, Error> {
		let url = url.to_url()?;
		Request::new(Method::HEAD, url, self.clone())
	}

	/// Create a new put request.
	///
	/// Returns an error if the url is invalid.
	pub fn put<U: IntoUrl>(&self, url: U) -> Result<Request, Error> {
		let url = url.to_url()?;
		Request::new(Method::PUT, url, self.clone())
	}

	/// Create a new patch request.
	///
	/// Returns an error if the url is invalid.
	pub fn patch<U: IntoUrl>(&self, url: U) -> Result<Request, Error> {
		let url = url.to_url()?;
		Request::new(Method::PATCH, url, self.clone())
	}

	/// Create a new delete request.
	///
	/// Returns an error if the url is invalid.
	pub fn delete<U: IntoUrl>(&self, url: U) -> Result<Request, Error> {
		let url = url.to_url()?;
		Request::new(Method::DELETE, url, self.clone())
	}
}

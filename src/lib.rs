#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod request;
pub mod response;

pub use crate::{request::RequestData, request::RequestDataStruct, response::ResponseData, response::ResponseDataStruct};

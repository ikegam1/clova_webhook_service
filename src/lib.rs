#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod request;
mod response;
pub use crate::{request::RequestData, request::RequestDataStruct, response::ResponseData, response::ResponseDataStruct};


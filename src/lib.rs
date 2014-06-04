#![crate_id = "curl"]
#![feature(macro_rules)]

extern crate collections;
extern crate libc;

use ffi::{easy,opt};
use ffi::err::ErrCode;
use handle::Handle;

pub use response::{Headers,Response};

mod ffi;
mod handle;
mod header;
mod response;

pub fn request() -> Request {
  Request::new()
}

pub fn get(uri: &str) -> Result<Response, ErrCode> {
  request()
    .method(Get)
    .uri(uri)
    .execute()
}

pub fn head(uri: &str) -> Result<Response, ErrCode> {
  request()
    .method(Head)
    .uri(uri)
    .execute()
  /*
  let mut handle = Handle::new();
  handle.setopt(opt::URL, uri);
  handle.setopt(opt::NOBODY, 1);
  handle.perform()
  */
}

pub enum Method {
  Options,
  Get,
  Head,
  Post,
  Put,
  Delete,
  Trace,
  Connect
}

pub struct Request {
  err: Option<ErrCode>,
  handle: Handle,
  headers: ffi::List
}

impl Request {
  fn new() -> Request {
    Request {
      err: None,
      handle: Handle::new(),
      headers: ffi::List::new()
    }
  }

  pub fn method(self, method: Method) -> Request {
    match method {
      Get => {} // Nothing to do
      _ => { unimplemented!() }
    }
    self
  }

  pub fn header(mut self, name: &str, val: &str) -> Request {
    self.append_header(name, val);
    self
  }

  pub fn headers<'a, I: Iterator<(&'a str, &'a str)>>(mut self, mut hdrs: I) -> Request {
    for (name, val) in hdrs {
      self.append_header(name, val);
    }

    self
  }

  fn append_header(&mut self, name, &str, val: &str) {
    let mut c_str = Vec::with_capacity(name.len() + val.len() + 2);
    c_str.push_all(name.as_bytes());
    c_str.push(':' as u8);
    c_str.push_all(val.as_bytes());
    c_str.push(0);

    self.headers.push_bytes(c_str.as_slice());
  }

  pub fn uri(mut self, uri: &str) -> Request {
    match self.handle.setopt(opt::URL, uri) {
      Ok(_) => {}
      Err(e) => self.err = Some(e)
    }

    self
  }

  pub fn execute(mut self) -> Result<Response, ErrCode> {
    match self.err {
      Some(e) => return Err(e),
      None => {}
    }

    if !self.headers.is_empty() {
      try!(self.handle.setopt(opt::HTTPHEADER, &self.headers));
    }

    self.handle.perform()
  }
}

#[cfg(hax)]
pub fn main() {
  let resp = get("https://www.bing.com").unwrap();
  println!("resp: {}", resp);
}

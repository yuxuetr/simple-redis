mod decode;
mod encode;
use enum_dispatch::enum_dispatch;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

#[enum_dispatch]
pub trait RespEncode {
  fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
  fn decode(buf: Self) -> Result<RespFrame, String>;
}

#[enum_dispatch(RespEncode)]
#[derive(Debug)]
pub enum RespFrame {
  SimpleString(SimpleString),
  Error(SimpleError),
  Integer(i64),
  BulkString(BulkString),
  NullBulkString(RespNullBulkString),
  Array(RespArray),
  Null(RespNull),
  NullArray(RespNullArray),
  Boolean(bool),
  Double(f64),
  Map(RespMap),
  Set(RespSet),
}

#[derive(Debug)]
pub struct SimpleString(String);
#[derive(Debug)]
pub struct SimpleError(String);
#[derive(Debug)]
pub struct BulkString(Vec<u8>);
#[derive(Debug)]
pub struct RespNull;
#[derive(Debug)]
pub struct RespArray(Vec<RespFrame>);
#[derive(Debug)]
pub struct RespNullArray;
#[derive(Debug)]
pub struct RespNullBulkString;
#[derive(Debug)]
pub struct RespMap(BTreeMap<String, RespFrame>);
#[derive(Debug)]
pub struct RespSet(Vec<RespFrame>);

impl Deref for SimpleString {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for SimpleError {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for BulkString {
  type Target = Vec<u8>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for RespArray {
  type Target = Vec<RespFrame>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for RespMap {
  type Target = BTreeMap<String, RespFrame>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for RespMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Deref for RespSet {
  type Target = Vec<RespFrame>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl SimpleString {
  pub fn new(s: impl Into<String>) -> Self {
    SimpleString(s.into())
  }
}

impl SimpleError {
  pub fn new(s: impl Into<String>) -> Self {
    SimpleError(s.into())
  }
}

impl BulkString {
  pub fn new(s: impl Into<Vec<u8>>) -> Self {
    BulkString(s.into())
  }
}

impl RespArray {
  pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
    RespArray(s.into())
  }
}

impl RespMap {
  pub fn new() -> Self {
    RespMap(BTreeMap::new())
  }
}

impl Default for RespMap {
  fn default() -> Self {
    RespMap::new()
  }
}

impl RespSet {
  pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
    RespSet(s.into())
  }
}

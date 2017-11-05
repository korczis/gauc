#![feature(test)]
extern crate test;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate hyper;
extern crate iron;

#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate params;
extern crate router;
extern crate serde;

#[macro_use]
extern crate serde_json;


pub mod cli;
pub mod client;
pub mod couchbase;
pub mod web;

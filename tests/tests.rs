// #![deny(warnings)]

extern crate chrono;
extern crate mockito;
extern crate sherpa;
#[macro_use] extern crate serde_json;
extern crate tempdir;
extern crate time;

// Module full of support functions and structs for integration tests
mod support;

mod auth;
mod config;
mod deploy;

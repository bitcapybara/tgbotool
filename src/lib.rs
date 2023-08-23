#![allow(dead_code)]

pub mod client;
pub mod command;
pub mod methods;
pub mod types;

pub use client::Client;
pub use tgbotool_derive::*;

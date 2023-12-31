#![allow(dead_code)]

use super::elastic::*;

// SECRET information
pub const ES_URL: &str = ES_URL_SECRET;
pub const ID: &str = ID_SECRET;
pub const PW: &str = PW_SECRET;

// INDICES for src/main.rs
pub const INDICES: [&str; 2] = [
    ".ds-winlogbeat-8.8.1-2023.08.16-000001",
    ".ds-winlogbeat-8.8.2-2023.08.06-000001",
];

// Search start and end timestamp
pub const TIMESTAMP_START: &str = "2023-08-06T15:00:00.000Z";
pub const TIMESTAMP_END: &str = "2023-09-07T02:00:00.000Z";

// Query size
pub const SIZE: usize = 100;

// File save location
pub const SAVELOCATION: &str = "/Users/dong-ju/Documents/My_code/elarocks/file/temp/event";
pub const CSVNAME: &str = "_logs.csv";

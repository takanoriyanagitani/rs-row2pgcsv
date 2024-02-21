#![forbid(clippy::unwrap_used)]

//! Simple array-to-string serializer for PostgreSQL COPY Command.
//!
//! Use external crate(e.g, csv) to serialize a row as a CSV row and import to PostgreSQL.
//!
//! ## What this crate can:
//!
//! - Serialize an array of numbers to a string(e.g, ``[1,2] => {1,2}``)
//! - Serialize a nested array of numbers to a string(e.g, ``[[1,2],[3,4]] => {{1,2},{3,4}}``)
//!
//! ## What this crate can't(don't):
//!
//! - Serialize an array of strings to a string
//! - Serialize a nested array of strings to a string
//! - Convert a string to a string which can be used for PostgreSQL COPY Command

pub mod arr;

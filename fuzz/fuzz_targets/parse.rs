#![no_main]
use libfuzzer_sys::fuzz_target;

use parcel_css::stylesheet::{ParserOptions, StyleSheet};

fuzz_target!(|contents: &str| {
  let _ = StyleSheet::parse(contents, ParserOptions::default());
});

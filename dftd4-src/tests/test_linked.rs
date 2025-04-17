extern crate dftd4_src;
use dftd4::prelude::*;

#[test]
fn test_linked() {
    let ver = dftd4_get_api_version();
    println!("DFTD4 version: {}", ver);
}

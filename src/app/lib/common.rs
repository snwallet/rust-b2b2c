
use chrono::prelude::*;

use crypto::digest::Digest;
use crypto::md5::Md5;

// #[allow(dead_code)]
pub fn get_timestamp() -> i64 {
    let dt = Local::now();
    dt.timestamp()
}


pub fn md5<S:Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

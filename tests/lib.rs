extern crate dt;
use dt::*;

// Integration tests

#[test]
fn test_init() {
    assert_eq!(pub_add(1, 2), 3);
}

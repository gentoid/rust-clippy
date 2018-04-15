#![feature(plugin)]
#![plugin(clippy)]
#![deny(too_long_function_body)]

const MAX_BODY_LENGTH: i32 = 1;

// too long function body case
fn too_long_function() -> i32 {
    doing_something_useful();
    but_it_s_too_long()
}

fn but_it_s_too_long() -> i32 {
    12
}

// with a comment
fn doing_something_useful() -> i32 {
    // we don't need to cound a line with comments
    42
}

// need a main anyway, use it get rid of unused warnings too
fn main() {
    assert!(too_long_function() > 1);
    assert!(but_it_s_too_long() > 1);
    assert!(doing_something_useful() > 1);
}

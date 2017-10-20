fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

pub fn test() {
    used_function();
}
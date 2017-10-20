#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("you are running linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("you are **not** running linux")
}

pub fn test() {
    are_you_on_linux();

    if cfg!(target_os = "linux") {
        println!("Yes, It's definitely linux!");
    } else {
        println!("yes, It's definitely **not** linux!");
    }
}
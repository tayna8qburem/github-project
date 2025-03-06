// A simple function that takes a string and returns it in uppercase
fn capitalize(s: &str) -> String {
    s.to_uppercase()
}

// A function that takes two numbers as arguments and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// A function that takes three numbers as arguments and returns the largest of them
fn max(a: i32, b: i32, c: i32) -> i32 {
    std::cmp::max(std::cmp::max(a, b), c)
}

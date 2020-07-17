#[get("/example")]
pub fn example() -> &'static str {
    "Hello, world!"
}
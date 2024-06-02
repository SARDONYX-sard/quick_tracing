// Need feature ["derive"]

// #[quick_tracing::init(level = "DEBUG")]
// #[quick_tracing::init(file = "./log/test.log", stdio, level = "DEBUG")]
#[quick_tracing::init]
fn main() {
    tracing::info!("Hello, world!");
}

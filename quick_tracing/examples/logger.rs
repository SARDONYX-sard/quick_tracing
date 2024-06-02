// Need feature ["derive"]

// #[quick_tracing::init]
// #[quick_tracing::init(level = "DEBUG")]
// #[quick_tracing::init(test = "test_case", stdio, level = "DEBUG")]
// #[quick_tracing::init(file = "./log/test.log", stdio, level = "DEBUG")]
//
// #[quick_tracing::try_init]
// #[quick_tracing::try_init(level = "DEBUG")]
// #[quick_tracing::try_init(test = "test_case", stdio, level = "DEBUG")]
#[quick_tracing::try_init(file = "./log/test.log", stdio, level = "DEBUG")]
fn main() -> std::io::Result<()> {
    tracing::info!("Hello, world!");
    Ok(())
}

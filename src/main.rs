mod srv;
mod bit;



fn main() {
    let _subscriber = tracing_subscriber::fmt::fmt()
        .with_level(true)
        .init();
    srv::init();
}

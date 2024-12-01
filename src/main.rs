mod srv;
mod bit;
mod dns;



fn main() {
    let _subscriber = tracing_subscriber::fmt::fmt()
        .with_level(true)
        .init();
    srv::init();
}

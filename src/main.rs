use tracing::Level;

mod srv;
mod bit;
mod dns;



fn main() {
    let _subscriber = tracing_subscriber::fmt::fmt()
        .with_max_level(Level::TRACE)
        .init();
    srv::init();
}

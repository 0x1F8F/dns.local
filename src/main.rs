mod srv;
mod bit;



fn main() {
    let subscriber = tracing_subscriber::fmt()
        .init();
    srv::init();
}

use std::net::TcpListener;

use mass_mailer::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}



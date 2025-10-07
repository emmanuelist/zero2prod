use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // First await the Future to get the Result<Server, std::io::Error>
    // Then use ? to unwrap the Result
    // Then await the Server
    run().await?.await
}
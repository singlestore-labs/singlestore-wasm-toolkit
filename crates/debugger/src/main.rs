use anyhow::Result;

mod handle;
mod server;

#[async_std::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // print usage if no args
    if args.len() < 3 {
        println!("Usage: {} <port> <path/to/foo.wasm>", args[0]);
        std::process::exit(1);
    }

    let port_number = args[1].parse::<u16>().unwrap();
    let wasm_path = args[2].clone();
    let wit_path = args.get(3);

    println!("debugging: {}", wasm_path);
    if wit_path.is_some() {
        println!("with wit: {}", wit_path.unwrap());
    }

    let factory = handle::HandleFactory::new(&wasm_path)?;

    server::listen_and_serve(port_number, factory).await
}

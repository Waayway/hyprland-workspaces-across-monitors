use std;

mod client;
mod server;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let server_mode = args.iter().any(|x| match x.as_ref() {
        "-d" => true,
        "--daemon" => true,
        _ => false,
    });
    if server_mode {
        server::main();
        return;
    } else {
        client::main(args);
        return;
    }
}

use std::io::{Read, Write};

const FINISH_TARGET_SIZE: usize = 1024 * 1024 * 1024 * 10; // 10GB
const BUFFER_SIZE: usize = 256;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} [client|server]", args[0]);
        std::process::exit(1);
    }
    let thread = match args[1].as_str() {
        "client" => std::thread::spawn(|| client()),
        "server" => std::thread::spawn(|| server()),
        _ => {
            println!("Usage: {} [client|server]", args[0]);
            std::process::exit(1);
        }
    };

    thread.join().unwrap();
}

fn client() {
    println!("Running client...");

    let mut stdout = std::io::stdout();

    let buffer: [u8; 256] = [0; BUFFER_SIZE];

    loop {
        let res = stdout.write(&buffer);
        if res.is_err() {
            break;
        }
    }
}

fn server() {
    println!("Running server...");

    let start = std::time::Instant::now();

    let mut total: usize = 0;
    let mut buffer = [0; BUFFER_SIZE];

    let mut stdin = std::io::stdin();

    while let Ok(n) = stdin.read(&mut buffer) {
        total += n;

        if total > FINISH_TARGET_SIZE {
            break;
        }
    }

    let elapsed = start.elapsed();
    println!("Total bytes read: {} GB", total as f64 / 1024.0 / 1024.0);
    println!("Elapsed time: {:?}", elapsed);
    println!(
        "Throughput: {} MB/s",
        total as f64 / 1024.0 / 1024.0 / elapsed.as_secs_f64()
    );
}

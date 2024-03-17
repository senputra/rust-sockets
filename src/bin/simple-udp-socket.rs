use std::{net::UdpSocket, process::exit};

const FINISH_TARGET_SIZE: usize = 1024 * 1024 * 1024 * 10; // 1GB
const BUFFER_SIZE: usize = 1024;

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
    let mut fail_count = 0;
    let socket = UdpSocket::bind("127.0.0.1:42068").unwrap();
    socket.connect("127.0.0.1:42069").unwrap();

    let buffer = [0; BUFFER_SIZE];

    loop {
        let res = socket.send(&buffer);
        if res.is_err() {
            fail_count += 1;
            println!("Error: {:?}", res.err());
            if fail_count > 10 {
                exit(0);
            }
            // break;
        }
    }
}

fn server() {
    println!("Running server...");

    let socket = UdpSocket::bind("127.0.0.1:42069").unwrap();

    let start = std::time::Instant::now();

    let mut total: usize = 0;
    let mut buffer = [0; BUFFER_SIZE];

    while let Ok(n) = socket.recv(&mut buffer) {
        total += n;
        if total > FINISH_TARGET_SIZE {
            break;
        }
    }

    let elapsed = start.elapsed();
    println!(
        "Total bytes read: {} GB",
        total as f64 / 1024.0 / 1024.0 / 1024.0
    );
    println!("Elapsed time: {:?}", elapsed);
    println!(
        "Throughput: {} MB/s",
        total as f64 / 1024.0 / 1024.0 / elapsed.as_secs_f64()
    );
}

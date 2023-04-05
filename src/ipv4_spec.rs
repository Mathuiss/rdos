#[derive(Debug)]
pub struct IPv4SpecParseError;

#[derive(Clone)]
pub struct IPv4Spec {
    host: String,
    port: u16,
}

impl IPv4Spec {
    // Simple implementation that parses string like localhost:8080 to IPv4Spec.
    // Throws ParseError on parse failure.
    pub fn parse(ip_address: String) -> Result<IPv4Spec, IPv4SpecParseError> {
        let sp: Vec<&str> = ip_address.split(':').collect();

        if sp.len() != 2 {
            return Err(IPv4SpecParseError);
        }

        return Ok(IPv4Spec {
            host: String::from(sp[0]),
            port: sp[1]
                .to_owned()
                .parse::<u16>()
                .map_err(|_| IPv4SpecParseError)?,
        });
    }

    pub fn get_host<'a>(&'a self) -> &'a String {
        return &self.host;
    }

    pub fn get_port(&self) -> u16 {
        return self.port;
    }
}

pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    server.clone().unwrap_or_else(|err| match security_level {
        Security::Unknown => return server.unwrap(),
        Security::High => return  server.unwrap_or("ERROR: program stops".to_string()),
        Security::Medium => return server.unwrap_or("WARNING: check the server".to_string()),        
        Security::Low => return server.unwrap_or("Not found: ".to_string() + &err),
        _ => server.to_string(),
    })
    server = match security_level {
        Security::BlockServer => return server.unwrap_err(),
        _ => server.to_string(),
    };
}
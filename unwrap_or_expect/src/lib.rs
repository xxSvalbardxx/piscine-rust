pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}


pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match &server {
        Err(e) => match security_level {
            Security::Unknown => server.unwrap(),
            Security::High => panic!("ERROR: program stops"),
            Security::Medium => return "WARNING: check the server".to_string(),
            Security::Low => return "Not found: ".to_string()+&e.to_string(),
            Security::BlockServer => server.unwrap(),
        
        },
        Ok(f) => match security_level {
            Security::BlockServer => server.unwrap_err(),
                _ => f.to_string(),
        }
        ,
    }
}
use unwrap_or_expect::*;

fn main() {
    println!("{}", fetch_data(Ok("server1.com".to_string()), Security::Medium));
    println!("{}", fetch_data(Err(String::new()), Security::Medium));
    println!("{}", fetch_data(Err("server2.com".to_string()), Security::Low));

    // Panics with no custom message
    //(Err("ERROR CRITICAL".to_string()), Security::Unknown);

    // Panics with the message "ERROR: program stops"
    //fetch_data(Err(String::new()), Security::High);

    // Panics with the message "malicious_server.com"
    //fetch_data(Ok("malicious_server.com".to_string()), Security::BlockServer);
}
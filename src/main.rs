use std::env;

fn parse_http_data(request: &str) -> &str {
    let parts: Vec<&str> = request.split("\r\n\r\n").collect();
    
    // The data (body) is usually after the empty line separating headers and body
    if parts.len() > 1 {
        return parts[1]; // Body of the request
    }

    ""
}

fn extract_url(request: &str) -> String {
    let mut host = "";
    let mut path = "";

    let lines: Vec<&str> = request.split("\r\n").collect();

    for line in &lines {
        if line.starts_with("POST") || line.starts_with("GET") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                path = parts[1];
            }
        } else if line.starts_with("Host:") {
            host = line.split_whitespace().nth(1).unwrap_or("");
        }
    }

    format!("https://{}{}", host, path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <program> <raw_http_request>");
        return;
    }

    let request = &args[1];
    let data = parse_http_data(request);
    let url = extract_url(request);
    println!("Parsed Data: {}", data);
    println!("Parsed URL: {}", url);
}

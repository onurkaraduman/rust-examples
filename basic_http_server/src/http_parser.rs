pub fn parse(http_content: String) -> (String, String) {
    let mut lines = http_content.lines();

    if let Some(line) = lines.next() {
        println!("{}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();

        let method = parts.get(0).unwrap();
        let path = parts.get(1).unwrap();

        println!("method: {}", method);
        println!("path: {}", path);
        (method.to_string(), path.to_string())
    } else {
        ("".to_string(), "".to_string())
    }
}
pub fn parse_str(target: &str, mut list: String, query: &str) -> String {
    for line in target.lines() {
        match line {
            s if s.contains(&query) => list.push_str(s),
            _ => ()
        }
    }
    list
}
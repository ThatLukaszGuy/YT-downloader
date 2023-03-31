pub fn make_path(name: &str) -> String {
    let mut full_path = String::from("needed_to_get_url/");
    full_path.push_str(name);
    full_path.push_str(".tmp");

    full_path
}
pub fn pad_str(s: &str, width: usize) -> String {
    return format!("{:^width$}", s, width = width);
}

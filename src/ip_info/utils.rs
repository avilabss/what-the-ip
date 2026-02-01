pub(crate) fn print_opt<T: std::fmt::Display>(label: &str, value: &Option<T>) {
    const W: usize = 10;
    if let Some(value) = value {
        println!("{:W$} {}", label, value);
    }
}

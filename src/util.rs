pub fn file_size_to_string(file_size: u64) -> String {
    let mut result = String::new();

    if file_size == 0 {
        return result
    }

    if file_size > 1024 && file_size < u64::pow(1024, 2) {
        let size = file_size / 1024;
        result.push_str(&size.to_string());
        result.push_str(" kB");
    } else if file_size > u64::pow(1024, 2) && file_size < u64::pow(1024, 3) {
        let size = file_size / u64::pow(1024, 2);
        result.push_str(&size.to_string());
        result.push_str(" MB");
    } else if file_size > u64::pow(1024, 3) && file_size < u64::pow(1024, 4) {
        let size = file_size / u64::pow(1024, 3);
        result.push_str(&size.to_string());
        result.push_str(" GB");
    } else {
        result.push_str(&file_size.to_string());
        result.push_str(" B");
    }

    result
}

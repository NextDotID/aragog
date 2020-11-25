const QUERY_IDENTFIERS: &[&str] = &["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];

pub fn get_str_identifier(mut current_index: usize) -> String {
    let mut res = String::new();
    let len = QUERY_IDENTFIERS.len();
    while current_index > len - 1 {
        let last_letter = current_index % len;
        current_index = current_index / len - 1;
        res = format!("{}{}", QUERY_IDENTFIERS[last_letter], res);
    }
    format!("{}{}", QUERY_IDENTFIERS[current_index], res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_single_letter() {
        assert_eq!(&get_str_identifier(0), "a");
        assert_eq!(&get_str_identifier(5), "f");
        assert_eq!(&get_str_identifier(10), "k");
    }

    #[test]
    fn increases_letter_count() {
        assert_eq!(get_str_identifier(10), "k");
        assert_eq!(get_str_identifier(11), "aa");
        assert_eq!(get_str_identifier(12), "ab");

        assert_eq!(get_str_identifier(21), "ak");
        assert_eq!(get_str_identifier(22), "ba");
        assert_eq!(get_str_identifier(23), "bb");

        assert_eq!(get_str_identifier(32), "bk");
        assert_eq!(get_str_identifier(33), "ca");
        assert_eq!(get_str_identifier(34), "cb");

        assert_eq!(get_str_identifier(131), "kk");
        assert_eq!(get_str_identifier(132), "aaa");
        assert_eq!(get_str_identifier(133), "aab");

        assert_eq!(get_str_identifier(1000000), "fbcceb");
    }
}

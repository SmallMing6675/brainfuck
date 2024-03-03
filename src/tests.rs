#[cfg(test)]
mod tests {

    use crate::expand;
    use crate::PathBuf;

    #[test]
    fn test_movright_macro() {
        let input_code = r"\movright\>>>\/movright/";
        let expected_output = ">>>";
        let expanded_code = expand(input_code, &PathBuf::new());
        assert_eq!(expanded_code, expected_output);
    }

    #[test]
    fn test_recursive_macro() {
        let input_code = r"\movright\+++/movright/\";
        let expanded_code = expand(input_code, &PathBuf::new());
    }
}

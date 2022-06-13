#[cfg(test)]
mod init_tests {
    #[test]
    fn cli_tests() {
        trycmd::TestCases::new().case("tests/cmd/**/*.toml");
    }
}

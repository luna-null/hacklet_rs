const VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn has_a_version_number() {
        assert!(!VERSION.is_empty(), "VERSION should not be nil");
    }
}

pub fn is_even_number(x: i32) -> bool {
    x % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_number() {
        let result = is_even_number(3);
        assert_eq!(result, false);
        let result = is_even_number(4);
        assert_eq!(result, true);
    }
}

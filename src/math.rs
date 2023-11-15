pub fn is_even_number(x: i32) -> bool {
    x % 2 == 0
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_number() {
        let result: bool = is_even_number(3);
        assert_eq!(result, false);
        let result: bool = is_even_number(4);
        assert_eq!(result, true);
    }

    #[test]
    fn test_add() {
        let result: usize = add(2, 2);
        assert_eq!(result, 4);
    }
}

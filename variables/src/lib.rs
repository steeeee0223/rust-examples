pub fn add_one(x: i32) -> i32 {
    println!("💡 Adding one to {x}");
    return x + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let result: i32 = add_one(5);
        assert_eq!(result, 6);
    }
}

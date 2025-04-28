// Testing in Rust
// This example demonstrates different types of tests and testing patterns

// Function to be tested
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function that might panic
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

// Function that returns Result
pub fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("Failed to parse '{}' as a number", s)),
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Basic test
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    // Test that should panic
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    // Test with Result
    #[test]
    fn test_parse_number() {
        assert!(parse_number("42").is_ok());
        assert!(parse_number("abc").is_err());
        
        // Using Result's unwrap in tests
        assert_eq!(parse_number("42").unwrap(), 42);
        
        // Using Result's expect in tests
        assert_eq!(
            parse_number("42").expect("Failed to parse number"),
            42
        );
    }

    // Test with custom error messages
    #[test]
    fn test_with_messages() {
        let result = add(2, 2);
        assert_eq!(
            result,
            4,
            "Expected 2 + 2 to be 4, but got {}",
            result
        );
    }

    // Test with setup and teardown
    #[test]
    fn test_with_setup() {
        // Setup
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        
        // Test
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        
        // Teardown happens automatically when vec goes out of scope
    }

    // Test with multiple assertions
    #[test]
    fn test_multiple_assertions() {
        let result = add(2, 2);
        assert!(result > 0);
        assert!(result < 10);
        assert_eq!(result, 4);
    }

    // Test with ignored attribute
    #[test]
    #[ignore]
    fn test_ignored() {
        // This test will be ignored unless specifically run
        assert_eq!(add(1, 1), 2);
    }

    // Test with custom test name
    #[test]
    fn test_with_custom_name() {
        assert_eq!(add(1, 1), 2);
    }
}

// Integration tests
#[cfg(test)]
mod integration_tests {
    use super::*;

    // Integration test
    #[test]
    fn test_add_and_divide() {
        let sum = add(10, 5);
        let result = divide(sum, 3);
        assert_eq!(result, 5);
    }

    // Test with multiple operations
    #[test]
    fn test_parse_and_add() {
        let num1 = parse_number("10").unwrap();
        let num2 = parse_number("20").unwrap();
        assert_eq!(add(num1, num2), 30);
    }
}

// Documentation tests
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn documented_add(a: i32, b: i32) -> i32 {
    a + b
}

// Test helpers
#[cfg(test)]
mod test_helpers {
    use super::*;

    // Helper function for tests
    fn setup_test_data() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }

    // Test using helper function
    #[test]
    fn test_with_helper() {
        let data = setup_test_data();
        assert_eq!(data.len(), 5);
        assert_eq!(data[0], 1);
    }
} 
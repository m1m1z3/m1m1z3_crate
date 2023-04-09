//! # My crate
//! This crate is just for learning about cargo and crate publishing.
//! 

/// # Example
/// This functions takes a string an returns a greeting from me.
///
/// ```
/// let greeting = m1m1z3::hello_from_m1m1mz3("John Doe");
/// assert_eq!(greeting,  "Hello from m1m1z3 to John Doe!".to_string());
/// ```
/// # Panics
///  There are no panics.
/// 
/// # Errors
/// There is always a result so no errors.
/// 

pub fn hello_from_m1m1mz3(to: &str) -> String {
    format!("Hello from m1m1z3 to {}!", to).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello_from_m1m1mz3("world");
        assert_eq!(result, "Hello from m1m1z3 to world!".to_string());
    }
}

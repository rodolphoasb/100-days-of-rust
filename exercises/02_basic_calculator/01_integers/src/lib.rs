fn compute(a: u32, b: u32) -> u32 {
    // TODO: change the line below to fix the compiler error and make the tests pass.
    a + b * 4u32
    // OR: a + b * 4
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}

// In Rust, when performing arithmetic operations, the operands must be of the same type.
// Rust doesn't implicitly convert between integer types to prevent potential data loss or unexpected behavior.
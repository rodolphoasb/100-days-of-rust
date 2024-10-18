// TODO: fix the function signature below to make the tests pass.
//  Make sure to read the compiler error message—the Rust compiler is your pair programming
//  partner in this course and it'll often guide you in the right direction!
//
// The input parameters should have the same type of the return type.
fn compute(a: u32, b: u32) -> u32 {
    // Don't touch the function body.
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}

// u32 in Rust represents an unsigned 32-bit integer.
// "u" stands for unsigned, meaning it can only represent non-negative numbers (0 and positive integers).
// "32" indicates the size of the integer, specifying that it occupies 32 bits in memory.
// This type can store integer values from 0 to 4,294,967,295 (2^32 - 1).


// &str vs &'static str
// Lifetime:

// &str: The lifetime is flexible and determined by the context.
// &'static str: The lifetime is always the entire duration of the program.

// Use cases:

// &str is more common and flexible. It's used for string slices with various lifetimes.
// &'static str is typically used for string literals or constants defined in the program.

// Compilation:

// &'static str values are often stored in the compiled binary.
// &str can refer to dynamically created strings or slices of other strings.
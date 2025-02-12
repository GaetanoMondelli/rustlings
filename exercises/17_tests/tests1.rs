// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::is_even;
    // or import everything in the outer module
    // use super::*;

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(2));
        assert!(is_even(4));
        assert!(is_even(6));
        assert!(is_even(8));
        assert!(is_even(10));
    }
}

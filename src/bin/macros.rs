macro_rules! transform {
    () => {
        
    };
}

fn main() {
    transform!();
}

fn add(left:i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
/// Creates and runs test cases on the add method.
macro_rules! test_add {
    ($name:ident: $value:expr) => {
        #[test]
        fn $name() {
            let (left, right, equals) = $value;
            assert_eq!(add(left, right), equals);
        }
    };
}

// cargo test --bin macros
#[cfg(test)]
mod tests {
    use super::*;

    test_add!(add1: (1,2,3));
    test_add!(add_minus: (-1,2,1));
    test_add!(add_zero: (0,1,1));
    test_add!(add_large: (1000,1000,2000));

}
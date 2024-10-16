// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// I AM DONE

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let a: i32 = 1;
        let mut b: i32 = 3;
        b = a;
        assert_eq!(a, b);
    }
}

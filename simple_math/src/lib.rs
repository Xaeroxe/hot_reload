#[no_mangle]
pub extern fn double(input: i32) -> i32 {
    input * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(double(2), 4);
    }
}

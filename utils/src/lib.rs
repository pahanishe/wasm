#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn add_one() {
        assert_eq!(crate::add_one(1), 2);
    }

    #[test]
    fn multiply() {
        assert_eq!(crate::multiply(3,5), 15);
    }

}


#[no_mangle]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}

#[no_mangle]
pub extern fn multiply(x: u32, y: u32) -> u32 {
    x * y
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn foo(input_filename: &str) {
    println!("param: {:?}", input_filename);
}

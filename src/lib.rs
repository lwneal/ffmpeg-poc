#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn foo(input_filename:&str) {
    println!("The foo() function wants to run an ffmpeg command with param: {:?}", input_filename);
}

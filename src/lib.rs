use std::process::Command;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn foo(input_filename: &str, output_filename: &str) {
    //println!("Run the following ffmpeg -i {:?} -vc libx264 -pix_fmt yuvj420p {:?}", input_filename, output_filename);
    //let cmd = format!("ffmpeg -i {:?} -vc libx264 -pix_fmt yuvj420p {:?}", input_filename, output_filename);
    let cmd_output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_filename)
        .arg(output_filename)
        .output()
        .expect("FFmpeg call failed");
}

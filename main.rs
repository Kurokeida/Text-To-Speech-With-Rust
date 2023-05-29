use std::process::Command;

fn main() {
    let text = "Hello audio";
    let output = Command::new("espeak")
                     .arg("-w")
                     .arg("output.wav")
                     .arg(text)
                     .output()
                     .expect("failed to execute process");

    let output = Command::new("aplay")
                     .arg("output.wav")
                     .output()
                     .expect("failed to execute process");
}
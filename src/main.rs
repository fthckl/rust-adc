
use std::{fs::File, io::Read, thread, time::Duration};
fn main() {
    // Time in milliseconds
    let ms = 250;

    let pin = "0"; // sensor, A0, P9_39
    let iio_path = format!("/sys/bus/iio/devices/iio:device0/in_voltage{}_raw", pin);

    println!("Hit ^C to stop");

    // Loop to continuously read sensor data
    loop {
        // Open the file in read-only mode
        let mut file = match File::open(&iio_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
                return;
            }
        };

        // Read data from the file
        let mut data = String::new();
        if let Err(e) = file.read_to_string(&mut data) {
            eprintln!("Failed to read data: {}", e);
            return;
        }

        // Trim the newline character at the end of the data
        let data = data.trim_end();
        println!("data= {}", data);

        // Sleep for the specified time
        thread::sleep(Duration::from_millis(ms));
    }
}
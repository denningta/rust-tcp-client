use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn send_and_listen(address: &str) -> io::Result<()> {
    // Connect to the server at the specified address
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to the server at {}", address);

    // Create a clone of the stream for reading responses
    let mut reader = BufReader::new(stream.try_clone()?);

    // A separate thread to continuously read from the stream
    thread::spawn(move || {
        let mut buffer = String::new();

        loop {
            // Read each line from the stream (assuming server sends data line by line)
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    // Connection closed by the server
                    println!("Connection closed by the server.");
                    break;
                }
                Ok(_) => {
                    // Print the response from the server
                    println!("Received: {}", buffer.trim_end());
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }

            // Sleep briefly to prevent busy waiting
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Continuously send messages to the server
    loop {
        let mut message = String::new();

        // Read input from the user
        println!("Enter message to send (or 'exit' to quit):");
        io::stdin().read_line(&mut message)?;

        // Exit the loop if the user types "exit"
        if message.trim() == "exit" {
            break;
        }

        // Send the message to the server
        stream.write_all(message.as_bytes())?;
        stream.flush()?;
    }

    println!("Connection closed by client.");
    Ok(())
}

fn main() {
    let address = "192.168.1.240:65432"; // Example IP and port

    // Handle errors in communication
    if let Err(e) = send_and_listen(address) {
        eprintln!("Failed to communicate: {}", e);
    }
}

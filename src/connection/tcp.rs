use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    time::Duration,
};

pub fn message_loop(address: &str) -> io::Result<()> {
    loop {
        let mut message = String::new();

        // Read input from the user
        println!("Enter message to send (or 'exit' to quit):");
        io::stdin().read_line(&mut message)?;

        // Exit the loop if the user types "exit"
        if message.trim() == "exit" {
            println!("Exiting...");
            break;
        }

        // Send the message and listen for responses
        if let Err(e) = send_message(address, &message.trim()) {
            eprintln!("Error communicating with server: {}", e);
        }

        // Sleep briefly before asking for another message to avoid rapid retries
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn send_message(address: &str, message: &str) -> io::Result<()> {
    // Establish a new TCP connection
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to the server at {}", address);

    // Send the message to the server
    stream.write_all(message.as_bytes())?;
    stream.flush()?;

    // Create a BufReader for reading the responses
    let mut reader = BufReader::new(stream.try_clone()?);

    // Buffer to store incoming responses
    let mut buffer = String::new();

    // Listen for multiple responses
    loop {
        buffer.clear();

        // Read the response from the server
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                // Connection closed by the server
                println!("Connection closed by the server.");
                break;
            }
            Ok(_) => {
                // Print the received message
                println!("Received: {}", buffer.trim_end());
            }
            Err(e) => {
                // Handle any read errors
                eprintln!("Error reading from server: {}", e);
                break;
            }
        }

        // Sleep briefly to prevent busy waiting
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

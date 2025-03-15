# A Trait for Reading and writing Binary Data

## Example usage

### Cargo.toml

```toml
[dependencies]
binary = { git = "https://github.com/wioenena/binary.git" }
```

```rust
use binary::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut client = TcpStream::connect("127.0.0.1:8080").unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut string = String::new();
            loop {
                let char = stream.read_u8();
                match char {
                    Ok(c) => string.push(c as char),
                    Err(_) => break,
                }
            }

            println!("incoming message: {}", string);
            tx.lock().unwrap().send(()).unwrap();
        }
    });

    std::thread::spawn(move || {
        let message = "Hello, World!".as_bytes();
        for &byte in message {
            client.write_u8(byte).unwrap();
        }
    });

    rx.recv().unwrap();
}
```
### To accept a connection in TCP:

```rust
use std::net::{TcpListener, TcpStream};

// -- snip --

let listener = TcpListner::bind("127.0.0.1:443").unwrap();

loop {
	match listener.incoming() {
		Ok(stream) => handleConn(stream),
		Err(error) => panic!(error),
	};
}
```

### Reading the HTTP Request

```rust
use std::io::{prelude::*, BufReader};

fn handleConn(mut stream: TcpStream) {
	let buf_reader = 
		BufReader::new(&mut stream);

	let http_request: Vec<_> = buf_reader
		.lines()
		.map(|result| result.unwrap())
		.take_while(|line| !line.is_empty())
		.collect();

	println!("{:#?}", http_request);
}
```

The `BufReader` wraps a mutable reference to the `stream` and adds buffering by managing calls to the `std::io::Read` trait method.

Output:
```
[
    "GET /styles.css HTTP/1.1",
    "Host: localhost:8080",
    "Connection: keep-alive",
    "sec-ch-ua: \"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"114\", \"Google Chrome\";v=\"114\"",
    "DNT: 1",
    "sec-ch-ua-mobile: ?0",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "sec-ch-ua-platform: \"Windows\"",
    "Accept: text/css,*/*;q=0.1",
    "Sec-Fetch-Site: same-origin",
    "Sec-Fetch-Mode: no-cors",
    "Sec-Fetch-Dest: style",
    "Referer: http://localhost:8080/",
    "Accept-Encoding: gzip, deflate, br",
    "Accept-Language: en-US,en;q=0.9",
]
```

### Writing the HTTP Response

The HTTP response should be in the following format

```
HTTP/2.0 200 OK
Content-Encoding: gzip
Content-Type: text/html
Content-Length: 585

<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover">
        <meta name="description" content="A chat application">
        <title>Hello!</title>
        <link rel="stylesheet" href="styles.css" media="screen">
        <link defer rel="icon" type="image/x-icon" href="favicon.ico">
    </head>
    <body>
        <h1>hello!</h1>
        <p>Hi from Rust</p>
    </body>
    <footer>
        <script defer src="script.js"></script>
    </footer>
</html>
```

## Closing a TCP Stream

To close a TCP stream in Rust, you can call the `close` method on the `TcpStream` object. The `close` method shuts down the TCP connection and releases any associated resources. Here's an example:

```rust
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");

    // Perform operations on the stream...

    // Close the TCP stream
    stream.shutdown(std::net::Shutdown::Both).expect("Failed to close stream");
}
```

The `Shutdown::Both` parameter indicates that we want to shut down both the reading and writing halves of the TCP stream. Alternatively, you can use `Shutdown::Read` or `Shutdown::Write` if you only want to shut down the corresponding halves.


## Rust std::TCP v/s Tokio TCP

Rust's standard library (`std`) provides a basic set of networking capabilities, including TCP networking. On the other hand, Tokio is an asynchronous runtime and networking framework for Rust that builds on top of `std` and provides more advanced and efficient networking abstractions.

Here are some key differences between Rust's `std` TCP and Tokio TCP:

1. Asynchronous and Concurrency: Tokio is designed around asynchronous programming, which allows you to efficiently handle multiple connections concurrently. It utilizes non-blocking I/O operations and asynchronous task scheduling to achieve high scalability. Rust's `std` TCP, on the other hand, primarily offers blocking I/O operations, meaning that each connection will block the execution until data is sent or received.

2. Performance and Scalability: Tokio's design and architecture make it well-suited for building highly concurrent and scalable networked applications. It uses an event-driven, reactor-based model to efficiently handle large numbers of connections with a small number of threads. This enables Tokio applications to handle thousands of connections concurrently without incurring significant performance penalties. Rust's `std` TCP can also handle concurrent connections but may not provide the same level of performance and scalability as Tokio.

3. Async/Await Syntax: Tokio takes advantage of Rust's `async`/`await` syntax to write asynchronous code in a more concise and readable manner. It allows you to write asynchronous operations that can be awaited, making it easier to work with complex asynchronous workflows. Rust's `std` TCP uses traditional callback-based APIs or explicit threading to handle concurrency, which can be more verbose and error-prone compared to the `async`/`await` syntax.

4. Additional Utilities and Abstractions: Tokio provides a rich set of utilities and abstractions on top of `std` TCP. It offers features like timers, timeouts, task coordination primitives, and specialized I/O types. These additional tools simplify common networking tasks and enable advanced functionality in asynchronous applications. Rust's `std` TCP provides a more basic set of networking functionalities without the additional abstractions and utilities that Tokio offers.

In summary, Rust's `std` TCP provides a straightforward and blocking I/O approach to TCP networking, while Tokio TCP offers a powerful and efficient asynchronous networking framework with advanced abstractions, concurrency support, and performance optimizations. Tokio is particularly suitable for building high-performance and highly concurrent networked applications.

## Tokio TCP built on top of Rust std::TCP

Yes, Rust's Tokio TCP is built on top of the TCP networking capabilities provided by Rust's standard library (`std`). Tokio is designed as an asynchronous runtime and networking framework that extends and enhances the functionality provided by `std`. It leverages the underlying `std` TCP implementation and adds additional abstractions, utilities, and optimizations to provide a more powerful and efficient networking experience.

Tokio uses `std`'s TCP stream types (`TcpStream` and `TcpListener`) and extends them with Tokio-specific traits, such as `AsyncRead` and `AsyncWrite`, which enable asynchronous reading and writing operations. Tokio's runtime handles the scheduling and execution of asynchronous tasks, allowing you to write non-blocking, asynchronous code using `async`/`await` syntax.

By building on top of `std`, Tokio benefits from the underlying TCP implementation provided by Rust, ensuring compatibility and leveraging the performance optimizations and bug fixes implemented in `std` over time. However, Tokio introduces its own abstractions and optimizations to enhance the asynchronous networking capabilities and provide a more ergonomic and efficient development experience.

## Using Tokio

Adding `tokio` to the app

```toml
tokio = { version = "1", features = ["full"] }
```

Tokio requires an `async` main function to apply the `tokio!` macro

```rust
#[tokio::main]
async fn main() {
	// -- snip --
}
```

The macro converts the above function into the following

```rust
fn main() {
	let mut rt = tokio::runtime::Runtime::new().unwrap();
	rt.block_on(async {
		// -- snip --
	});
}
```

### Tokio TCP Handling

```rust
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

	loop {
		let (socket, _) = listener.accept().await.unwrap();
		process(socket).await;
	}
}

async fn process(socket: TcpStream) {
	// -- snip --
}
```


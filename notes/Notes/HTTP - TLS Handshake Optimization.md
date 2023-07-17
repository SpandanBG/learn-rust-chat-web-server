To reduce HTTP TLS (Transport Layer Security) handshaking time, you can consider the following strategies:

1. **Session resumption**: TLS supports session resumption, which allows a client and server to reuse a previously established session. This can significantly reduce handshaking time for subsequent connections. Ensure that your client and server implementations support session resumption. In Rust, you can use libraries like `rustls` or `native-tls` that provide session resumption mechanisms.

2. **Connection pooling**: Instead of establishing a new TLS connection for every request, consider using connection pooling. Connection pooling allows you to reuse existing connections for multiple requests, eliminating the overhead of TLS handshakes. Libraries like `hyper` in Rust provide connection pooling capabilities.

3. **TLS session tickets**: TLS session tickets are another way to optimize TLS handshakes. They allow the server to store session-specific encryption data, reducing the need for full handshakes. Both the client and server must support session tickets for this optimization to work. Libraries like `rustls` support session ticket functionality.

4. **Early data (0-RTT)**: If your TLS implementation supports it, consider utilizing 0-RTT (Zero Round-Trip Time) or early data feature. This allows the client to send encrypted data in the initial handshake, reducing the overall round-trip time. However, be cautious with 0-RTT as it can introduce security risks if not implemented correctly.

5. **TLS termination at load balancer**: If you're using a load balancer or reverse proxy, consider terminating TLS at the load balancer rather than at each individual server. This allows the load balancer to handle the TLS handshake, offloading the overhead from backend servers and potentially reducing overall latency.

6. **Optimize server-side TLS configuration**: Ensure that your server's TLS configuration is optimized for performance. This includes using appropriate cipher suites, protocols, and settings that balance security and performance. Keep up-to-date with best practices and recommendations for TLS configuration.

Remember that TLS handshaking time is influenced by various factors, including network latency, server load, and client capabilities. It's essential to measure and profile your application's performance to identify specific areas for improvement.
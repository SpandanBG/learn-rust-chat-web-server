# Lighthouse 
- `npm i -g lighthouse`
- `lighthouse https://localhost:8080 --view --output-file=localhost.report.html`

# Cargo Watch
- `cargo install cargo-watch`
- `cargo watch -s src -x run`

# Hyperfine (App benchmarking tool)
[Install form here](https://github.com/sharkdp/hyperfine)

# Load Testing with K6
- `choco install k6`
- `k6 run -vus 10 loadtest.js`

Threadpool vs Tokio
![Threadpool vs Tokio](https://github.com/SpandanBG/learning-rust-web-server/blob/master/tokio_vs_threadpool_load_test.png?raw=true)
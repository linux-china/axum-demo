Axum demo
============

Axum: an easy to use, yet powerful, web framework designed to take full advantage of the Tokio ecosystem

# Features

* static assets handle
* index page
* json output: struct and json!() macro
* Path variable `/user/:id`

# Docker support
Refer from  [Packaging a Rust web service using Docker](https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/) 

```bash
docker build -t axum-demo .
docker run -p 3000:3000 axum-demo
```

# References

* Axum home: https://github.com/tokio-rs/axum
* Axum examples: https://github.com/tokio-rs/axum/tree/main/examples
* Announcing Axum: https://tokio.rs/blog/2021-07-announcing-axum

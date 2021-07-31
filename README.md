Axum demo
============

Axum: ergonomic and modular web framework built with Tokio, Tower, and Hyper.

* Route requests to handlers with a macro free API.
* Declaratively parse requests using extractors.
* Simple and predictable error handling model.
* Generate responses with minimal boilerplate.
* Take full advantage of the tower and tower-http ecosystem of middleware, services, and utilities.

Hyper: A fast and correct HTTP implementation for Rust.

* HTTP/1 and HTTP/2
* Asynchronous design
* Leading in performance
* Tested and correct
* Extensive production use
* Client and Server APIs

tower-http: a collection of HTTP specific middleware and utilities built with Tower's Service trait.

* Trace: Easily add high level tracing/logging to your application. Supports determining success or failure via status codes as well as gRPC specific headers. Has great defaults but also supports deep customization.
* Compression and Decompression: Automatically compress or decompress response bodies. This goes really well with serving static files using ServeDir.
* FollowRedirect: Automatically follow redirection responses.

http: a general purpose library of common HTTP types, for examples `http::{Request, Response, StatusCode}` 

# Vocabulary

* Handler: an async function that accepts zero or more “extractors” as arguments and returns something that can be converted into a response.
* Routing: between handlers and request paths
* Extractor: a type that implements FromRequest. Extractors are how you pick apart the incoming request to get the parts your handler needs.
* Building responses: anything that implements IntoResponse can be returned from a handler
* Middleware: used to decorate the application, providing additional functionality

# Demo cases

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

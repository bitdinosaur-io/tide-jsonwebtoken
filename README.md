# Tide (http-rs/tide) JWT Authorization Middleware

This Rust library offers a middleware for the [Tide](https://github.com/http-rs/tide) web server framework, focusing on API key authentication via JWT (JSON Web Tokens) using the `tide_jsonwebtoken` crate.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/tide-jsonwebtoken.svg)](https://crates.io/crates/tide-jsonwebtoken)
[![Documentation](https://docs.rs/wall-echain/badge.svg)](https://docs.rs/tide-jsonwebtoken)


## Features

- Seamless API key validation using JWT.
- Direct integration with Tide routes.
- Efficient handling of protected and unprotected routes.

## Quick Start

Add the required dependencies to your `Cargo.toml`:

```toml
[dependencies]
tide = "0.16" # Use the latest version
tide-jsonwebtoken = "0.1.0" # Use the latest version
async-std = { version = "1.12.0", features = ["attributes"] } # Use the latest version
```

## Usage

1. **Initialize the Middleware**:

   First, create an instance of the `ApiKeyMiddleware`:

   ```rust
   let jwt = ApiKeyMiddleware::new("your-secret-key");
   ```

2. **Set Up Tide Application**:

   Initialize the Tide application and apply the middleware to specific routes:

   ```rust
   let mut app = tide::new();
   app.at("/login").get(|_| async { Ok("Login route!") });
   app.at("/name/:id")
      .with(jwt)
      .get(|req: Request<()>| async move {
           Ok(format!("Hello, {}!", req.param("id").unwrap_or("0")))
       });
   ```

3. **Run the Server**:

   ```rust
   app.listen("127.0.0.1:8080").await?;
   ```

## Error Handling

The middleware inspects the `x-api-key` header in incoming requests. If the JWT is validated, the request continues; otherwise, the middleware returns a `401 Unauthorized` status. Potential error messages include:

- `API key missing`: The request lacks the `x-api-key` header.
- `Invalid API key`: The supplied API key (JWT) is not valid.

## Contributing

Pull requests are encouraged. For major adjustments, kindly initiate an issue first to deliberate on the desired changes.

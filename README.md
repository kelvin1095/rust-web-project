# Rust Web Server

This project is to build a web server wtih rust.

The files in the public folder form a basic website built with sveltekit.

Things to accompilsh:

- Serve static files in the public directory. These files are generated from SvelteKit.
  - SvelteKit also does routing and error pages so thats less I need to worry about in Rust.
- Graceful shutdown.
- Log requests, reponse, bytes etc.
- Make requests to a postgres database.
  - I still need to have an error response to invalid get request for get pokemon by pokedex number
- User Authentication.
- Utilise multithreading?

https://docs.rs/tower-http/latest/tower_http/trace/
https://docs.rs/axum/latest/axum/extract/struct.OriginalUri.html#
https://docs.rs/axum/latest/axum/extract/index.html

I think what would be very fun is to create a web app that helps learn language. Some kind of mix and match: One word in one language matches to the same word in another language.

# Rust Web Server

This project is to build a web server wtih rust.

The files in the public folder form a basic website built with sveltekit.

To Do:

- Refactor code nicely (Never ending).
- Proper Error handling.
- User Authentication.

Done:

- Serve static files in the public directory. These files are generated from SvelteKit.
  - SvelteKit also does routing and error pages so thats less I need to worry about in Rust.
- Graceful shutdown.
- Log requests, reponse, bytes etc.
- Make requests to a postgres database.

https://docs.rs/tower-http/latest/tower_http/trace/
https://docs.rs/axum/latest/axum/extract/struct.OriginalUri.html#
https://docs.rs/axum/latest/axum/extract/index.html

I think what would be very fun is to create a web app that helps learn language. Some kind of mix and match: One word in one language matches to the same word in another language.

<!--
DROP TABLE pokemon;

CREATE TABLE pokemon (
  PokedexNumber INT,
  Name VARCHAR(255),
  Form VARCHAR(255),
  Type1 VARCHAR(255),
  Type2 VARCHAR(255),
  Ability1 VARCHAR(255),
  Ability2 VARCHAR(255),
  HiddenAbility VARCHAR(255),
  HP INT,
  Att INT,
  Def INT,
  SpA INT,
  SpD INT,
  Spe INT,
  Height REAL,
  Weight REAL,
  PokemonImageFilename VARCHAR(255)
  );

\COPY pokemon FROM 'PokemonStats.csv' WITH (FORMAT csv, HEADER true);
-->

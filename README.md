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

# Important commands

npm --prefix svelte-front run build
cargo run --manifest-path rust-back/Cargo.toml

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

<!-- CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    hashed_password VARCHAR(255) NOT NULL,
    date_registered TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    account_status VARCHAR(20) DEFAULT 'active',
    preferred_name VARCHAR(50)
); -->

<!-- CREATE TABLE user_salts (
    user_id SERIAL PRIMARY KEY,
    salt_value VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
); -->

<!-- CREATE TABLE user_peppers (
    user_id SERIAL PRIMARY KEY,
    pepper_value VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
); -->

-- Inserting a new user
INSERT INTO users (username, email, hashed_password, hashing_algorithm, preferred_name)
VALUES ('example_user', 'user@example.com', 'hashed_password_value', 'bcrypt', 'John');

-- Retrieving the user_id for the newly inserted user
SELECT user_id, hashed_password FROM users WHERE username = 'example_user';

-- Inserting the user's salt into the user_salts table
INSERT INTO user_salts (user_id, salt_value) VALUES (1, 'unique_salt_value');

curl -X POST -i localhost:3000/api/register -H 'Content-Type: application/json' -d '{"username": "admin", "name": "admin", "email": "example@email.com", "password":"password"}'

Accessing postgres from the terminal:

sudo su postgres
psql
psql -f pokemon.sql

psql -f pokemon.sql -v pokemonIndex=5
psql -f pokemonByType.sql -v attack=100 -v spattack=100
psql -f pokemonByType.sql -v type1=Fire -v type2=Flying

note: for string type columns, surround the variable name in ''

curl -X POST -i \
 localhost:3000/api/register \
 -H 'Content-Type: application/json' \
 -d '{"username": "admin", "name": "admin", "email": "example@email.com", "password":"password"}'

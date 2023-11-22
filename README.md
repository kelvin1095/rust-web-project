# Learning project

This project is to build a webapp using svelte as the frontend and rust as the backend.

## To Do (backend):

- Split code (Never ending).
- Proper Error handling (Never ending).
- User Authentication.
  - Need to modify what happens after user has logged in.
  - Need to do the secret key for JWT better

## To Do (frontend):

- Create webpage for learning language, current aim is to make something for Japanese, Korean and Mandarin.
- Design stuff (Never ending).
  - Mobile layout
  - Dark Mode

## Potentially useful links

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

### Creating tables for user log in information

`CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    hashed_password VARCHAR(255) NOT NULL,
    date_registered TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    account_status VARCHAR(20) DEFAULT 'active',
    preferred_name VARCHAR(50)
);`

`CREATE TABLE user_salts (
    user_id SERIAL PRIMARY KEY,
    salt_value VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);`

`CREATE TABLE user_peppers (
    user_id SERIAL PRIMARY KEY,
    pepper_value VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);`

Resetting table:

`DROP TABLE users;
DROP TABLE user_peppers;`

### Accessing postgres from the terminal:

`sudo su postgres
psql

psql -f pokemon.sql
psql -f pokemon.sql -v pokemonIndex=5`

note: for string type columns, surround the variable name in ''

Test logging in:

`curl -X POST -i \
 localhost:3000/api/register \
 -H 'Content-Type: application/json' \
 -d '{"username": "admin", "name": "admin", "email": "example@email.com", "password":"password"}'
`

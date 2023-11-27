# Learning project

This project is to build a webapp using svelte as the frontend and rust as the backend. My goal is to build a basic language learning app.

For my frontend, I'm using svelte with typescript.
For my backend, I'm using rust.

## To Do (backend):

- Refactor/organise code (Never ending).
- Proper Error handling (Never ending).
- User Authentication.
  - Should change registration process from account_status active by default to unverified and send out email to change to active.
  - The JWT secret key should be randomised.
  - The table storing user peppers is likely better off in a key value database.
  - Need to deal with the case where if JWT is expired, what does the frontend do.
- Need to figure out a way to deliver the actual language part to the user.
  - I want to be able to capture relationship between presented word and submitted choice too.
  - Also need to figure out how to track progress per user.

## To Do (frontend):

- Better Design (Never ending).
- Types of quizzes:
  - A constant stream of mix and match.
  - Typical quizz with 4 choices.
  - A sentence building exercise.

## To Do (security):

- Input validation, need to implement validation to remove spaces
- Set JWT to httpOnly and map the cookie to the authorization header instead of setting it at the frontend.

## Potential future plans

- Use openai in some capacity to be able to mark more open ended assignments (such as introduce yourself or what did you do on the weekend).
- Unique avatars
- Being able to comment on verified sentences/words (probably would need some moderating system)
- Translating competitions

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

### Accessing postgres from the terminal:

`sudo su postgres
psql

psql -f pokemon.sql
psql -f pokemon.sql -v pokemonIndex=5`

note: for string type columns, surround the variable name in

Test logging in:

`curl -X POST -i \
 localhost:3000/api/register \
 -H 'Content-Type: application/json' \
 -d '{"username": "admin", "name": "admin", "email": "example@email.com", "password":"password"}'
`

### Creating tables for user log in information

I think there should be another table for things you can change frequently such as:

- username
- email
- preferred name

`CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    hashed_password VARCHAR(255) NOT NULL,
    date_registered TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_changed_password TIMESTAMP,
    account_status VARCHAR(20) DEFAULT 'active',
);`

Since I'm storing the salt in the hashed_password column as phc format, the salts table is redundant.

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

# TO DO:

- Make tracing the base router, move the router to a different function/file

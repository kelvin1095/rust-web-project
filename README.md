# Learning project

This project is to build a webapp using svelte as the frontend and rust as the backend. My goal is to build a basic language learning app.

For my frontend, I'm using svelte with typescript.
For my backend, I'm using rust.

## Quiz Question Types

- Show an english word with possible answers in a different language
- Show a different language word with possible answers in english
- Mix and match, 5 english words and 5 foreign language words
- Show a sentance in english and get the user to put words together
- Show a sentance in foreign language and get the user to put words together

## To Do

### Backend:

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
- I want to be implement OAuth as well so people can sign in via google, microsfot, facebook or apple.

### Frontend:

- Better Design (Never ending).
- Types of quizzes:
  - A constant stream of mix and match.
  - Typical quizz with 4 choices.
  - A sentence building exercise.

### Database:

- Probably need to have more roles.
- Still thinking of how to set user progress tracking.
- I want to use enums

### Security:

- Input validation, need to implement validation to remove spaces.
- Set JWT to httpOnly and map the cookie to the authorization header instead of setting it at the frontend.

### Infrastructure:

- Looking for a place to host my app. Currently considering AWS

## Potential future plans

- Use openai in some capacity to be able to mark more open ended assignments (such as introduce yourself or what did you do on the weekend).
- Unique avatars
- Being able to comment on verified sentences/words (probably would need some moderating system)
- Translating competitions

# Important commands

Build front and backend.

```
npm --prefix svelte-front run build
cargo run --manifest-path rust-back/Cargo.toml
```

<!--
DROP TABLE pokemon;

CREATE TYPE pokemon_type AS ENUM ('Normal', 'Fire', 'Water', 'Electric', 'Grass', 'Ice', 'Fighting', 'Poison', 'Ground', 'Flying',  'Psychic', 'Bug', 'Rock', 'Ghost', 'Dragon', 'Dark', 'Steel', 'Fairy');

view enums in the pg_enum table.

CREATE TABLE pokemon (
  Pokedex_Number INT NOT NULL,
  Name VARCHAR(50) NOT NULL,
  Form VARCHAR(50),
  Type_1 pokemon_type NOT NULL,
  Type_2 pokemon_type,
  Ability_1 VARCHAR(50) NOT NULL,
  Ability_2 VARCHAR(50),
  Hidden_Ability VARCHAR(50),
  HP INT NOT NULL,
  Att INT NOT NULL,
  Def INT NOT NULL,
  SpA INT NOT NULL,
  SpD INT NOT NULL,
  Spe INT NOT NULL,
  Height REAL,
  Weight REAL,
  Pokemon_Image VARCHAR(50) NOT NULL
  );

\COPY pokemon FROM 'PokemonStats.csv' WITH (FORMAT csv, HEADER true);
-->

Accessing postgres from the terminal:

```bash
sudo su postgres
psql

psql -f pokemon.sql
psql -f pokemon.sql -v pokemonIndex=5`
```

Test logging in:

```bash
curl -X POST -i \
localhost:3000/api/register \
-H 'Content-Type: application/json' \
-d '{"username": "admin", "name": "admin", "email": "example@email.com", "password":"password"}'

curl -X POST -i \
localhost:3000/api/product \
-H 'Content-Type: application/json' \
-d '{"num1": "22", "num2": "33"}'

curl -X POST -i \
localhost:3000/api/japanese/wordlist \
-H 'Content-Type: application/json' \
-d '{"category": "Location"}'

curl -X GET -i \
localhost:3000/api/japanese/sentence
```

### Creating tables for user log in information

I think there should be another table for things you can change frequently such as:

- username
- email
- preferred name
- profile picture?

I'm storing the salt in the hashed_password column as phc format. The peppers should be stored in a different database

```SQL
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    hashed_password VARCHAR(100) NOT NULL,
    date_registered TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_changed_password TIMESTAMP,
    account_status VARCHAR(20) DEFAULT 'active'
);

CREATE TABLE user_peppers (
    user_id SERIAL PRIMARY KEY,
    pepper_value VARCHAR(20) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);

DROP TABLE users;
DROP TABLE user_peppers;
```

### Creating a vocab list table

```SQL
CREATE TABLE vocablist (
    English varchar(50) NOT NULL,
    Japanese varchar(50) NOT NULL,
    Japanese_Romanized varchar(50) NOT NULL,
    Korean varchar(50) NOT NULL,
    Korean_Romanized varchar(50) NOT NULL,
    Mandarin varchar(50) NOT NULL,
    Mandarin_Romanized varchar(50) NOT NULL,
    Category varchar(50) NOT NULL
);

\COPY vocablist FROM 'vocablist.csv' DELIMITER ',' CSV HEADER;

CREATE TABLE language_data (
    id SERIAL PRIMARY KEY,
    english_text TEXT NOT NULL,
    japanese_text TEXT NOT NULL,
    japanese_broken_down JSONB NOT NULL
);

DROP TABLE vocablist;
```

# CURRENT TO DO:

Figure out a better way to retrieve the word list. Must consider if its better to create seperate routes for each language or try create a general route for all.

```SQL
SELECT english,
    CASE
        WHEN $1 = 'japanese' THEN japanese
        WHEN $1 = 'korean' THEN korean
        WHEN $1 = 'mandarin' THEN mandarin
    END AS foreign
FROM vocablist WHERE category = $2;
```

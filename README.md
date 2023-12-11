# Learning project

This project is to build a webapp. My goal is to learn by building a language learning app. The current focus is on Japanese, Korean and Mandarin.

I am using Svelte with typescript to build out the webpage. My backend is written using rust. I'm using postgresql as my database.

I will need to create a few more tables. Firstly will be a table to keep track of user progress.

## Quiz Question Types

-   Show an english word with possible answers in a different language.
-   Show a different language word with possible answers in english.
-   Mix and match, 5 english words and 5 foreign language words.
-   Show a sentence in english and get the user to put words together.
-   Show a sentence in foreign language and get the user to put words together.
-   Show a sentence and ask the correct form of the word in that situation (probably more specific to japanese and korean).
-   Having stories would also be excellent.

## To Do

### Backend:

-   Refactor/organise code (Never ending).
-   Better Error handling (Never ending).
-   User Registration/Authentication:
    -   Need to deal with the case where if JWT is expired, what does the frontend do.
    -   The table storing user peppers should be in a different database.
    -   Change registration process from account_status active by default to unverified and send out email to change to active.
    -   Page should redirect with a success message.
    -   Reason for registration failing would be good: username already registered, email already registered or password does not match. Should be able to grab from response message or client side validation.
-   Need to figure out a way to deliver content to the user.
    -   I have a test example of a mixed vector being sent from the back to the front. Will need to figure out how to generate quizzes in the backend. Refer to the above on how to pass this information to the user.
    -   I want to be able to capture relationship between presented word and submitted choice too. Definitely a want for words, but still deciding on sentances.
    -   Also need to figure out a way to store data that tracks user progress.
-   I want to be implement OAuth as well so people can sign in via google, microsoft, facebook or apple.

### Frontend:

-   Better Design (Never ending).
-   Displaying the quiz after receiving data from the backend:
    -   A game of mix and match. 5 seems like a good number.
    -   Multiple choice with 4 options.
    -   Build sentences using provided blocks.

### Database:

-   Still thinking of how to set user progress tracking.
-   Test to see if using enums will make performance much better.

### Security:

-   Input validation, need to implement validation to remove spaces.

### Infrastructure:

-   Looking for a place to host my app. Currently considering AWS but AWS is expensive.

## Potential future plans

-   Would be nice to have each foreign word be associated with an image as well as audio of how to pronounce the word.
-   Use openai in some capacity to be able to mark more open ended assignments (such as introduce yourself or what did you do on the weekend). Capturing this data to use to train home made model would be nice.
-   Being able to comment on verified sentences/words (probably would need some moderating system).
-   Translation competitions.

# Important commands

Build front and backend.

```
npm --prefix svelte-front run build
cargo run --manifest-path rust-back/Cargo.toml
```

<!--
```SQL
DROP TABLE pokemon;

CREATE TYPE pokemon_type AS ENUM ('Normal', 'Fire', 'Water', 'Electric', 'Grass', 'Ice', 'Fighting', 'Poison', 'Ground', 'Flying', 'Psychic', 'Bug', 'Rock', 'Ghost', 'Dragon', 'Dark', 'Steel', 'Fairy');

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
```
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
localhost:3000/api/sum \
-H 'Content-Type: application/json' \
-d '{"num1": "22", "num2": "33"}'

curl -X POST -i \
localhost:3000/api/sum \
-H 'Content-Type: application/json' \
-H 'Cookie: auth-token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIiwibmFtZSI6ImFkbWluIiwiaWF0IjoxNzAyMDY3OTM0LCJleHAiOjIwMDAwMDAwMDB9.MGNATq2XT5AZXWDvQMdSfaKAYtMMQhVfTTyIdFyBAmE' \
-d '{"num1": "22", "num2": "33"}'

curl -X POST -i \
localhost:3000/api/japanese/word \
-H 'Content-Type: application/json' \
-d '{"category": "nature"}'

curl -X POST -i \
localhost:3000/api/korean/word \
-H 'Content-Type: application/json' \
-d '{"category": "nature"}'

curl -X POST -i \
localhost:3000/api/mandarin/word \
-H 'Content-Type: application/json' \
-d '{"category": "nature"}'

curl -X GET -i \
localhost:3000/api/japanese/sentence
```

### Creating tables for user log in information

I think there should be another table for things you can change frequently such as:

-   username
-   email
-   preferred name
-   profile picture?

I'm storing the salt in the hashed_password column as phc format. The peppers should be stored in a different database.

It may also be a good idea to create a table that keep tracks of when a jwt was created and which user.

```SQL
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL CHECK (LENGTH(username) > 0 AND LENGTH(username) < 20),
    email VARCHAR(30) UNIQUE NOT NULL CHECK (LENGTH(email) > 0 AND LENGTH(email) < 30 AND POSITION('@' IN email) > 0),
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

DROP TABLE user_peppers;
DROP TABLE users;
```

### Creating a vocab list table

I'm not sure about the category field. It may need to be changed to allow multiple categories in the future. I'm also thinking there might need to be another column to correspond with where user is up to in their learning path.

```SQL
CREATE TABLE word_data (
    id SERIAL PRIMARY KEY,
    english VARCHAR(20) NOT NULL,
    translated VARCHAR(20) NOT NULL,
    romanized VARCHAR(20) NOT NULL,
    language VARCHAR(20) NOT NULL,
    category VARCHAR(20) NOT NULL
);

\COPY word_data(english, translated, romanized, language, category) FROM 'word_list.csv' DELIMITER ',' CSV HEADER;

CREATE TABLE sentence_data (
    id SERIAL PRIMARY KEY,
    english_text TEXT NOT NULL,
    translated_text TEXT NOT NULL,
    broken_down JSONB NOT NULL,
    language VARCHAR(20) NOT NULL
);

\COPY sentence_data(english_text, translated_text, broken_down, language) FROM 'help.csv' DELIMITER ',' CSV;

DROP TABLE word_data;
DROP TABLE sentence_data;

-- Potentially useful for query something inside a JSONB column:
SELECT * FROM sentence_data WHERE broken_down @> '"は"';
```

## Current to do

Change the responses to be of some form like

```json
{
    "status": "success",
    "message": "success",
    "data": {}
}
```

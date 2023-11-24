WITH
    inserted_user AS (
        INSERT INTO
            users (username, email, hashed_password)
        VALUES
            ($1, $2, $3)
        RETURNING
            user_id
    )
INSERT INTO
    user_peppers (user_id, pepper_value)
SELECT
    user_id,
    $4
FROM
    inserted_user;

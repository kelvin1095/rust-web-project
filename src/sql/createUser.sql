WITH
    inserted_user AS (
        INSERT INTO
            users (username, email, hashed_password, preferred_name)
        VALUES
            ($1, $2, $3, $4)
        RETURNING
            user_id
    )
INSERT INTO
    user_peppers (user_id, pepper_value)
SELECT
    user_id,
    $5
FROM
    inserted_user;

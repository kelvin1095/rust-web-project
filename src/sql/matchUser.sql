SELECT
    users.user_id,
    users.preferred_name,
    user_peppers.pepper_value,
    users.hashed_password
FROM
    user_peppers
    LEFT JOIN users ON users.user_id = user_peppers.user_id
WHERE
    username = $1;
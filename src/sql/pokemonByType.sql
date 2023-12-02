SELECT
    pokedex_number,
    name,
    form
FROM
    pokemon
WHERE
    type_1 = $1
    AND (
        CASE
            WHEN $2 = '' THEN type_2 IS NULL
            ELSE type_2 = $2
        END
    );
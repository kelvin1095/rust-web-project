SELECT
    pokedexnumber,
    name,
    form,
    type1,
    type2
FROM
    pokemon
WHERE
    type1 LIKE $1
    AND (
        CASE
            WHEN $2 = '' THEN type2 IS NULL
            ELSE type2 LIKE $2
        END
    );
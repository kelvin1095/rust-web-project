SELECT
    pokedex_number,
    name,
    form,
    category,
    type_1,
    type_2,
    ability_1,
    ability_2,
    hidden_ability,
    hp,
    att,
    def,
    spa,
    spd,
    spe,
    height,
    weight,
    pokemon_image
FROM
    pokemon
WHERE
    pokedex_number = $1;
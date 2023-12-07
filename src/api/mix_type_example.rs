use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;

#[derive(Serialize)]
struct WordType {
    word: String,
    translation: String,
}

#[derive(Serialize)]
struct SentenceType {
    sentence: String,
    translation: String,
}

#[derive(Serialize)]
enum MixedType {
    WordType(WordType),
    SentenceType(SentenceType),
}

pub async fn mixed_vector() -> String {
    let example_1 = WordType {
        word: "Cat".to_string(),
        translation: "猫".to_string(),
    };
    let example_2 = SentenceType {
        sentence: "It is a Cat".to_string(),
        translation: "猫です".to_string(),
    };
    let example_3 = WordType {
        word: "Dog".to_string(),
        translation: "犬".to_string(),
    };

    let mut vector = vec![
        MixedType::WordType(example_1),
        MixedType::SentenceType(example_2),
        MixedType::WordType(example_3),
    ];

    let mut rng = thread_rng();
    vector.shuffle(&mut rng);

    let vector_json = serde_json::to_string_pretty(&vector);

    return vector_json.unwrap();
}

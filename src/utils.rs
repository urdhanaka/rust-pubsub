use rand::seq::IteratorRandom;

pub fn random_string(length: i32) -> String {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let mut result = String::with_capacity(length as usize);

    for _ in 0..length {
        let mut rng = rand::rng();
        let copied_char = chars.chars().choose(&mut rng).unwrap();

        result.push(copied_char);
    }

    result
}

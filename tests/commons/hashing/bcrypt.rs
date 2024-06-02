use rstest::rstest;

use commons::hashing::bcrypt::{encode, validate};

#[rstest]
#[case("test", "$2y$12$tA5nRGE/vNU/XzyAZ0op..Ukxfx5LIh7S/DBKL5q8BhD4raQbToLS")]
#[case(
    "password",
    "$2y$12$iY3Oi.uVqSIKNCd9G1FZCezhfLGdREBaHE2q9YxbonI951zYHHhYa"
)]
#[case(
    "some_test_string",
    "$2y$12$cYwFc6dr67oBPH.MU6iqpO/d42vVPZg3aJjZcw1fZmsMdPMhXn6hu"
)]
fn sha512_test(#[case] input: &str, #[case] output: &str) {
    let validate_known = validate(input, output);
    assert!(validate_known.is_ok());
    assert!(validate_known.unwrap());
    let encoded = encode(input);
    assert!(encoded.is_ok());
    let validate_encoded = validate(input, &encoded.unwrap());
    assert!(validate_encoded.is_ok());
    assert!(validate_encoded.unwrap());
}

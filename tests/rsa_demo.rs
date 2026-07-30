use rust_crypto::rsa_demo::{RsaExample, mod_pow};

#[test]
fn modular_exponentiation_matches_a_small_known_result() {
    assert_eq!(mod_pow(4, 13, 497), 445);
}

#[test]
fn small_rsa_example_round_trips_a_message() {
    let rsa = RsaExample::textbook();
    let ciphertext = rsa.encrypt(65);

    assert_eq!(ciphertext, 2790);
    assert_eq!(rsa.decrypt(ciphertext), 65);
}

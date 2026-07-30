use rust_crypto::sha256::sha256;

fn hex(bytes: [u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn hashes_the_empty_message() {
    assert_eq!(
        hex(sha256(b"")),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn hashes_a_standard_short_vector() {
    assert_eq!(
        hex(sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn hashes_messages_that_cross_a_block_boundary() {
    assert_eq!(
        hex(sha256(&[b'a'; 100])),
        "2816597888e4a0d3a36b82b83316ab32680eb8f00f8cd3b904d681246d285a0e"
    );
}

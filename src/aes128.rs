//! AES-128 de un bloque para estudiar su estado, rondas y expansión de clave.
//!
//! No es una API de cifrado de producción: no proporciona AEAD, nonces,
//! gestión de claves ni defensas contra canales laterales.

/// Cifra un bloque AES-128 con una clave de 16 bytes.
pub fn encrypt_block(key: [u8; 16], plaintext: [u8; 16]) -> [u8; 16] {
    let round_keys = expand_key(key);
    let mut state = plaintext;
    add_round_key(&mut state, &round_keys[0..16]);

    for round in 1..10 {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &round_keys[round * 16..round * 16 + 16]);
    }

    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &round_keys[160..176]);
    state
}

fn expand_key(key: [u8; 16]) -> [u8; 176] {
    let mut expanded = [0_u8; 176];
    expanded[..16].copy_from_slice(&key);
    let mut bytes = 16;
    let mut rcon = 1_u8;

    while bytes < expanded.len() {
        let mut word = [
            expanded[bytes - 4],
            expanded[bytes - 3],
            expanded[bytes - 2],
            expanded[bytes - 1],
        ];
        if bytes % 16 == 0 {
            word.rotate_left(1);
            for byte in &mut word {
                *byte = s_box(*byte);
            }
            word[0] ^= rcon;
            rcon = xtime(rcon);
        }
        for value in word {
            expanded[bytes] = expanded[bytes - 16] ^ value;
            bytes += 1;
        }
    }
    expanded
}

fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state {
        *byte = s_box(*byte);
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let original = *state;
    for row in 0..4 {
        for column in 0..4 {
            state[row + 4 * column] = original[row + 4 * ((column + row) % 4)];
        }
    }
}

fn mix_columns(state: &mut [u8; 16]) {
    for column in 0..4 {
        let offset = column * 4;
        let [a, b, c, d] = [
            state[offset],
            state[offset + 1],
            state[offset + 2],
            state[offset + 3],
        ];
        state[offset] = gf_mul(a, 2) ^ gf_mul(b, 3) ^ c ^ d;
        state[offset + 1] = a ^ gf_mul(b, 2) ^ gf_mul(c, 3) ^ d;
        state[offset + 2] = a ^ b ^ gf_mul(c, 2) ^ gf_mul(d, 3);
        state[offset + 3] = gf_mul(a, 3) ^ b ^ c ^ gf_mul(d, 2);
    }
}

fn add_round_key(state: &mut [u8; 16], key: &[u8]) {
    for (byte, key_byte) in state.iter_mut().zip(key) {
        *byte ^= key_byte;
    }
}

fn s_box(value: u8) -> u8 {
    let inverse = if value == 0 { 0 } else { gf_pow(value, 254) };
    inverse
        ^ inverse.rotate_left(1)
        ^ inverse.rotate_left(2)
        ^ inverse.rotate_left(3)
        ^ inverse.rotate_left(4)
        ^ 0x63
}

fn gf_pow(mut base: u8, mut exponent: u8) -> u8 {
    let mut result = 1_u8;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = gf_mul(result, base);
        }
        base = gf_mul(base, base);
        exponent >>= 1;
    }
    result
}

fn xtime(value: u8) -> u8 {
    if value & 0x80 == 0 {
        value << 1
    } else {
        (value << 1) ^ 0x1b
    }
}

fn gf_mul(mut left: u8, mut right: u8) -> u8 {
    let mut product = 0_u8;
    while right > 0 {
        if right & 1 == 1 {
            product ^= left;
        }
        left = xtime(left);
        right >>= 1;
    }
    product
}

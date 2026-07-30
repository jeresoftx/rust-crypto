//! Aritmética RSA de enteros pequeños para comprender la operación modular.
//!
//! Esta representación no usa claves grandes, padding ni generación segura de
//! claves; es insegura y solo existe como material de aprendizaje.

/// Calcula `base^exponent mod modulus` con multiplicación modular segura para `u64`.
pub fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    assert!(modulus > 0, "el módulo debe ser positivo");
    let mut result = 1_u64 % modulus;
    base %= modulus;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

/// Claves de un ejemplo clásico de RSA con `p = 61` y `q = 53`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RsaExample {
    modulus: u64,
    public_exponent: u64,
    private_exponent: u64,
}

impl RsaExample {
    /// Construye un ejemplo didáctico factorizable y sin padding.
    pub const fn textbook() -> Self {
        Self {
            modulus: 3233,
            public_exponent: 17,
            private_exponent: 2753,
        }
    }

    /// Cifra un entero menor que el módulo del ejemplo.
    pub fn encrypt(self, message: u64) -> u64 {
        assert!(
            message < self.modulus,
            "el mensaje debe ser menor que el módulo"
        );
        mod_pow(message, self.public_exponent, self.modulus)
    }

    /// Descifra un entero del espacio del ejemplo.
    pub fn decrypt(self, ciphertext: u64) -> u64 {
        assert!(
            ciphertext < self.modulus,
            "el ciphertext debe ser menor que el módulo"
        );
        mod_pow(ciphertext, self.private_exponent, self.modulus)
    }
}

fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 * right as u128) % modulus as u128) as u64
}

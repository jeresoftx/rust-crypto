//! Árbol de Merkle educativo construido sobre el SHA-256 del crate.
//!
//! La raíz solo compromete el orden y contenido de hojas entregados; no aporta
//! autenticidad sin una fuente confiable para distribuirla.

use crate::sha256::sha256;

/// Calcula una raíz de Merkle y duplica el último nodo de niveles impares.
pub fn merkle_root(leaves: &[&[u8]]) -> Option<[u8; 32]> {
    if leaves.is_empty() {
        return None;
    }

    let mut level: Vec<[u8; 32]> = leaves.iter().map(|leaf| sha256(leaf)).collect();
    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last = *level.last().expect("a nonempty level has a last node");
            level.push(last);
        }

        level = level
            .chunks_exact(2)
            .map(|pair| {
                let mut parent_input = [0_u8; 64];
                parent_input[..32].copy_from_slice(&pair[0]);
                parent_input[32..].copy_from_slice(&pair[1]);
                sha256(&parent_input)
            })
            .collect();
    }
    level.pop()
}

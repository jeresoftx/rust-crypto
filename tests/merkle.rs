use rust_crypto::{merkle::merkle_root, sha256::sha256};

#[test]
fn a_single_leaf_uses_its_hash_as_root() {
    assert_eq!(merkle_root(&[b"leaf" as &[u8]]), Some(sha256(b"leaf")));
}

#[test]
fn an_odd_level_duplicates_its_last_node() {
    let left = sha256(b"a");
    let right = sha256(b"b");
    let last = sha256(b"c");
    let first_parent = sha256(&[left, right].concat());
    let second_parent = sha256(&[last, last].concat());
    let expected = sha256(&[first_parent, second_parent].concat());

    assert_eq!(merkle_root(&[b"a", b"b", b"c"]), Some(expected));
}

#[test]
fn changing_a_leaf_changes_the_root() {
    assert_ne!(
        merkle_root(&[b"a" as &[u8], b"b" as &[u8]]),
        merkle_root(&[b"a" as &[u8], b"changed" as &[u8]])
    );
}

#[test]
fn an_empty_tree_has_no_root_without_an_extra_contract() {
    assert_eq!(merkle_root(&[]), None);
}

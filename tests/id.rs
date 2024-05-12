use thriller_core::{initialize, next_id};

#[test]
fn test_id_generation() {
    initialize();
    let id1 = next_id();
    assert!(id1 == 0);

    let id2 = next_id();
    assert!(id2 == 1);

    let id3 = next_id();
    assert!(id3 == 2);
}

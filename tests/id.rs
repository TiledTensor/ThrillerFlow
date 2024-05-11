use thriller_core::initialize;

#[test]
fn test_id_generation() {
    initialize();
    let id1 = thriller_flow::next_id();
    assert!(id1 == 0);

    let id2 = thriller_flow::next_id();
    assert!(id2 == 1);

    let id3 = thriller_flow::next_id();
    assert!(id3 == 2);
}

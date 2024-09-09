use thriller_core::{Dimension, Layout, Shape};

#[test]
fn test_strides() {
    let layout = Layout::RowMajor;
    let shape = Shape::new(&[2, 3, 4], layout);
    let strides = shape.get_strides();

    assert_eq!(strides.slice(), &[12, 4, 1]);
}

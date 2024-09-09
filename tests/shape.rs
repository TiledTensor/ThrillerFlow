use thriller_core::{Dimension, Layout, Shape};

#[test]
fn test_strides() {
    let layout_0 = Layout::RowMajor;
    let shape_0 = Shape::new(&[2, 3, 4], layout_0);
    let strides_0 = shape_0.get_strides();

    assert_eq!(strides_0.slice(), &[12, 4, 1]);

    let layout_1 = Layout::ColumnMajor;
    let shape_1 = Shape::new(&[2, 3, 4], layout_1);
    let strides_1 = shape_1.get_strides();

    assert_eq!(strides_1.slice(), &[1, 2, 6]);
}

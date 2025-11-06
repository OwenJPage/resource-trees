use super::super::*;

#[test]
fn test_10x5_ample_stock() {
    let starting_stock = PillCount::new(50, 50);
    let doses = [Dose::new(5, 10)];

    let tree = SearchTree::build(doses, starting_stock);

    const EXPECTED_SIZE: SizeType = 5;
    assert_eq!(
        tree.size(),
        EXPECTED_SIZE,
        "Tree size is {:#?} but expected {EXPECTED_SIZE}",
        tree.size()
    );

    const EXPECTED_HEIGHT: HeightType = 5;
    assert_eq!(
        tree.height(),
        EXPECTED_HEIGHT,
        "Tree height is {:#?} but expected {EXPECTED_HEIGHT}",
        tree.height()
    );

    const EXPECTED_TOPS_LEN: usize = 1;
    assert_eq!(
        tree.tops.len(),
        EXPECTED_TOPS_LEN,
        "Tree tops length is {} but expected {EXPECTED_TOPS_LEN}",
        tree.tops.len()
    );
}

#[test]
fn test_10x5_insufficient_stock() {
    let starting_stock = PillCount::new(2, 0);
    let doses = [Dose::new(5, 10)];

    let tree = SearchTree::build(doses, starting_stock);

    const EXPECTED_SIZE: SizeType = 2;
    assert_eq!(
        tree.size(),
        EXPECTED_SIZE,
        "Tree size is {:#?} but expected {EXPECTED_SIZE}",
        tree.size()
    );

    const EXPECTED_HEIGHT: HeightType = 2;
    assert_eq!(
        tree.height(),
        EXPECTED_HEIGHT,
        "Tree height is {:#?} but expected {EXPECTED_HEIGHT}",
        tree.height()
    );

    const EXPECTED_TOPS_LEN: usize = 1;
    assert_eq!(
        tree.tops.len(),
        EXPECTED_TOPS_LEN,
        "Tree tops length is {} but expected {EXPECTED_TOPS_LEN}",
        tree.tops.len()
    );
}

#[test]
fn test_20x5_ample_stock() {
    let starting_stock = PillCount::new(50, 50);
    let doses = [Dose::new(5, 20)];

    let tree = SearchTree::build(doses, starting_stock);

    const EXPECTED_SIZE: SizeType = 62;
    assert_eq!(
        tree.size(),
        EXPECTED_SIZE,
        "Tree size is {:#?} but expected {EXPECTED_SIZE}",
        tree.size()
    );

    const EXPECTED_HEIGHT: HeightType = 5;
    assert_eq!(
        tree.height(),
        EXPECTED_HEIGHT,
        "Tree height is {:#?} but expected {EXPECTED_HEIGHT}",
        tree.height()
    );

    const EXPECTED_TOPS_LEN: usize = 32;
    assert_eq!(
        tree.tops.len(),
        EXPECTED_TOPS_LEN,
        "Tree tops length is {} but expected {EXPECTED_TOPS_LEN}",
        tree.tops.len()
    );
}

#[test]
fn test_20x5_limited_20_stock() {
    let starting_stock = PillCount::new(50, 3);
    let doses = [Dose::new(5, 20)];

    let tree = SearchTree::build(doses, starting_stock);

    const EXPECTED_HEIGHT: HeightType = 5;
    assert_eq!(
        tree.height(),
        EXPECTED_HEIGHT,
        "Tree height is {:#?} but expected {EXPECTED_HEIGHT}",
        tree.height()
    );

    const EXPECTED_TOPS_LEN: usize = 26;
    assert_eq!(
        tree.tops.len(),
        EXPECTED_TOPS_LEN,
        "Tree tops length is {} but expected {EXPECTED_TOPS_LEN}",
        tree.tops.len()
    );
}

#[test]
fn test_20x5_limited_both_stock() {
    let starting_stock = PillCount::new(3, 3);
    let doses = [Dose::new(5, 20)];

    let tree = SearchTree::build(doses, starting_stock);

    const EXPECTED_HEIGHT: HeightType = 4;
    assert_eq!(
        tree.height(),
        EXPECTED_HEIGHT,
        "Tree height is {:#?} but expected {EXPECTED_HEIGHT}",
        tree.height()
    );

    const EXPECTED_TOPS_LEN: usize = 4;
    assert_eq!(
        tree.tops.len(),
        EXPECTED_TOPS_LEN,
        "Tree tops length is {} but expected {EXPECTED_TOPS_LEN}",
        tree.tops.len()
    );
}

#[test]
fn test_20x5_exact_both_stock() {
    let starting_stock = PillCount::new(4, 3);
    let doses = [Dose::new(5, 20)];

    let tree = SearchTree::build(doses, starting_stock);

    const EXPECTED_HEIGHT: HeightType = 5;
    assert_eq!(
        tree.height(),
        EXPECTED_HEIGHT,
        "Tree height is {:#?} but expected {EXPECTED_HEIGHT}",
        tree.height()
    );

    const EXPECTED_TOPS_LEN: usize = 10;
    assert_eq!(
        tree.tops.len(),
        EXPECTED_TOPS_LEN,
        "Tree tops length is {} but expected {EXPECTED_TOPS_LEN}",
        tree.tops.len()
    );
}

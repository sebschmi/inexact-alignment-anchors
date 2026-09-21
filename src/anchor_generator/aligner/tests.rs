use generic_a_star::cost::U32Cost;

use crate::{
    anchor_generator::aligner::{AnchorLimit, InexactAnchorLimitGenerator},
    costs::SimpleAlignmentCost,
};

fn simple_example(
    offset_a: usize,
    offset_b: usize,
    k: usize,
    max_mismatches: usize,
) -> Vec<AnchorLimit<U32Cost>> {
    let sequence_a = b"ACGT";
    let sequence_b = b"GACT";
    let costs = SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    };

    let mut generator = InexactAnchorLimitGenerator::new(
        offset_a,
        offset_b,
        sequence_a,
        sequence_b,
        &costs,
        k,
        max_mismatches,
    )
    .unwrap();
    let mut actual = Vec::new();
    while let Some(anchor_limit) = generator.next() {
        actual.push(anchor_limit);
    }
    actual.sort_unstable();
    actual
}

#[test]
fn simple_example_0_0_2_0() {
    let offset_a = 0;
    let offset_b = 0;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_0_1_2_0() {
    let offset_a = 0;
    let offset_b = 1;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![AnchorLimit::new(2, 2, U32Cost::from(0u32))];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_0_2_2_0() {
    let offset_a = 0;
    let offset_b = 2;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_1_0_2_0() {
    let offset_a = 1;
    let offset_b = 0;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_1_1_2_0() {
    let offset_a = 1;
    let offset_b = 1;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_1_2_2_0() {
    let offset_a = 1;
    let offset_b = 2;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_2_0_2_0() {
    let offset_a = 2;
    let offset_b = 0;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_2_1_2_0() {
    let offset_a = 2;
    let offset_b = 1;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_2_2_2_0() {
    let offset_a = 2;
    let offset_b = 2;
    let k = 2;
    let max_mismatches = 0;
    let actual = simple_example(offset_a, offset_b, k, max_mismatches);

    let mut expected = vec![];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

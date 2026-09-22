use std::collections::HashSet;

use generic_a_star::cost::U32Cost;

use crate::{
    anchor::Anchor, anchor_generator::InexactAlignmentAnchorGenerator, costs::SimpleAlignmentCost,
};

#[test]
fn simple_example_2_0() {
    let sequence_a = b"ACGT";
    let sequence_b = b"GACT";
    let k = 2;
    let max_mismatches = 0;
    let costs = SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    };

    let generator =
        InexactAlignmentAnchorGenerator::new(sequence_a, sequence_b, &costs, k, max_mismatches)
            .unwrap();
    let mut actual: Vec<_> = generator.collect();
    actual.sort_unstable();

    let mut expected = vec![Anchor::new(0, 2, 1, 3, U32Cost::from(0u32))];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn simple_example_2_1() {
    let sequence_a = b"ACGT";
    let sequence_b = b"GACT";
    let k = 2;
    let max_mismatches = 1;
    let costs = SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    };

    let generator =
        InexactAlignmentAnchorGenerator::new(sequence_a, sequence_b, &costs, k, max_mismatches)
            .unwrap();
    let mut actual: Vec<_> = generator.collect();
    actual.sort_unstable();

    let mut expected = vec![
        Anchor::new(0, 2, 1, 3, U32Cost::from(0u32)),
        Anchor::new(0, 2, 1, 2, U32Cost::from(3u32)),
        Anchor::new(0, 2, 2, 3, U32Cost::from(3u32)),
        Anchor::new(1, 3, 2, 3, U32Cost::from(3u32)),
        Anchor::new(1, 3, 0, 1, U32Cost::from(3u32)),
        Anchor::new(1, 3, 2, 4, U32Cost::from(2u32)),
        Anchor::new(2, 4, 0, 1, U32Cost::from(3u32)),
        Anchor::new(2, 4, 3, 4, U32Cost::from(3u32)),
        Anchor::new(2, 4, 2, 4, U32Cost::from(2u32)),
        Anchor::new(2, 4, 0, 2, U32Cost::from(2u32)),
        Anchor::new(0, 1, 0, 2, U32Cost::from(3u32)),
        Anchor::new(2, 3, 0, 2, U32Cost::from(3u32)),
        Anchor::new(0, 1, 1, 3, U32Cost::from(3u32)),
        Anchor::new(1, 2, 1, 3, U32Cost::from(3u32)),
        Anchor::new(1, 2, 2, 4, U32Cost::from(3u32)),
        Anchor::new(3, 4, 2, 4, U32Cost::from(3u32)),
        Anchor::new(0, 2, 0, 3, U32Cost::from(3u32)),
        Anchor::new(0, 2, 1, 4, U32Cost::from(3u32)),
        Anchor::new(0, 3, 1, 3, U32Cost::from(3u32)),
        Anchor::new(1, 4, 2, 4, U32Cost::from(3u32)),
    ];
    expected.sort_unstable();

    let actual_set: HashSet<_> = actual.iter().cloned().collect();
    let expected_set: HashSet<_> = expected.iter().cloned().collect();

    assert_eq!(
        actual,
        expected,
        "\nactual:\n{}\nexpected\n{}\nonly actual:\n{}\nonly expected:\n{}",
        actual
            .iter()
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
        expected
            .iter()
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
        actual_set
            .difference(&expected_set)
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
        expected_set
            .difference(&actual_set)
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
    );
}

#[test]
fn simple_example_3_2() {
    let sequence_a = b"ACGT";
    let sequence_b = b"GACT";
    let k = 3;
    let max_mismatches = 2;
    let costs = SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    };

    let generator =
        InexactAlignmentAnchorGenerator::new(sequence_a, sequence_b, &costs, k, max_mismatches)
            .unwrap();
    let mut actual: Vec<_> = generator.collect();
    actual.sort_unstable();

    // Suboptimal and duplicate anchors are commented out.
    let mut expected = vec![
        // 1 mismatch.
        Anchor::new(0, 3, 1, 3, U32Cost::from(3u32)),
        Anchor::new(0, 3, 1, 4, U32Cost::from(2u32)),
        Anchor::new(1, 4, 2, 4, U32Cost::from(3u32)),
        Anchor::new(0, 2, 0, 3, U32Cost::from(3u32)),
        Anchor::new(0, 2, 1, 4, U32Cost::from(3u32)),
        Anchor::new(0, 4, 1, 4, U32Cost::from(3u32)),
        // 2 deletions.
        Anchor::new(0, 3, 0, 1, U32Cost::from(4u32)),
        Anchor::new(0, 3, 2, 3, U32Cost::from(6u32)),
        Anchor::new(0, 3, 1, 2, U32Cost::from(4u32)),
        Anchor::new(1, 4, 2, 3, U32Cost::from(4u32)),
        Anchor::new(1, 4, 0, 1, U32Cost::from(6u32)),
        Anchor::new(1, 4, 3, 4, U32Cost::from(4u32)),
        Anchor::new(2, 3, 0, 3, U32Cost::from(4u32)),
        Anchor::new(0, 1, 0, 3, U32Cost::from(6u32)),
        Anchor::new(1, 2, 0, 3, U32Cost::from(4u32)),
        Anchor::new(0, 1, 1, 4, U32Cost::from(4u32)),
        Anchor::new(1, 2, 1, 4, U32Cost::from(6u32)),
        Anchor::new(3, 4, 1, 4, U32Cost::from(4u32)),
        // 1 deletion and 1 substitution.
        // Anchor::new(0, 3, 1, 3, U32Cost::from(5u32)),
        Anchor::new(0, 3, 2, 4, U32Cost::from(5u32)),
        // Anchor::new(1, 4, 2, 4, U32Cost::from(5u32)),
        Anchor::new(1, 4, 0, 2, U32Cost::from(5u32)),
        // Anchor::new(1, 4, 2, 4, U32Cost::from(5u32)),
        Anchor::new(2, 4, 0, 3, U32Cost::from(5u32)),
        // Anchor::new(0, 2, 0, 3, U32Cost::from(5u32)),
        // Anchor::new(0, 2, 1, 4, U32Cost::from(5u32)),
        Anchor::new(2, 4, 1, 4, U32Cost::from(5u32)),
        Anchor::new(1, 3, 1, 4, U32Cost::from(5u32)),
        // 1 deletion and 1 insertion.
        Anchor::new(0, 3, 0, 3, U32Cost::from(6u32)),
        // Anchor::new(1, 4, 1, 4, U32Cost::from(6u32)),
        // 2 substitutions.
        Anchor::new(1, 4, 1, 4, U32Cost::from(4u32)),
        // 1 substitution and 1 insertion.
        Anchor::new(0, 3, 0, 4, U32Cost::from(5u32)),
    ];
    expected.sort_unstable();

    let actual_set: HashSet<_> = actual.iter().cloned().collect();
    let expected_set: HashSet<_> = expected.iter().cloned().collect();

    assert_eq!(
        actual,
        expected,
        "\nactual:\n{}\nexpected\n{}\nonly actual:\n{}\nonly expected:\n{}",
        actual
            .iter()
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
        expected
            .iter()
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
        actual_set
            .difference(&expected_set)
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
        expected_set
            .difference(&actual_set)
            .map(ToString::to_string)
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            }),
    );
}

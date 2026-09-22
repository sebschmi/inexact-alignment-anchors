use std::collections::HashSet;

use generic_a_star::cost::U32Cost;

use crate::{
    anchor::Anchor, costs::SimpleAlignmentCost, generate_inexact_alignment_anchors_of_subsequences,
};

#[test]
fn subsequence_2_0() {
    let sequence_a = b"XXACGTXX";
    let sequence_b = b"XXGACTXX";
    let range = 2..6;
    let k = 2;
    let max_mismatches = 0;
    let costs = SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    };

    let generator = generate_inexact_alignment_anchors_of_subsequences(
        sequence_a,
        sequence_b,
        range.clone(),
        range,
        &costs,
        k,
        max_mismatches,
    )
    .unwrap();
    let mut actual: Vec<_> = generator.collect();
    actual.sort_unstable();

    let mut expected = vec![Anchor::new(2, 4, 3, 5, U32Cost::from(0u32))];
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn subsequence_2_1() {
    let sequence_a = b"XXACGTXX";
    let sequence_b = b"XXGACTXX";
    let range = 2..6;
    let k = 2;
    let max_mismatches = 1;
    let costs = SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    };

    let generator = generate_inexact_alignment_anchors_of_subsequences(
        sequence_a,
        sequence_b,
        range.clone(),
        range,
        &costs,
        k,
        max_mismatches,
    )
    .unwrap();
    let mut actual: Vec<_> = generator.collect();
    actual.sort_unstable();

    let mut expected = vec![
        Anchor::new(2, 4, 3, 5, U32Cost::from(0u32)),
        Anchor::new(2, 4, 3, 4, U32Cost::from(3u32)),
        Anchor::new(2, 4, 4, 5, U32Cost::from(3u32)),
        Anchor::new(3, 5, 4, 5, U32Cost::from(3u32)),
        Anchor::new(3, 5, 2, 3, U32Cost::from(3u32)),
        Anchor::new(3, 5, 4, 6, U32Cost::from(2u32)),
        Anchor::new(4, 6, 2, 3, U32Cost::from(3u32)),
        Anchor::new(4, 6, 5, 6, U32Cost::from(3u32)),
        Anchor::new(4, 6, 4, 6, U32Cost::from(2u32)),
        Anchor::new(4, 6, 2, 4, U32Cost::from(2u32)),
        Anchor::new(2, 3, 2, 4, U32Cost::from(3u32)),
        Anchor::new(4, 5, 2, 4, U32Cost::from(3u32)),
        Anchor::new(2, 3, 3, 5, U32Cost::from(3u32)),
        Anchor::new(3, 4, 3, 5, U32Cost::from(3u32)),
        Anchor::new(3, 4, 4, 6, U32Cost::from(3u32)),
        Anchor::new(5, 6, 4, 6, U32Cost::from(3u32)),
        Anchor::new(2, 4, 2, 5, U32Cost::from(3u32)),
        Anchor::new(2, 4, 3, 6, U32Cost::from(3u32)),
        Anchor::new(2, 5, 3, 5, U32Cost::from(3u32)),
        Anchor::new(3, 6, 4, 6, U32Cost::from(3u32)),
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
fn subsequence_3_2() {
    let sequence_a = b"XXACGTXX";
    let sequence_b = b"XXGACTXX";
    let range = 2..6;
    let k = 3;
    let max_mismatches = 2;
    let costs = SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    };

    let generator = generate_inexact_alignment_anchors_of_subsequences(
        sequence_a,
        sequence_b,
        range.clone(),
        range,
        &costs,
        k,
        max_mismatches,
    )
    .unwrap();
    let mut actual: Vec<_> = generator.collect();
    actual.sort_unstable();

    // Suboptimal and duplicate anchors are commented out.
    let mut expected = vec![
        // 1 mismatch.
        Anchor::new(2, 5, 3, 5, U32Cost::from(3u32)),
        Anchor::new(2, 5, 3, 6, U32Cost::from(2u32)),
        Anchor::new(3, 6, 4, 6, U32Cost::from(3u32)),
        Anchor::new(2, 4, 2, 5, U32Cost::from(3u32)),
        Anchor::new(2, 4, 3, 6, U32Cost::from(3u32)),
        Anchor::new(2, 6, 3, 6, U32Cost::from(3u32)),
        // 2 deletions.
        Anchor::new(2, 5, 2, 3, U32Cost::from(4u32)),
        Anchor::new(2, 5, 4, 5, U32Cost::from(6u32)),
        Anchor::new(2, 5, 3, 4, U32Cost::from(4u32)),
        Anchor::new(3, 6, 4, 5, U32Cost::from(4u32)),
        Anchor::new(3, 6, 2, 3, U32Cost::from(6u32)),
        Anchor::new(3, 6, 5, 6, U32Cost::from(4u32)),
        Anchor::new(4, 5, 2, 5, U32Cost::from(4u32)),
        Anchor::new(2, 3, 2, 5, U32Cost::from(6u32)),
        Anchor::new(3, 4, 2, 5, U32Cost::from(4u32)),
        Anchor::new(2, 3, 3, 6, U32Cost::from(4u32)),
        Anchor::new(3, 4, 3, 6, U32Cost::from(6u32)),
        Anchor::new(5, 6, 3, 6, U32Cost::from(4u32)),
        // 1 deletion and 1 substitution.
        // Anchor::new(2, 5, 3, 5, U32Cost::from(5u32)),
        Anchor::new(2, 5, 4, 6, U32Cost::from(5u32)),
        // Anchor::new(3, 6, 4, 6, U32Cost::from(5u32)),
        Anchor::new(3, 6, 2, 4, U32Cost::from(5u32)),
        // Anchor::new(3, 6, 4, 6, U32Cost::from(5u32)),
        Anchor::new(4, 6, 2, 5, U32Cost::from(5u32)),
        // Anchor::new(2, 4, 2, 5, U32Cost::from(5u32)),
        // Anchor::new(2, 4, 3, 6, U32Cost::from(5u32)),
        Anchor::new(4, 6, 3, 6, U32Cost::from(5u32)),
        Anchor::new(3, 5, 3, 6, U32Cost::from(5u32)),
        // 1 deletion and 1 insertion.
        Anchor::new(2, 5, 2, 5, U32Cost::from(6u32)),
        // Anchor::new(3, 6, 3, 6, U32Cost::from(6u32)),
        // 2 substitutions.
        Anchor::new(3, 6, 3, 6, U32Cost::from(4u32)),
        // 1 substitution and 1 insertion.
        Anchor::new(2, 5, 2, 6, U32Cost::from(5u32)),
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

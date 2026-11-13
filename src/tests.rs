use std::collections::HashSet;

use generic_a_star::cost::U32Cost;

use crate::{
    anchor::Anchor, costs::SimpleAlignmentCost, generate_inexact_alignment_anchors,
    generate_inexact_alignment_anchors_of_subsequences,
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

// Sequences that are shorter than k.
//
// An anchor has length exactly k on one sequence and at least k - max_mismatches on the other.
// If one of the sequences is shorter than that, the generator must produce the anchors that fit, or none, but not panic.
// These cases were found by the end-to-end tests of tschainalign in the template-switch-aligner repository,
// where they fail the debug assertions in `DpMatrixResultCellIter::next`.

fn short_sequence_costs() -> SimpleAlignmentCost {
    SimpleAlignmentCost {
        mismatch_cost: U32Cost::from(2u32),
        gap_open_cost: U32Cost::from(3u32),
        gap_extend_cost: U32Cost::from(1u32),
    }
}

/// Generate the anchors with both entry points of the crate and check that they agree.
fn generate_short_sequence_anchors(
    sequence_a: &[u8],
    sequence_b: &[u8],
    k: usize,
    max_mismatches: usize,
) -> Vec<Anchor<U32Cost>> {
    let costs = short_sequence_costs();

    let mut anchors: Vec<_> =
        generate_inexact_alignment_anchors(sequence_a, sequence_b, &costs, k, max_mismatches)
            .unwrap()
            .collect();
    anchors.sort_unstable();

    let mut subsequence_anchors: Vec<_> = generate_inexact_alignment_anchors_of_subsequences(
        sequence_a,
        sequence_b,
        0..sequence_a.len(),
        0..sequence_b.len(),
        &costs,
        k,
        max_mismatches,
    )
    .unwrap()
    .collect();
    subsequence_anchors.sort_unstable();

    assert_eq!(anchors, subsequence_anchors);
    anchors
}

/// Compute the expected anchors by appending padding to both sequences, such that both are long enough.
///
/// The anchors that lie completely within the original sequences do not depend on the padding,
/// so they are exactly the anchors of the original sequences.
fn expected_short_sequence_anchors(
    sequence_a: &[u8],
    sequence_b: &[u8],
    k: usize,
    max_mismatches: usize,
) -> Vec<Anchor<U32Cost>> {
    let padding = k + max_mismatches + 1;
    let padded_a: Vec<u8> = sequence_a
        .iter()
        .copied()
        .chain(std::iter::repeat_n(b'X', padding))
        .collect();
    let padded_b: Vec<u8> = sequence_b
        .iter()
        .copied()
        .chain(std::iter::repeat_n(b'Y', padding))
        .collect();

    let costs = short_sequence_costs();
    let mut anchors: Vec<_> =
        generate_inexact_alignment_anchors(&padded_a, &padded_b, &costs, k, max_mismatches)
            .unwrap()
            .filter(|anchor| {
                anchor.limit_a <= sequence_a.len() && anchor.limit_b <= sequence_b.len()
            })
            .collect();
    anchors.sort_unstable();
    anchors
}

fn check_short_sequences(sequence_a: &[u8], sequence_b: &[u8], k: usize, max_mismatches: usize) {
    let expected = expected_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches);
    let actual = generate_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches);
    assert_eq!(actual, expected);
}

/// Both sequences are shorter than k, so no anchor fits.
#[test]
fn both_sequences_shorter_than_k() {
    let sequence_a = b"ACGTACGTA";
    let sequence_b = b"ACGTACGTA";
    let k = 10;
    let max_mismatches = 1;

    assert_eq!(
        generate_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches),
        Vec::new()
    );
    check_short_sequences(sequence_a, sequence_b, k, max_mismatches);
}

/// Both sequences are much shorter than k, so no anchor fits.
#[test]
fn both_sequences_much_shorter_than_k() {
    let sequence_a = b"ACGTA";
    let sequence_b = b"ACGA";
    let k = 12;
    let max_mismatches = 2;

    assert_eq!(
        generate_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches),
        Vec::new()
    );
    check_short_sequences(sequence_a, sequence_b, k, max_mismatches);
}

/// Sequence a is shorter than k - max_mismatches, so no anchor fits, even though sequence b is long.
#[test]
fn sequence_a_shorter_than_k_minus_max_mismatches() {
    let sequence_a = b"A";
    let sequence_b = b"ACGTACGTACGTACGTACGT";
    let k = 6;
    let max_mismatches = 1;

    assert_eq!(
        generate_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches),
        Vec::new()
    );
    check_short_sequences(sequence_a, sequence_b, k, max_mismatches);
}

/// Sequence b is shorter than k - max_mismatches, so no anchor fits, even though sequence a is long.
#[test]
fn sequence_b_shorter_than_k_minus_max_mismatches() {
    let sequence_a = b"ACGTACGTACGT";
    let sequence_b = b"ACGTACGTAC";
    let k = 12;
    let max_mismatches = 1;

    assert_eq!(
        generate_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches),
        Vec::new()
    );
    check_short_sequences(sequence_a, sequence_b, k, max_mismatches);
}

/// Sequence a is one shorter than k and sequence b has length k,
/// so the only anchor aligns all of a to all of b with one gap.
#[test]
fn sequence_a_one_shorter_than_k() {
    let sequence_a = b"ACG";
    let sequence_b = b"ACGT";
    let k = 4;
    let max_mismatches = 1;

    assert_eq!(
        generate_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches),
        vec![Anchor::new(0, 3, 0, 4, U32Cost::from(3u32))]
    );
    check_short_sequences(sequence_a, sequence_b, k, max_mismatches);
}

/// Sequence b is one shorter than k and sequence a has length k,
/// so the only anchor aligns all of a to all of b with one gap.
#[test]
fn sequence_b_one_shorter_than_k() {
    let sequence_a = b"ACGT";
    let sequence_b = b"ACG";
    let k = 4;
    let max_mismatches = 1;

    assert_eq!(
        generate_short_sequence_anchors(sequence_a, sequence_b, k, max_mismatches),
        vec![Anchor::new(0, 4, 0, 3, U32Cost::from(3u32))]
    );
    check_short_sequences(sequence_a, sequence_b, k, max_mismatches);
}

/// Sequence a is shorter than k, but long enough for anchors with gaps, and sequence b is long.
#[test]
fn sequence_a_shorter_than_k_and_sequence_b_long() {
    check_short_sequences(b"ACGTACGTA", b"TTACGTACGTACTT", 10, 1);
    check_short_sequences(b"ACGTACGT", b"TTACGTACGTACTT", 10, 2);
}

/// Sequence b is shorter than k, but long enough for anchors with gaps, and sequence a is long.
#[test]
fn sequence_b_shorter_than_k_and_sequence_a_long() {
    check_short_sequences(b"TTACGTACGTACTT", b"ACGTACGTA", 10, 1);
    check_short_sequences(b"TTACGTACGTACTT", b"ACGTACGT", 10, 2);
}

/// Exhaustively check all small length combinations around k.
/// All failing combinations are collected and reported together.
#[test]
fn all_short_length_combinations() {
    let sequence = b"ACGTTGCAACGGTCA";
    let mut failures = Vec::new();
    let mut total = 0;

    for k in 2..=6 {
        for max_mismatches in 1..k {
            for length_a in 0..=k + max_mismatches + 1 {
                for length_b in 0..=k + max_mismatches + 1 {
                    if length_a == 0 && length_b == 0 {
                        continue;
                    }
                    total += 1;
                    let sequence_a = &sequence[..length_a];
                    let sequence_b = &sequence[sequence.len() - length_b..];
                    let result = std::panic::catch_unwind(|| {
                        check_short_sequences(sequence_a, sequence_b, k, max_mismatches)
                    });
                    if let Err(payload) = result {
                        let message = payload
                            .downcast_ref::<String>()
                            .map(String::as_str)
                            .or_else(|| payload.downcast_ref::<&str>().copied())
                            .unwrap_or("<unknown panic>")
                            .lines()
                            .next()
                            .unwrap_or_default()
                            .to_string();
                        failures.push(format!(
                            "k = {k}, max_mismatches = {max_mismatches}, |a| = {length_a}, |b| = {length_b}: {message}"
                        ));
                    }
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {total} combinations failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

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

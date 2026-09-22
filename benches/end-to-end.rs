use divan::Bencher;
use inexact_alignment_anchors::{
    costs::SimpleAlignmentCost, generate_inexact_alignment_anchors,
    generate_inexact_alignment_anchors_of_subsequences,
};
use rand::{rngs::ThreadRng, seq::IndexedRandom};

fn main() {
    divan::main()
}

#[divan::bench(args = [10, 100, 1000])]
fn bench_full(bencher: Bencher, len: usize) {
    let mut rand = ThreadRng::default();
    let sequence_a: Vec<_> = b"ACGT"
        .choose_iter(&mut rand)
        .unwrap()
        .take(len)
        .copied()
        .collect();
    let sequence_b: Vec<_> = b"ACGT"
        .choose_iter(&mut rand)
        .unwrap()
        .take(len)
        .copied()
        .collect();
    let costs = SimpleAlignmentCost {
        mismatch_cost: 2u32.into(),
        gap_open_cost: 3u32.into(),
        gap_extend_cost: 1u32.into(),
    };
    let k = 8;
    let max_mismatches = 2;

    bencher.counter(len).bench(|| {
        generate_inexact_alignment_anchors(&sequence_a, &sequence_b, &costs, k, max_mismatches)
            .unwrap()
            .collect::<Vec<_>>()
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn bench_subsequence(bencher: Bencher, len: usize) {
    let mut rand = ThreadRng::default();
    let sequence_a: Vec<_> = b"ACGT"
        .choose_iter(&mut rand)
        .unwrap()
        .take(len)
        .copied()
        .collect();
    let sequence_b: Vec<_> = b"ACGT"
        .choose_iter(&mut rand)
        .unwrap()
        .take(len)
        .copied()
        .collect();
    let range = len / 4..len / 4 * 3;
    let costs = SimpleAlignmentCost {
        mismatch_cost: 2u32.into(),
        gap_open_cost: 3u32.into(),
        gap_extend_cost: 1u32.into(),
    };
    let k = 8;
    let max_mismatches = 2;

    bencher.counter(len).bench(|| {
        generate_inexact_alignment_anchors_of_subsequences(
            &sequence_a,
            &sequence_b,
            range.clone(),
            range.clone(),
            &costs,
            k,
            max_mismatches,
        )
        .unwrap()
        .collect::<Vec<_>>()
    });
}

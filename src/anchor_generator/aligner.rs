use std::{fmt::Display, iter, marker::PhantomData};

use generic_a_star::{
    AStar, AStarContext, AStarIdentifier, AStarNode, cost::AStarCost, reset::Reset,
};

use crate::{
    anchor_generator::result_cell_iter::DpMatrixResultCellIter, costs::AlignmentCost, error::Error,
};

pub struct InexactAnchorLimitGenerator<
    'context,
    Character: Eq,
    Cost: AStarCost,
    AlignmentCostImpl: AlignmentCost<Character, Cost>,
> {
    a_star: AStar<Context<'context, Cost, Character, AlignmentCostImpl>>,
    targets: Vec<AnchorLimit<Cost>>,
}

pub struct AnchorLimit<Cost> {
    pub limit_a: usize,
    pub limit_b: usize,
    pub cost: Cost,
}

struct Context<'context, Cost, Character, AlignmentCostImpl> {
    sequence_a: &'context [Character],
    sequence_b: &'context [Character],
    costs: &'context AlignmentCostImpl,
    k: u8,
    max_mismatches: u8,
    phantom_data: PhantomData<Cost>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node<Cost> {
    identifier: Identifier,
    cost: Cost,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Identifier {
    limit_a: u8,
    limit_b: u8,
    gap_type: GapType,
    mismatches: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GapType {
    None,
    GapA,
    GapB,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct EdgeType;

impl<'context, Character: Eq, Cost: AStarCost, AlignmentCostImpl: AlignmentCost<Character, Cost>>
    InexactAnchorLimitGenerator<'context, Character, Cost, AlignmentCostImpl>
{
    pub fn new(
        offset_a: usize,
        offset_b: usize,
        complete_sequence_a: &'context [Character],
        complete_sequence_b: &'context [Character],
        costs: &'context AlignmentCostImpl,
        k: usize,
        max_mismatches: usize,
    ) -> Result<Self, Error> {
        let context = Context::new(
            &complete_sequence_a
                [offset_a..complete_sequence_a.len().min(offset_a + k + max_mismatches)],
            &complete_sequence_b
                [offset_b..complete_sequence_b.len().min(offset_b + k + max_mismatches)],
            costs,
            k.try_into().map_err(|_| Error::KTooLarge { k })?,
            max_mismatches
                .try_into()
                .map_err(|_| Error::MaxMismatchesTooLarge { max_mismatches })?,
        );
        let a_star = AStar::new(context);
        let mut result = Self {
            a_star,
            targets: Vec::new(),
        };
        result.reset(offset_a, offset_b, complete_sequence_a, complete_sequence_b);
        Ok(result)
    }

    pub fn reset(
        &mut self,
        offset_a: usize,
        offset_b: usize,
        complete_sequence_a: &'context [Character],
        complete_sequence_b: &'context [Character],
    ) {
        self.targets.clear();
        self.a_star.reset();
        self.a_star.context_mut().sequence_a = &complete_sequence_a[offset_a
            ..complete_sequence_a.len().min(
                offset_a
                    + self.a_star.context().k as usize
                    + self.a_star.context().max_mismatches as usize,
            )];
        self.a_star.context_mut().sequence_b = &complete_sequence_b[offset_b
            ..complete_sequence_b.len().min(
                offset_b
                    + self.a_star.context().k as usize
                    + self.a_star.context().max_mismatches as usize,
            )];

        // Do an exhaustive search to find all possible targets.
        self.a_star.initialise();
        self.a_star
            .search_until_with_target_policy(|_, _| false, false);

        let k = self.a_star.context().k.into();
        let max_mismatches = self.a_star.context().max_mismatches;
        let max_mismatches_usize = max_mismatches.into();
        let subsequence_a_len = self.a_star.context().sequence_a.len();
        let subsequence_b_len = self.a_star.context().sequence_b.len();

        let mut result_cell_iter = DpMatrixResultCellIter::new(k, max_mismatches_usize);

        while let Some((limit_a, limit_b)) = result_cell_iter.next(
            subsequence_a_len,
            subsequence_b_len,
            k,
            max_mismatches_usize,
        ) {
            let limit_a = u8::try_from(limit_a).unwrap();
            let limit_b = u8::try_from(limit_b).unwrap();
            self.targets.extend(
                (0..=max_mismatches)
                    .flat_map(|mismatches| {
                        [
                            (mismatches, GapType::None),
                            (mismatches, GapType::GapA),
                            (mismatches, GapType::GapB),
                        ]
                    })
                    .flat_map(|(mismatches, gap_type)| {
                        self.a_star
                            .closed_node(&Identifier {
                                limit_a,
                                limit_b,
                                gap_type,
                                mismatches,
                            })
                            .map(|node| node.cost)
                    })
                    .min()
                    .map(|cost| AnchorLimit {
                        limit_a: limit_a.into(),
                        limit_b: limit_b.into(),
                        cost,
                    }),
            );
        }
    }

    pub fn next(&mut self) -> Option<AnchorLimit<Cost>> {
        self.targets.pop()
    }

    pub fn peek(&self) -> Option<&AnchorLimit<Cost>> {
        self.targets.last()
    }
}

impl<'context, Cost, Character, AlignmentCostImpl>
    Context<'context, Cost, Character, AlignmentCostImpl>
{
    pub fn new(
        sequence_a: &'context [Character],
        sequence_b: &'context [Character],
        costs: &'context AlignmentCostImpl,
        k: u8,
        max_mismatches: u8,
    ) -> Self {
        assert!(sequence_a.len() <= u8::MAX as usize);
        assert!(sequence_b.len() <= u8::MAX as usize);

        Self {
            sequence_a,
            sequence_b,
            costs,
            k,
            max_mismatches,
            phantom_data: PhantomData,
        }
    }
}

impl<'context, Cost: AStarCost, Character: Eq, AlignmentCostImpl: AlignmentCost<Character, Cost>>
    AStarContext for Context<'context, Cost, Character, AlignmentCostImpl>
{
    type Node = Node<Cost>;

    fn create_root(&self) -> Self::Node {
        Node {
            identifier: Identifier {
                limit_a: 0,
                limit_b: 0,
                gap_type: GapType::None,
                mismatches: 0,
            },
            cost: Cost::zero(),
        }
    }

    fn generate_successors(&mut self, node: &Self::Node, output: &mut impl Extend<Self::Node>) {
        let Node {
            identifier:
                Identifier {
                    limit_a,
                    limit_b,
                    gap_type,
                    mismatches,
                },
            cost,
        } = node;
        let limit_a_usize = usize::from(*limit_a);
        let limit_b_usize = usize::from(*limit_b);

        // Match or mismatch.
        if limit_a_usize < self.sequence_a.len() && limit_b_usize < self.sequence_b.len() {
            let a = &self.sequence_a[limit_a_usize];
            let b = &self.sequence_b[limit_b_usize];
            let cost_increment = self.costs.cost(a, b);
            let is_mismatch = a != b;
            let mismatches = *mismatches + if is_mismatch { 1 } else { 0 };
            if mismatches <= self.max_mismatches {
                let identifier = Identifier {
                    limit_a: *limit_a + 1,
                    limit_b: *limit_b + 1,
                    gap_type: GapType::None,
                    mismatches,
                };
                let cost = *cost + cost_increment;
                output.extend(iter::once(Node { identifier, cost }));
            }
        }

        // Gap in b.
        if limit_a_usize < self.sequence_a.len() {
            let cost_increment = if *gap_type == GapType::GapB {
                self.costs.gap_extend_cost()
            } else {
                self.costs.gap_open_cost()
            };
            let mismatches = *mismatches + 1;
            if mismatches <= self.max_mismatches {
                let identifier = Identifier {
                    limit_a: *limit_a + 1,
                    limit_b: *limit_b,
                    gap_type: GapType::GapB,
                    mismatches,
                };
                let cost = *cost + cost_increment;
                output.extend(iter::once(Node { identifier, cost }));
            }
        }

        // Gap in a.
        if limit_b_usize < self.sequence_b.len() {
            let cost_increment = if *gap_type == GapType::GapA {
                self.costs.gap_extend_cost()
            } else {
                self.costs.gap_open_cost()
            };
            let mismatches = *mismatches + 1;
            if mismatches <= self.max_mismatches {
                let identifier = Identifier {
                    limit_a: *limit_a,
                    limit_b: *limit_b + 1,
                    gap_type: GapType::GapA,
                    mismatches,
                };
                let cost = *cost + cost_increment;
                output.extend(iter::once(Node { identifier, cost }));
            }
        }
    }

    fn is_target(&self, node: &Self::Node) -> bool {
        let Node {
            identifier: Identifier {
                limit_a, limit_b, ..
            },
            ..
        } = node;
        (*limit_a == self.k
            && *limit_b >= self.k - self.max_mismatches
            && *limit_b <= self.k + self.max_mismatches)
            || (*limit_b == self.k
                && *limit_a >= self.k - self.max_mismatches
                && *limit_a <= self.k + self.max_mismatches)
    }

    fn cost_limit(&self) -> Option<<Self::Node as generic_a_star::AStarNode>::Cost> {
        None
    }

    fn memory_limit(&self) -> Option<usize> {
        None
    }
}

impl<'context, Cost, Character, AlignmentCostImpl: AlignmentCost<Character, Cost>> Reset
    for Context<'context, Cost, Character, AlignmentCostImpl>
{
    fn reset(&mut self) {
        // Do nothing.
    }
}

impl<Cost: AStarCost> AStarNode for Node<Cost> {
    type Identifier = Identifier;

    type EdgeType = EdgeType;

    type Cost = Cost;

    fn identifier(&self) -> &Self::Identifier {
        &self.identifier
    }

    fn cost(&self) -> Self::Cost {
        self.cost
    }

    fn a_star_lower_bound(&self) -> Self::Cost {
        Cost::zero()
    }

    fn secondary_maximisable_score(&self) -> usize {
        0
    }

    fn predecessor(&self) -> Option<&Self::Identifier> {
        unimplemented!("Backtracking not supported.")
    }

    fn predecessor_edge_type(&self) -> Option<Self::EdgeType> {
        unimplemented!("Backtracking not supported.")
    }
}

impl<Cost: Ord> Ord for Node<Cost> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl<Cost: Ord> PartialOrd for Node<Cost> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Cost: Display> Display for Node<Cost> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "N({})", self.cost)
    }
}

impl AStarIdentifier for Identifier {}

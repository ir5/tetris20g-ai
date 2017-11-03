use std::collections::{BinaryHeap, BTreeSet};
use core::{Field, PieceState, new_piece, Command, CommandResult, FixedInfo, apply_command};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct SearchNode {
    neg_cost: i32,
    state: PieceState,
    lock_delay: i8,
    synchro_move: i8,
}

pub fn enumerate_single(field: &Field, piece_type: u8) -> Vec<FixedInfo> {
    let initial_state = new_piece(piece_type);
    let mut queue = BinaryHeap::<SearchNode>::new();
    queue.push(SearchNode {
        neg_cost: 0,
        state: initial_state,
        lock_delay: 0,
        synchro_move: 0,
    });
    let mut visited: BTreeSet<SearchNode> = BTreeSet::new();
    let mut result: BTreeSet<FixedInfo> = BTreeSet::new();

    while let Some(node) = queue.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());

        let mut commands = vec![Command::Fix];
        for m in vec![-1, 1] {
            commands.push(Command::Move(m, 0));
            commands.push(Command::Move(0, m));
            if node.synchro_move != 0 {
                commands.push(Command::Move(node.synchro_move, m));
            }
        }

        for command in commands {
            match apply_command(&field, &node.state, &command) {
                CommandResult::Moved(next_state, reset) => {
                    let next_lock_delay = if reset { 0 } else { node.lock_delay + 1 };
                    if next_lock_delay >= 4 {
                        continue;
                    }
                    let mut synchro_move = 0;
                    if node.state == next_state {
                        if let Command::Move(m, 0) = command {
                            synchro_move = m;
                        }
                    }
                    queue.push(SearchNode {
                        neg_cost: node.neg_cost - 1,
                        state: next_state,
                        lock_delay: next_lock_delay,
                        synchro_move,
                    });
                }
                CommandResult::Fixed(info) => {
                    result.insert(info);
                }
                _ => (),
            }
        }
    }

    result.into_iter().collect()
}

pub fn enumerate_multi(field: &Field, piece_types: &Vec<u8>) -> Vec<Vec<FixedInfo>> {
    fn recurse(
        field: &Field,
        idx: usize,
        parent_trajectory: Vec<FixedInfo>,
        piece_types: &Vec<u8>,
        res: &mut Vec<Vec<FixedInfo>>,
    ) {
        if idx == piece_types.len() {
            res.push(parent_trajectory.clone());
            assert_eq!(piece_types.len(), res[0].len());
            return;
        }
        let candidates = enumerate_single(&field, piece_types[idx]);
        for candidate in candidates {
            let mut new_parent = parent_trajectory.clone();
            new_parent.push(candidate.clone());
            recurse(&candidate.new_field, idx + 1, new_parent, &piece_types, res);
        }
    }

    let mut res: Vec<Vec<FixedInfo>> = vec![];
    recurse(&field, 0, vec![], &piece_types, &mut res);

    res
}

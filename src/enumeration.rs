use std::collections::{BinaryHeap, BTreeSet};
use core::{Field, CurrentPieceState, new_piece, Command, CommandResult, apply_command};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct SearchNode {
    neg_cost: i32,
    state: CurrentPieceState,
    lock_delay: i8,
    synchro_move: i8,
}

pub fn enumerate_single(field: &Field, piece_type: u8) -> Vec<CurrentPieceState> {
    let initial_state = new_piece(piece_type);
    let mut queue = BinaryHeap::<SearchNode>::new();
    queue.push(SearchNode {
        neg_cost: 0,
        state: initial_state,
        lock_delay: 0,
        synchro_move: 0 });
    let mut visited: BTreeSet<SearchNode> = BTreeSet::new();
    let mut result: BTreeSet<CurrentPieceState> = BTreeSet::new();

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
                    if next_lock_delay >= 5 {
                        continue;
                    }
                    let mut synchro_move = 0;
                    if node.state == next_state {
                        if let Command::Move(m, 0) = command {
                            synchro_move = m;
                        }
                    }
                    queue.push(SearchNode{
                        neg_cost: node.neg_cost - 1,
                        state: next_state,
                        lock_delay: next_lock_delay,
                        synchro_move,
                    });
                }
                CommandResult::Fixed(fixed_state, _, _) => {
                    result.insert(fixed_state.clone());
                }
                _ => (),
            }
        }
    }

    result.into_iter().collect()
}

//! Module for enumerating possible moves.
use std::collections::{BinaryHeap, HashSet, HashMap};
use core::{Field, PieceState, new_piece, Command, CommandResult, FixedInfo, apply_command};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Hash)]
struct SearchNode {
    neg_cost: i32,
    state: PieceState,
    lock_delay: i8,
    synchro_move: i8,
}

/// Enumerates possible moves in a single step.
pub fn enumerate_single(field: &Field, piece_type: u8) -> Vec<FixedInfo> {
    let initial_state = new_piece(piece_type);
    let mut queue = BinaryHeap::<SearchNode>::new();
    queue.push(SearchNode {
        neg_cost: 0,
        state: initial_state,
        lock_delay: 0,
        synchro_move: 0,
    });
    let mut visited: HashSet<SearchNode> = HashSet::new();
    let mut result: HashSet<FixedInfo> = HashSet::new();

    while let Some(node) = queue.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());

        let (new_nodes, fixed_infos) = transition(&node, &field);
        for new_node in new_nodes {
            if visited.contains(&new_node) {
                continue;
            }
            queue.push(new_node);
        }
        for fixed_info in fixed_infos {
            result.insert(fixed_info);
        }
    }

    result.into_iter().collect()
}

/// Enumerates possible moves in multiple steps.
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

fn transition(node: &SearchNode, field: &Field) -> (Vec<SearchNode>, Vec<FixedInfo>) {
    let mut commands = vec![Command::Fix];
    for m in vec![-1, 1] {
        commands.push(Command::Move(m, 0));
        commands.push(Command::Move(0, m));
        if node.synchro_move != 0 {
            commands.push(Command::Move(node.synchro_move, m));
        }
    }

    let mut new_nodes = vec![];
    let mut fixed_infos = vec![];
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
                let new_node = SearchNode {
                    neg_cost: node.neg_cost - 1,
                    state: next_state,
                    lock_delay: next_lock_delay,
                    synchro_move,
                };
                new_nodes.push(new_node);
            }
            CommandResult::Fixed(info) => {
                fixed_infos.push(info);
            }
            _ => (),
        }
    }

    (new_nodes, fixed_infos)
}

pub fn find_command_sequence(field: &Field, piece_type: u8, dest_state: &PieceState) -> Vec<Command> {
    let initial_state = new_piece(piece_type);
    let mut queue = BinaryHeap::<(SearchNode, SearchNode)>::new(); // (current, previous)
    let initial_node = SearchNode {
        neg_cost: 0,
        state: initial_state,
        lock_delay: 0,
        synchro_move: 0,
    };
    queue.push((initial_node.clone(), initial_node.clone()));
    let mut visited: HashMap<SearchNode, SearchNode> = HashMap::new();

    let mut last_node = initial_node.clone();  // dummy
    'search_loop: while let Some((node, prev)) = queue.pop() {
        if visited.contains_key(&node) {
            continue;
        }
        visited.insert(node.clone(), prev.clone());

        let (new_nodes, fixed_infos) = transition(&node, &field);
        for new_node in new_nodes {
            queue.push((new_node, node.clone()));
        }
        for fixed_info in fixed_infos {
            if fixed_info.last_state == *dest_state {
                last_node = node.clone();
                break 'search_loop;
            }
        }
    }

    // trace-back
    let mut seq = vec![Command::Fix];
    let mut node = last_node;
    while node != initial_node {
        let prev = visited.get(&node).unwrap();

        // find the best transition from prev to state
        let mut commands = vec![];
        for m in vec![-1, 1] {
            commands.push(Command::Move(m, 0));
        }
        for m in vec![-1, 1] {
            commands.push(Command::Move(0, m));
        }
        for m in vec![-1, 1] {
            if prev.synchro_move != 0 {
                commands.push(Command::Move(prev.synchro_move, m));
            }
        }

        for command in commands {
            if let CommandResult::Moved(next_state, _) = apply_command(&field, &prev.state, &command) {
                if next_state == node.state {
                    seq.push(command);
                    break;
                }
            }
        }
        node = prev.clone();
    }

    seq.reverse();
    seq
}


#[cfg(test)]
mod tests {
    use core;
    use super::*;

    #[test]
    fn test_find_command_sequence() {
        let field = core::EMPTY_FIELD;
        let piece_type = b'L';
        let dest_state = core::PieceState {
            piece_type,
            x: 1,
            y: 16,
            rotation: 2,
            first: false,
        };

        let seq = find_command_sequence(&field, piece_type, &dest_state);

        let mut curr = core::new_piece(piece_type);
        for command in seq {
            match apply_command(&field, &curr, &command) {
                CommandResult::Moved(next_state, _) => {
                    curr = next_state;
                }
                CommandResult::Fixed(info) => {
                    curr = info.last_state.clone();
                }
                _ => (),
            }
        }
        assert_eq!(curr, dest_state);
    }
}

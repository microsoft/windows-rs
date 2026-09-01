use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeyedOperation<K> {
    Remove { key: K },
    Insert { key: K, before: Option<K> },
    Move { key: K, before: Option<K> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DuplicateKeyError<K>(pub K);

pub fn diff<K>(previous: &[K], next: &[K]) -> Result<Vec<KeyedOperation<K>>, DuplicateKeyError<K>>
where
    K: Clone + Eq + Hash,
{
    if previous == next {
        return Ok(Vec::new());
    }

    let previous_indices = unique_indices(previous)?;
    let next_indices = unique_indices(next)?;
    let mut operations = Vec::new();

    for key in previous.iter().rev() {
        if !next_indices.contains_key(key) {
            operations.push(KeyedOperation::Remove { key: key.clone() });
        }
    }

    let retained = next
        .iter()
        .enumerate()
        .filter_map(|(next_index, key)| {
            previous_indices
                .get(key)
                .map(|previous_index| (next_index, *previous_index))
        })
        .collect::<Vec<_>>();
    let stable = longest_increasing_positions(&retained);

    for (next_index, key) in next.iter().enumerate().rev() {
        let before = next.get(next_index + 1).cloned();
        if !previous_indices.contains_key(key) {
            operations.push(KeyedOperation::Insert {
                key: key.clone(),
                before,
            });
        } else if !stable.contains(&next_index) {
            operations.push(KeyedOperation::Move {
                key: key.clone(),
                before,
            });
        }
    }

    Ok(operations)
}

fn unique_indices<K>(keys: &[K]) -> Result<HashMap<K, usize>, DuplicateKeyError<K>>
where
    K: Clone + Eq + Hash,
{
    let mut indices = HashMap::with_capacity(keys.len());
    for (index, key) in keys.iter().enumerate() {
        if indices.insert(key.clone(), index).is_some() {
            return Err(DuplicateKeyError(key.clone()));
        }
    }
    Ok(indices)
}

fn longest_increasing_positions(sequence: &[(usize, usize)]) -> HashSet<usize> {
    let mut tails = Vec::<usize>::new();
    let mut predecessors = vec![None; sequence.len()];

    for (sequence_index, &(_, value)) in sequence.iter().enumerate() {
        let position = tails.partition_point(|tail| sequence[*tail].1 < value);
        if position > 0 {
            predecessors[sequence_index] = Some(tails[position - 1]);
        }
        if position == tails.len() {
            tails.push(sequence_index);
        } else {
            tails[position] = sequence_index;
        }
    }

    let mut positions = HashSet::with_capacity(tails.len());
    let mut cursor = tails.last().copied();
    while let Some(sequence_index) = cursor {
        positions.insert(sequence[sequence_index].0);
        cursor = predecessors[sequence_index];
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            (self.0 >> 32) as usize
        }
    }

    fn apply<K>(items: &mut Vec<K>, operations: &[KeyedOperation<K>])
    where
        K: Clone + Eq + std::fmt::Debug,
    {
        for operation in operations {
            match operation {
                KeyedOperation::Remove { key } => {
                    let index = items.iter().position(|item| item == key).unwrap();
                    items.remove(index);
                }
                KeyedOperation::Insert { key, before } => {
                    let index = before.as_ref().map_or(items.len(), |before| {
                        items.iter().position(|item| item == before).unwrap()
                    });
                    items.insert(index, key.clone());
                }
                KeyedOperation::Move { key, before } => {
                    let current = items.iter().position(|item| item == key).unwrap();
                    let key = items.remove(current);
                    let index = before.as_ref().map_or(items.len(), |before| {
                        items.iter().position(|item| item == before).unwrap()
                    });
                    items.insert(index, key);
                }
            }
        }
    }

    #[test]
    fn identical_input_has_no_operations() {
        assert_eq!(diff(&[1, 2, 3], &[1, 2, 3]), Ok(Vec::new()));
    }

    #[test]
    fn reverse_preserves_one_item_and_moves_the_rest() {
        let operations = diff(&[1, 2, 3, 4], &[4, 3, 2, 1]).unwrap();
        let moves = operations
            .iter()
            .filter(|operation| matches!(operation, KeyedOperation::Move { .. }))
            .count();
        let mut actual = vec![1, 2, 3, 4];

        apply(&mut actual, &operations);

        assert_eq!(actual, [4, 3, 2, 1]);
        assert_eq!(moves, 3);
    }

    #[test]
    fn rejects_duplicates_in_either_input() {
        assert_eq!(diff(&[1, 1], &[1]), Err(DuplicateKeyError(1)));
        assert_eq!(diff(&[1], &[1, 1]), Err(DuplicateKeyError(1)));
    }

    #[test]
    fn randomized_operations_reproduce_target_order() {
        let mut rng = Rng(0x51ced);
        for _ in 0..5_000 {
            let previous_len = rng.next() % 24;
            let next_len = rng.next() % 24;
            let mut pool = (0..32).collect::<Vec<_>>();
            for index in (1..pool.len()).rev() {
                let other = rng.next() % (index + 1);
                pool.swap(index, other);
            }
            let previous = pool[..previous_len].to_vec();

            for index in (1..pool.len()).rev() {
                let other = rng.next() % (index + 1);
                pool.swap(index, other);
            }
            let next = pool[..next_len].to_vec();
            let operations = diff(&previous, &next).unwrap();
            let mut actual = previous;

            apply(&mut actual, &operations);

            assert_eq!(actual, next);
        }
    }
}

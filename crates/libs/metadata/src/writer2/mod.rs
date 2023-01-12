mod blobs;
mod strings;
mod tables;
mod references;

use std::collections::BTreeMap;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

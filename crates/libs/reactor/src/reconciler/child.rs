use std::borrow::Cow;

use rustc_hash::{FxHashMap, FxHashSet};

use super::*;

fn mounted_output<B: Backend + 'static>(
    reconciler: &Reconciler<B>,
    parent: ControlId,
    index: usize,
) -> MountedOutput {
    reconciler
        .tree
        .logical_child(parent, index)
        .expect("logical child slot missing")
}

fn output_for_token<B: Backend + 'static>(
    reconciler: &Reconciler<B>,
    parent: ControlId,
    token: LogicalSlotId,
) -> Option<usize> {
    reconciler
        .tree
        .logical_children(parent)
        .iter()
        .position(|output| output.slot == token)
}

fn forced<B: Backend + 'static>(reconciler: &Reconciler<B>, output: MountedOutput) -> bool {
    output
        .native
        .is_some_and(|id| reconciler.is_control_forced(id))
        || output
            .logical
            .is_some_and(|id| reconciler.pass.forced_nodes.contains(&id))
}

fn live_index<B: Backend + 'static>(
    reconciler: &Reconciler<B>,
    parent: ControlId,
    control: ControlId,
) -> Option<usize> {
    reconciler.tree.child_position(parent, control)
}

fn live_len<B: Backend + 'static>(reconciler: &Reconciler<B>, parent: ControlId) -> usize {
    reconciler.tree.children(parent).len()
}

pub fn reconcile<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    old_live: LiveChildrenRef<'_>,
    new_live: LiveChildrenRef<'_>,
) {
    let has_keys = old_live.any_has_key() || new_live.any_has_key();

    if has_keys {
        reconcile_keyed_live(reconciler, parent, &old_live, &new_live);
    } else {
        reconcile_positional_live(reconciler, parent, &old_live, &new_live);
    }
}

pub fn reconcile_positional<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    old_live: LiveChildrenRef<'_>,
    new_live: LiveChildrenRef<'_>,
) {
    reconcile_positional_live(reconciler, parent, &old_live, &new_live);
}

fn reconcile_positional_live<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    old_live: &LiveChildrenRef<'_>,
    new_live: &LiveChildrenRef<'_>,
) {
    let old_len = old_live.len();
    let new_len = new_live.len();
    let common = old_len.min(new_len);

    debug_assert_eq!(
        reconciler.tree.logical_children(parent).len(),
        old_len,
        "logical child mirror disagrees with old child list"
    );

    for i in 0..common {
        let old_el = old_live.get(i).unwrap();
        let new_el = new_live.get(i).unwrap();
        let old_output = mounted_output(reconciler, parent, i);

        if can_skip_update(old_el, new_el) && !forced(reconciler, old_output) {
            reconciler.stats.elements_skipped += 1;
            continue;
        }

        let new_output = reconciler.update_output(old_el, new_el, old_output);
        if new_output != old_output {
            reconciler.replace_output_tracked(parent, i, new_output);
        }
    }

    for idx in (common..old_len).rev() {
        let output = mounted_output(reconciler, parent, idx);
        reconciler.unmount_output(output);
        reconciler.remove_output_tracked(parent, idx);
    }

    for i in common..new_len {
        let output = reconciler.mount_output(new_live.get(i).unwrap());
        if i == reconciler.tree.logical_children(parent).len() {
            reconciler.append_output_tracked(parent, output);
        } else {
            reconciler.insert_output_tracked(parent, i, output);
        }
    }
}

fn effective_key(el: &Element, positional_index: usize) -> Cow<'_, str> {
    if let Some(k) = el.key() {
        return Cow::Borrowed(k);
    }
    Cow::Owned(format!("__pos_{positional_index}_{}", el.kind_name()))
}

fn key_match(a: &Element, b: &Element) -> bool {
    a.can_update(b) && a.key() == b.key()
}

fn reconcile_keyed_live<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    old: &LiveChildrenRef<'_>,
    new: &LiveChildrenRef<'_>,
) {
    let old_len = old.len();
    let new_len = new.len();
    debug_assert_eq!(
        reconciler.tree.logical_children(parent).len(),
        old_len,
        "logical child mirror disagrees with old keyed child list"
    );

    let mut prefix = 0;
    while prefix < old_len
        && prefix < new_len
        && key_match(old.get(prefix).unwrap(), new.get(prefix).unwrap())
    {
        update_keyed_output(
            reconciler,
            parent,
            prefix,
            old.get(prefix).unwrap(),
            new.get(prefix).unwrap(),
        );
        prefix += 1;
    }

    let old_remaining = old_len - prefix;
    let new_remaining = new_len - prefix;
    let mut suffix = 0;
    while suffix < old_remaining
        && suffix < new_remaining
        && key_match(
            old.get(old_len - 1 - suffix).unwrap(),
            new.get(new_len - 1 - suffix).unwrap(),
        )
    {
        let logical_index = old_len - 1 - suffix;
        update_keyed_output(
            reconciler,
            parent,
            logical_index,
            old.get(logical_index).unwrap(),
            new.get(new_len - 1 - suffix).unwrap(),
        );
        suffix += 1;
    }

    let old_start = prefix;
    let old_mid_len = old_len - prefix - suffix;
    let new_start = prefix;
    let new_mid_len = new_len - prefix - suffix;
    if old_mid_len == 0 && new_mid_len == 0 {
        return;
    }

    let direct_native = (0..old_mid_len).all(|i| {
        old.get(old_start + i).unwrap().as_widget().is_some()
            && mounted_output(reconciler, parent, prefix + i)
                .logical
                .is_none()
    }) && (0..new_mid_len)
        .all(|i| new.get(new_start + i).unwrap().as_widget().is_some());
    if direct_native
        && reconcile_keyed_native_middle(
            reconciler,
            parent,
            old,
            new,
            old_start,
            old_mid_len,
            new_start,
            new_mid_len,
            prefix,
        )
    {
        return;
    }

    reconcile_keyed_middle(
        reconciler,
        parent,
        old,
        new,
        old_start,
        old_mid_len,
        new_start,
        new_mid_len,
        prefix,
    );
}

#[expect(clippy::too_many_arguments, clippy::needless_range_loop)]
fn reconcile_keyed_native_middle<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    old: &LiveChildrenRef<'_>,
    new: &LiveChildrenRef<'_>,
    old_start: usize,
    old_mid_len: usize,
    new_start: usize,
    new_mid_len: usize,
    prefix: usize,
) -> bool {
    if old_mid_len != new_mid_len {
        return false;
    }
    let mut old_key_map: FxHashMap<Cow<'_, str>, usize> = FxHashMap::default();
    old_key_map.reserve(old_mid_len);
    for i in 0..old_mid_len {
        old_key_map.insert(
            effective_key(old.get(old_start + i).unwrap(), old_start + i),
            i,
        );
    }

    let mut new_to_old = vec![-1; new_mid_len];
    let mut matched = vec![false; old_mid_len];
    for i in 0..new_mid_len {
        let new_el = new.get(new_start + i).unwrap();
        let key = effective_key(new_el, new_start + i);
        if let Some(&old_rel) = old_key_map.get(key.as_ref())
            && !matched[old_rel]
            && old.get(old_start + old_rel).unwrap().can_update(new_el)
        {
            new_to_old[i] = old_rel as i32;
            matched[old_rel] = true;
        }
    }
    if new_to_old.iter().any(|old| *old < 0) {
        return false;
    }

    let lis = compute_lis(&new_to_old);
    let mut old_controls = FxHashMap::default();
    old_controls.reserve(old_mid_len);
    for i in 0..old_mid_len {
        old_controls.insert(
            i,
            mounted_output(reconciler, parent, prefix + i)
                .native
                .unwrap(),
        );
    }
    let suffix_anchor = reconciler.tree.logical_children(parent)[prefix + old_mid_len..]
        .iter()
        .find_map(|output| output.native);
    let mut placed = vec![None; new_mid_len];

    for i in (0..new_mid_len).rev() {
        let new_el = new.get(new_start + i).unwrap();
        let anchor = placed
            .get(i + 1)
            .and_then(|control| *control)
            .or(suffix_anchor);
        let anchor_index = anchor
            .and_then(|control| live_index(reconciler, parent, control))
            .unwrap_or_else(|| live_len(reconciler, parent));

        let old_rel = new_to_old[i] as usize;
        let control = old_controls[&old_rel];
        if !lis.contains(&i) {
            let current = live_index(reconciler, parent, control).unwrap();
            let target = if current < anchor_index {
                anchor_index - 1
            } else {
                anchor_index
            };
            reconciler.move_child_tracked(parent, current, target);
        }
        let output = reconciler.update_output(
            old.get(old_start + old_rel).unwrap(),
            new_el,
            mounted_output(reconciler, parent, prefix + old_rel),
        );
        debug_assert_eq!(output.native, Some(control));
        placed[i] = Some(control);
    }

    reconciler
        .tree
        .permute_logical_children(parent, prefix, &new_to_old, &mut matched);
    true
}

fn update_keyed_output<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    index: usize,
    old: &Element,
    new: &Element,
) {
    let old_output = mounted_output(reconciler, parent, index);
    if can_skip_update(old, new) && !forced(reconciler, old_output) {
        reconciler.stats.elements_skipped += 1;
        return;
    }
    let new_output = reconciler.update_output(old, new, old_output);
    if new_output != old_output {
        reconciler.replace_output_tracked(parent, index, new_output);
    }
}

#[expect(clippy::too_many_arguments)]
fn reconcile_keyed_middle<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    old: &LiveChildrenRef<'_>,
    new: &LiveChildrenRef<'_>,
    old_start: usize,
    old_mid_len: usize,
    new_start: usize,
    new_mid_len: usize,
    prefix: usize,
) {
    let mut old_by_key: FxHashMap<Cow<'_, str>, (usize, LogicalSlotId)> = FxHashMap::default();
    for i in 0..old_mid_len {
        let old_index = old_start + i;
        old_by_key.insert(
            effective_key(old.get(old_index).unwrap(), old_index),
            (i, mounted_output(reconciler, parent, prefix + i).slot),
        );
    }

    let mut new_to_old = vec![-1; new_mid_len];
    let mut matched = vec![false; old_mid_len];
    for (i, old_index_slot) in new_to_old.iter_mut().enumerate() {
        let new_index = new_start + i;
        let key = effective_key(new.get(new_index).unwrap(), new_index);
        if let Some(&(old_index, _)) = old_by_key.get(key.as_ref())
            && !matched[old_index]
            && key_match(
                old.get(old_start + old_index).unwrap(),
                new.get(new_index).unwrap(),
            )
        {
            *old_index_slot = old_index as i32;
            matched[old_index] = true;
        }
    }

    let lis = compute_lis(&new_to_old);

    for i in (0..old_mid_len).rev() {
        if !matched[i] {
            let output = mounted_output(reconciler, parent, prefix + i);
            reconciler.unmount_output(output);
            reconciler.remove_output_tracked(parent, prefix + i);
        }
    }

    let matched_count = matched.iter().filter(|matched| **matched).count();
    let suffix_anchor = reconciler
        .tree
        .logical_child(parent, prefix + matched_count)
        .map(|output| output.slot);
    let mut placed = vec![None; new_mid_len];
    for i in (0..new_mid_len).rev() {
        let new_index = new_start + i;
        let anchor = placed
            .get(i + 1)
            .and_then(|output| *output)
            .or(suffix_anchor);

        let anchor_index = anchor
            .and_then(|slot| output_for_token(reconciler, parent, slot))
            .unwrap_or_else(|| reconciler.tree.logical_children(parent).len());

        if new_to_old[i] == -1 {
            let output = reconciler.mount_output(new.get(new_index).unwrap());
            reconciler.insert_output_tracked(parent, anchor_index, output);
            placed[i] = Some(output.slot);
            continue;
        }

        let old_slot = old_by_key
            .get(&effective_key(new.get(new_index).unwrap(), new_index))
            .map(|(_, slot)| *slot)
            .expect("matched keyed child has no mounted output");
        let Some(current) = output_for_token(reconciler, parent, old_slot) else {
            continue;
        };
        if !lis.contains(&i) && current != anchor_index {
            let target = if current < anchor_index {
                anchor_index - 1
            } else {
                anchor_index
            };
            reconciler.move_output_tracked(parent, current, target);
        }
        let current = output_for_token(reconciler, parent, old_slot).unwrap();
        let old_output = mounted_output(reconciler, parent, current);
        let updated = reconciler.update_output(
            old.get(old_start + new_to_old[i] as usize).unwrap(),
            new.get(new_index).unwrap(),
            old_output,
        );
        if updated != old_output {
            reconciler.replace_output_tracked(parent, current, updated);
        }
        placed[i] = Some(updated.slot);
    }
}

pub fn compute_lis(arr: &[i32]) -> FxHashSet<usize> {
    let n = arr.len();
    if n == 0 {
        return FxHashSet::default();
    }

    let mut tails: Vec<i32> = Vec::new();
    let mut tail_idx: Vec<usize> = Vec::new();
    let mut pred: Vec<i32> = vec![-1; n];

    for (i, &val) in arr.iter().enumerate() {
        if val == -1 {
            continue;
        }

        let mut lo = 0_usize;
        let mut hi = tails.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if tails[mid] < val {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        if lo == tails.len() {
            tails.push(val);
            tail_idx.push(i);
        } else {
            tails[lo] = val;
            tail_idx[lo] = i;
        }

        if lo > 0 {
            pred[i] = tail_idx[lo - 1] as i32;
        }
    }

    let mut result: FxHashSet<usize> = FxHashSet::default();
    if tail_idx.is_empty() {
        return result;
    }
    let mut idx = *tail_idx.last().unwrap() as i32;
    while idx != -1 {
        result.insert(idx as usize);
        idx = pred[idx as usize];
    }
    result
}

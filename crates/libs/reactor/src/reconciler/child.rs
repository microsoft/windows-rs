use rustc_hash::{FxHashMap, FxHashSet};
use std::borrow::Cow;

use super::*;

/// Update the child at `index` and return the control now occupying that slot.
fn update_child_tracked<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    index: usize,
    old: &Element,
    new: &Element,
) -> Option<ControlId> {
    let old_id = reconciler.child_at(parent, index)?;
    match reconciler.update(old, new, old_id) {
        Some(nid) if nid != old_id => {
            reconciler.replace_child_tracked(parent, index, nid);
            Some(nid)
        }
        Some(nid) => Some(nid),
        None => {
            reconciler.remove_child_tracked(parent, index);
            None
        }
    }
}

/// Current live index of `ctrl` in the authoritative children mirror.
fn live_index<B: Backend + 'static>(
    reconciler: &Reconciler<B>,
    parent: ControlId,
    ctrl: ControlId,
) -> Option<usize> {
    reconciler
        .children_mirror
        .get(&parent)
        .and_then(|v| v.iter().position(|&c| c == ctrl))
}

fn live_len<B: Backend + 'static>(reconciler: &Reconciler<B>, parent: ControlId) -> usize {
    reconciler
        .children_mirror
        .get(&parent)
        .map_or(0, |v| v.len())
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

    for i in 0..common {
        if reconciler.child_at(parent, i).is_none() {
            debug_assert!(
                false,
                "audit 7.1.10: positional reconcile out of mirror bounds at \
                 parent={parent:?} i={i} common={common} \
                 (mirror_len={mirror_len}, old_live.len={old_len}, \
                  new_live.len={new_len}). Likely a child-tracking miss \
                 in a recent edit to the reconciler.",
                mirror_len = reconciler
                    .children_mirror
                    .get(&parent)
                    .map_or(0, |v| v.len()),
                old_len = old_len,
                new_len = new_len,
            );
            break;
        }

        let old_el = old_live.get(i).unwrap();
        let new_el = new_live.get(i).unwrap();

        if !reconciler.force_component_rerender && can_skip_update(old_el, new_el) {
            // Dirty child state must pierce structural equality.
            let child_id = reconciler.child_at(parent, i);
            let state_dirty = child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
            if !state_dirty {
                reconciler.debug_elements_skipped += 1;
                continue;
            }
        }

        if old_el.kind_matches(new_el) {
            update_child_tracked(reconciler, parent, i, old_el, new_el);
        } else {
            if let Some(old_id) = reconciler.child_at(parent, i) {
                reconciler.unmount(old_id);
            }
            match reconciler.mount(new_el) {
                Some(new_id) => reconciler.replace_child_tracked(parent, i, new_id),
                None => reconciler.remove_child_tracked(parent, i),
            }
        }
    }

    if old_len > new_len {
        for idx in (common..old_len).rev() {
            if let Some(id) = reconciler.child_at(parent, idx) {
                reconciler.unmount(id);
            }
            reconciler.remove_child_tracked(parent, idx);
        }
    }

    if new_len > old_len {
        for i in common..new_len {
            let el = new_live.get(i).unwrap();
            if let Some(child_id) = reconciler.mount(el) {
                reconciler.append_child_tracked(parent, child_id);
            }
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
    if !a.can_update(b) {
        return false;
    }

    a.key() == b.key()
}

fn reconcile_keyed_live<B: Backend + 'static>(
    reconciler: &mut Reconciler<B>,
    parent: ControlId,
    old: &LiveChildrenRef<'_>,
    new: &LiveChildrenRef<'_>,
) {
    let old_len = old.len();
    let new_len = new.len();

    let mut prefix = 0;
    while prefix < old_len
        && prefix < new_len
        && key_match(old.get(prefix).unwrap(), new.get(prefix).unwrap())
    {
        let old_el = old.get(prefix).unwrap();
        let new_el = new.get(prefix).unwrap();
        if reconciler.child_at(parent, prefix).is_some() {
            if !reconciler.force_component_rerender && can_skip_update(old_el, new_el) {
                let child_id = reconciler.child_at(parent, prefix);
                let state_dirty =
                    child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
                if state_dirty {
                    update_child_tracked(reconciler, parent, prefix, old_el, new_el);
                } else {
                    reconciler.debug_elements_skipped += 1;
                }
            } else {
                update_child_tracked(reconciler, parent, prefix, old_el, new_el);
            }
        }
        prefix += 1;
    }

    debug_assert!(prefix <= old_len.min(new_len));
    let old_remaining = old_len.saturating_sub(prefix);
    let new_remaining = new_len.saturating_sub(prefix);

    let mut suffix = 0;
    while suffix < old_remaining
        && suffix < new_remaining
        && key_match(
            old.get(old_len - 1 - suffix).unwrap(),
            new.get(new_len - 1 - suffix).unwrap(),
        )
    {
        let old_idx = old_len - 1 - suffix;
        let panel_idx = reconciler
            .children_mirror
            .get(&parent)
            .map_or(0, |v| v.len().saturating_sub(1).saturating_sub(suffix));
        let old_el = old.get(old_idx).unwrap();
        let new_el = new.get(new_len - 1 - suffix).unwrap();
        if reconciler.child_at(parent, panel_idx).is_some() {
            if !reconciler.force_component_rerender && can_skip_update(old_el, new_el) {
                let child_id = reconciler.child_at(parent, panel_idx);
                let state_dirty =
                    child_id.is_some_and(|cid| reconciler.is_component_state_dirty(cid));
                if state_dirty {
                    update_child_tracked(reconciler, parent, panel_idx, old_el, new_el);
                } else {
                    reconciler.debug_elements_skipped += 1;
                }
            } else {
                update_child_tracked(reconciler, parent, panel_idx, old_el, new_el);
            }
        }
        suffix += 1;
    }

    let old_start = prefix;
    let old_end = old_len - suffix;
    let new_start = prefix;
    let new_end = new_len - suffix;
    let old_mid_len = old_end - old_start;
    let new_mid_len = new_end - new_start;

    if old_mid_len == 0 && new_mid_len == 0 {
        return;
    }

    if old_mid_len == 0 {
        for i in 0..new_mid_len {
            let new_el = new.get(new_start + i).unwrap();
            if let Some(ctrl) = reconciler.mount(new_el) {
                reconciler.insert_child_tracked(parent, prefix + i, ctrl);
            }
        }
        return;
    }

    if new_mid_len == 0 {
        for i in (0..old_mid_len).rev() {
            let panel_idx = prefix + i;
            if let Some(id) = reconciler.child_at(parent, panel_idx) {
                reconciler.unmount(id);
            }
            reconciler.remove_child_tracked(parent, panel_idx);
        }
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
    let mut old_key_map: FxHashMap<Cow<'_, str>, usize> = FxHashMap::default();
    old_key_map.reserve(old_mid_len);
    for i in 0..old_mid_len {
        let el = old.get(old_start + i).unwrap();
        let key = effective_key(el, old_start + i);
        old_key_map.insert(key, i);
    }

    let mut new_to_old: Vec<i32> = vec![-1; new_mid_len];
    let mut matched: Vec<bool> = vec![false; old_mid_len];
    for i in 0..new_mid_len {
        let el = new.get(new_start + i).unwrap();
        let key = effective_key(el, new_start + i);
        if let Some(&old_rel) = old_key_map.get(key.as_ref()) {
            let old_el = old.get(old_start + old_rel).unwrap();

            if old_el.can_update(el) {
                new_to_old[i] = old_rel as i32;
                matched[old_rel] = true;
            }
        }
    }

    let lis: FxHashSet<usize> = compute_lis(&new_to_old);

    for i in (0..old_mid_len).rev() {
        if !matched[i] {
            let panel_idx = prefix + i;
            if let Some(id) = reconciler.child_at(parent, panel_idx) {
                reconciler.unmount(id);
            }
            reconciler.remove_child_tracked(parent, panel_idx);
        }
    }

    // Matched controls survive compaction in old relative order before the suffix.
    let mut old_ctrl: FxHashMap<usize, ControlId> = FxHashMap::default();
    old_ctrl.reserve(old_mid_len);
    let mut matched_count = 0usize;
    {
        let mut panel_idx = prefix;
        for oi in 0..old_mid_len {
            if matched[oi] {
                if let Some(id) = reconciler.child_at(parent, panel_idx) {
                    old_ctrl.insert(oi, id);
                }
                panel_idx += 1;
                matched_count += 1;
            }
        }
    }
    // Stable right-edge anchor; the suffix never moves during middle reconciliation.
    let suffix_anchor = reconciler.child_at(parent, prefix + matched_count);

    // Walk right-to-left so each item is placed before an already-final successor.
    let mut placed: Vec<Option<ControlId>> = vec![None; new_mid_len];
    for i in (0..new_mid_len).rev() {
        let new_el = new.get(new_start + i).unwrap();

        let anchor = if i + 1 < new_mid_len {
            placed[i + 1]
        } else {
            suffix_anchor
        };
        let anchor_idx = match anchor {
            Some(a) => {
                live_index(reconciler, parent, a).unwrap_or_else(|| live_len(reconciler, parent))
            }
            None => live_len(reconciler, parent),
        };

        if new_to_old[i] == -1 {
            match reconciler.mount(new_el) {
                Some(ctrl) => {
                    reconciler.insert_child_tracked(parent, anchor_idx, ctrl);
                    placed[i] = Some(ctrl);
                }
                None => placed[i] = anchor,
            }
            continue;
        }

        let old_rel = new_to_old[i] as usize;
        let Some(&ctrl) = old_ctrl.get(&old_rel) else {
            continue;
        };
        let old_el = old.get(old_start + old_rel).unwrap();

        // Left-moving items shift the anchor during remove-then-insert.
        if !lis.contains(&i)
            && let Some(from) = live_index(reconciler, parent, ctrl)
        {
            let to = if from < anchor_idx {
                anchor_idx - 1
            } else {
                anchor_idx
            };
            reconciler.move_child_tracked(parent, from, to);
        }

        // Route through `update` so skip accounting, dirty-state forcing, and theme
        // re-resolution stay centralized.
        placed[i] = match live_index(reconciler, parent, ctrl) {
            Some(cur) => update_child_tracked(reconciler, parent, cur, old_el, new_el).or(anchor),
            None => anchor,
        };
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

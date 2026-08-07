//! Keyed reconciliation order and LIS move-optimality regression tests for #4716.

use std::cell::Cell;
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::{
    Component, ControlId, Element, KeyExt, Prop, PropValue, Reconciler, RenderCx, component,
    rich_edit_box, text_block, vstack,
};

fn keyed(items: &[&str]) -> Element {
    let children: Vec<Element> = items
        .iter()
        .map(|k| text_block(*k).with_key(*k).into())
        .collect();
    let mut s = vstack(children);
    s.key = Some("root".to_string());
    s.into()
}

struct Outcome {
    live: Vec<String>,
    moves: usize,
    creates: usize,
    destroys: usize,
    inserts: usize,
}

fn run(old_keys: &[&str], new_keys: &[&str]) -> Outcome {
    let mut r = Reconciler::new(RecordingBackend::new());
    let old = keyed(old_keys);
    let root = r
        .reconcile(None, &old, None, Rc::new(|| {}))
        .expect("mount");

    let base = r.backend.ops.len();
    let new = keyed(new_keys);
    r.reconcile(Some(&old), &new, Some(root), Rc::new(|| {}));

    // Skipped child updates retain the text recorded during mount.
    let text_of = |id: ControlId| -> String {
        r.backend
            .ops
            .iter()
            .rev()
            .find_map(|op| match op {
                Op::SetProp {
                    id: i,
                    prop: Prop::Text,
                    value: PropValue::Str(s),
                } if *i == id => Some(s.clone()),
                _ => None,
            })
            .expect("every live child has a Text op")
    };
    let live: Vec<String> = r
        .backend
        .children_of(root)
        .iter()
        .map(|&id| text_of(id))
        .collect();

    let tail = &r.backend.ops[base..];
    let count = |pred: fn(&Op) -> bool| tail.iter().filter(|o| pred(o)).count();
    Outcome {
        live,
        moves: count(|o| matches!(o, Op::MoveChild { .. })),
        creates: count(|o| matches!(o, Op::Create { .. })),
        destroys: count(|o| matches!(o, Op::Destroy { .. })),
        inserts: count(|o| matches!(o, Op::InsertChild { .. })),
    }
}

// Independent reference implementation prevents testing the reconciler against itself.
fn optimal_moves(old: &[&str], new: &[&str]) -> usize {
    let mut p = 0;
    while p < old.len() && p < new.len() && old[p] == new[p] {
        p += 1;
    }
    let mut s = 0;
    while s < old.len() - p && s < new.len() - p && old[old.len() - 1 - s] == new[new.len() - 1 - s]
    {
        s += 1;
    }
    let old_mid = &old[p..old.len() - s];
    let new_mid = &new[p..new.len() - s];

    let new_to_old: Vec<i32> = new_mid
        .iter()
        .map(|k| old_mid.iter().position(|o| o == k).map_or(-1, |i| i as i32))
        .collect();
    let matched = new_to_old.iter().filter(|&&v| v >= 0).count();
    matched - lis_len(&new_to_old)
}

fn lis_len(arr: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &v in arr {
        if v < 0 {
            continue;
        }
        match tails.binary_search(&v) {
            Ok(_) => {}
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(v);
                } else {
                    tails[pos] = v;
                }
            }
        }
    }
    tails.len()
}

fn assert_structural(old: &[&str], new: &[&str], o: &Outcome) {
    assert_eq!(
        o.live, new,
        "reconcile({old:?} -> {new:?}) produced unexpected live order"
    );

    let created: Vec<&&str> = new.iter().filter(|k| !old.contains(k)).collect();
    let removed: Vec<&&str> = old.iter().filter(|k| !new.contains(k)).collect();
    assert_eq!(
        o.creates,
        created.len(),
        "unexpected create count for {old:?} -> {new:?}"
    );
    assert_eq!(
        o.destroys,
        removed.len(),
        "unexpected destroy count for {old:?} -> {new:?}"
    );
    assert_eq!(
        o.inserts,
        created.len(),
        "every new key inserts exactly once for {old:?} -> {new:?}"
    );
    assert!(
        o.moves <= optimal_moves(old, new),
        "reconcile({old:?} -> {new:?}) used {} moves, exceeds LIS-optimal {}",
        o.moves,
        optimal_moves(old, new),
    );
}

#[test]
fn issue_4716_insert_plus_move_preserves_order() {
    // This previously produced ["B", "D", "C", "A"].
    let out = run(&["A", "B", "C"], &["B", "C", "D", "A"]);
    assert_eq!(out.live, ["B", "C", "D", "A"]);
    assert_eq!(out.creates, 1);
    assert_eq!(out.inserts, 1);
    assert_eq!(out.moves, 1, "only A needs to move; B,C form the LIS");
    assert_eq!(out.destroys, 0);
}

#[test]
fn insert_at_front_with_tail_rotation() {
    let out = run(&["a", "b", "c"], &["z", "b", "c", "a"]);
    assert_structural(&["a", "b", "c"], &["z", "b", "c", "a"], &out);
}

#[test]
fn interleave_new_between_moved_items() {
    let old = ["a", "b", "c", "d"];
    let new = ["d", "x", "a", "y", "b", "c"];
    let out = run(&old, &new);
    assert_structural(&old, &new, &out);
}

#[test]
fn reversal_with_two_inserts() {
    let old = ["a", "b", "c"];
    let new = ["p", "c", "b", "a", "q"];
    let out = run(&old, &new);
    assert_structural(&old, &new, &out);
}

#[test]
fn rotate_left_by_one_is_single_move() {
    let old = ["a", "b", "c", "d"];
    let new = ["b", "c", "d", "a"];
    let out = run(&old, &new);
    assert_structural(&old, &new, &out);
    assert_eq!(
        out.moves, 1,
        "rotate-by-one keeps b,c,d as LIS, moves only a"
    );
}

#[test]
fn full_replacement_destroys_and_creates() {
    let old = ["a", "b", "c"];
    let new = ["x", "y", "z"];
    let out = run(&old, &new);
    assert_structural(&old, &new, &out);
    assert_eq!(out.moves, 0);
    assert_eq!(out.creates, 3);
    assert_eq!(out.destroys, 3);
}

fn distinct_sequences(alphabet: &[&'static str], max_len: usize) -> Vec<Vec<&'static str>> {
    let mut out = vec![vec![]];
    let mut frontier: Vec<Vec<&'static str>> = vec![vec![]];
    for _ in 0..max_len {
        let mut next = vec![];
        for seq in &frontier {
            for &c in alphabet {
                if !seq.contains(&c) {
                    let mut s = seq.clone();
                    s.push(c);
                    out.push(s.clone());
                    next.push(s);
                }
            }
        }
        frontier = next;
    }
    out
}

#[test]
fn exhaustive_small_transitions_preserve_order_and_optimality() {
    let alphabet = ["a", "b", "c", "d", "e"];
    let seqs = distinct_sequences(&alphabet, 4);
    for old in &seqs {
        for new in &seqs {
            let out = run(old, new);
            assert_structural(old, new, &out);
        }
    }
}

// Heap's algorithm.
fn permutations(items: &[&'static str]) -> Vec<Vec<&'static str>> {
    let mut a = items.to_vec();
    let mut out = vec![];
    let n = a.len();
    let mut c = vec![0usize; n];
    out.push(a.clone());
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            out.push(a.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    out
}

#[test]
fn exhaustive_pure_reorderings_of_five() {
    let perms = permutations(&["a", "b", "c", "d", "e"]);
    for old in &perms {
        for new in &perms {
            let out = run(old, new);
            assert_eq!(out.creates, 0);
            assert_eq!(out.destroys, 0);
            assert_eq!(out.inserts, 0);
            assert_structural(old, new, &out);
        }
    }
}

fn text_of(r: &Reconciler<RecordingBackend>, id: ControlId) -> String {
    r.backend
        .ops
        .iter()
        .rev()
        .find_map(|op| match op {
            Op::SetProp {
                id: i,
                prop: Prop::Text,
                value: PropValue::Str(s),
            } if *i == id => Some(s.clone()),
            _ => None,
        })
        .expect("every live child has a Text op")
}

fn live_labels(r: &Reconciler<RecordingBackend>, root: ControlId) -> Vec<String> {
    r.backend
        .children_of(root)
        .iter()
        .map(|&id| text_of(r, id))
        .collect()
}

#[derive(Clone, PartialEq, Debug)]
struct KindProps {
    label: String,
    alt: bool,
}

struct KindRow;

impl Component<KindProps> for KindRow {
    fn render(&self, p: &KindProps, _cx: &mut RenderCx) -> Element {
        if p.alt {
            rich_edit_box(p.label.clone()).into()
        } else {
            text_block(p.label.clone()).into()
        }
    }
}

fn kind_stack(items: &[(&str, bool)]) -> Element {
    let children: Vec<Element> = items
        .iter()
        .map(|(k, alt)| {
            component(
                KindRow,
                KindProps {
                    label: (*k).to_string(),
                    alt: *alt,
                },
            )
            .with_key(*k)
        })
        .collect();
    let mut s = vstack(children);
    s.key = Some("root".to_string());
    s.into()
}

#[test]
fn component_root_kind_change_during_reorder_preserves_order() {
    // A remounted component must be anchored by its new `ControlId`.
    let mut r = Reconciler::new(RecordingBackend::new());
    let old = kind_stack(&[("a", false), ("b", false), ("c", false), ("d", false)]);
    let root = r
        .reconcile(None, &old, None, Rc::new(|| {}))
        .expect("mount");

    let new = kind_stack(&[("d", false), ("a", true), ("b", false), ("c", false)]);
    r.reconcile(Some(&old), &new, Some(root), Rc::new(|| {}));

    assert_eq!(live_labels(&r, root), ["d", "a", "b", "c"]);
}

#[derive(Clone, PartialEq, Debug)]
struct CountProps {
    label: String,
}

struct CountRow {
    renders: Rc<Cell<u32>>,
}

impl Component<CountProps> for CountRow {
    fn render(&self, p: &CountProps, _cx: &mut RenderCx) -> Element {
        self.renders.set(self.renders.get() + 1);
        text_block(p.label.clone()).into()
    }
}

fn count_stack(target: &Rc<Cell<u32>>, items: &[&str]) -> Element {
    let children: Vec<Element> = items
        .iter()
        .map(|k| {
            let renders = if *k == "t" {
                Rc::clone(target)
            } else {
                Rc::new(Cell::new(0))
            };
            component(
                CountRow { renders },
                CountProps {
                    label: (*k).to_string(),
                },
            )
            .with_key(*k)
        })
        .collect();
    let mut s = vstack(children);
    s.key = Some("root".to_string());
    s.into()
}

#[test]
fn forced_rerender_reaches_reordered_middle_component() {
    {
        let renders = Rc::new(Cell::new(0));
        let mut r = Reconciler::new(RecordingBackend::new());
        let old = count_stack(&renders, &["t", "x", "y"]);
        let root = r
            .reconcile(None, &old, None, Rc::new(|| {}))
            .expect("mount");

        renders.set(0);
        let new = count_stack(&renders, &["x", "y", "t"]);
        r.reconcile(Some(&old), &new, Some(root), Rc::new(|| {}));
        assert_eq!(
            renders.get(),
            0,
            "an unchanged component must be skipped when no re-render is forced"
        );
    }

    {
        let renders = Rc::new(Cell::new(0));
        let mut r = Reconciler::new(RecordingBackend::new());
        let old = count_stack(&renders, &["t", "x", "y"]);
        let root = r
            .reconcile(None, &old, None, Rc::new(|| {}))
            .expect("mount");

        let target_id = r.backend.children_of(root)[0];
        renders.set(0);
        r.force_components_at_control_for_test(target_id);

        let new = count_stack(&renders, &["x", "y", "t"]);
        r.reconcile(Some(&old), &new, Some(root), Rc::new(|| {}));
        assert_eq!(
            renders.get(),
            1,
            "a forced component must re-render even when reordered through the middle region"
        );
    }
}

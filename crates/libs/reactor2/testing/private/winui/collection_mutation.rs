use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::Duration;

use super::*;
use crate::winui::collection::tests as collection_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_COLLECTION_MUTATION_FIXTURE";

fn run_case(case: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::collection_mutation::collection_mutation_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn list_reorder_preserves_keyed_row_state() {
    run_case("list");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn grid_reorder_preserves_keyed_row_state_and_configuration() {
    run_case("grid");
}

#[test]
fn collection_mutation_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };

    bootstrap().unwrap();
    mutation_fixture(match case.to_str().unwrap() {
        "list" => false,
        "grid" => true,
        case => panic!("unknown collection mutation fixture: {case}"),
    });
}

fn mutation_fixture(grid: bool) {
    let state = Rc::new(RefCell::new(None::<State<VirtualItemKeys>>));
    let tokens = Rc::new(RefCell::new(BTreeMap::new()));
    let cleanups = Rc::new(RefCell::new(Vec::new()));
    let sequence = Rc::new(Cell::new(1usize));
    let state_for_render = Rc::clone(&state);
    let tokens_for_render = Rc::clone(&tokens);
    let cleanups_for_render = Rc::clone(&cleanups);
    let sequence_for_render = Rc::clone(&sequence);
    let content = component(move |cx| {
        let keys = cx.use_state(|| VirtualItemKeys::new([1_000, 1_001, 1_002, 1_003, 1_004]));
        *state_for_render.borrow_mut() = Some(keys.clone());
        let keys = keys.try_value().unwrap();
        let row_keys = keys.clone();
        let tokens = Rc::clone(&tokens_for_render);
        let cleanups = Rc::clone(&cleanups_for_render);
        let sequence = Rc::clone(&sequence_for_render);
        let row = move |index| {
            let key = row_keys.as_slice()[index];
            let tokens = Rc::clone(&tokens);
            let cleanups = Rc::clone(&cleanups);
            let sequence = Rc::clone(&sequence);
            component(move |cx| {
                let token = cx.use_state(|| {
                    let token = sequence.get();
                    sequence.set(token + 1);
                    token
                });
                tokens.borrow_mut().insert(key, token.try_value().unwrap());
                let cleanups = Rc::clone(&cleanups);
                cx.use_effect_with_cleanup((), move || move || cleanups.borrow_mut().push(key));
                text_block(format!("{key}:{}", token.try_value().unwrap()))
            })
        };
        if grid {
            VirtualGrid::new(keys.len(), 300.0, row)
                .item_keys(keys)
                .selection_mode(SelectionMode::Multiple)
                .selection(CollectionSelection::new([1_001]), |_| {})
                .empty_state(text_block("empty grid"))
                .automation_name("Native grid")
                .help_text("Virtual tile results")
                .on_item_invoked(|_| {})
                .build()
        } else {
            VirtualList::new(keys.len(), 300.0, row)
                .item_keys(keys)
                .reorderable(|_| {})
                .build()
        }
    });
    let title = if grid {
        "windows-reactor GridView mutation fixture"
    } else {
        "windows-reactor ListView mutation fixture"
    };
    let root = Application::new([Window::new(title, content, || {}).build()]).build();
    let outcome = Rc::new(RefCell::new(None::<Result<(), String>>));
    let finish_outcome = Rc::clone(&outcome);

    run_app_fixture(root, move |reactor| {
        if grid {
            let runtime = reactor.engine().runtime();
            let grids = collection_probe::collections(runtime, true);
            let lists = collection_probe::collections(runtime, false);
            assert_eq!(grids.len(), 1);
            assert!(lists.is_empty());
            let grid = grids[0];
            assert_eq!(collection_probe::item_count(runtime, grid)?, 5);
            assert_eq!(
                collection_probe::item_keys(runtime, grid)?,
                vec![1_000, 1_001, 1_002, 1_003, 1_004]
            );
            assert_eq!(
                collection_probe::selection_mode(runtime, grid)?,
                SelectionMode::Multiple
            );
            assert_eq!(
                collection_probe::selection(runtime, grid)?,
                CollectionSelection::new([1_001])
            );
            assert!(collection_probe::item_click_enabled(runtime, grid)?);
        } else {
            let runtime = reactor.engine().runtime();
            let lists = collection_probe::collections(runtime, false);
            assert_eq!(lists.len(), 1);
            assert!(collection_probe::can_reorder_items(runtime, lists[0])?);
        }

        let first = Rc::new(RefCell::new(None::<TestTimer>));
        let first_for_tick = Rc::clone(&first);
        let state_for_tick = state.borrow().as_ref().unwrap().clone();
        let tokens_for_tick = Rc::clone(&tokens);
        let cleanups_for_tick = Rc::clone(&cleanups);
        let outcome_for_tick = Rc::clone(&finish_outcome);
        *first.borrow_mut() = Some(TestTimer::one_shot(
            Duration::from_millis(100),
            move || {
                first_for_tick.borrow_mut().take();
                let before = tokens_for_tick.borrow().clone();
                if before.is_empty() {
                    *outcome_for_tick.borrow_mut() =
                        Some(Err("no keyed collection rows were realized".to_string()));
                    terminate_host();
                    return;
                }
                state_for_tick.set(VirtualItemKeys::new([2_000, 1_004, 1_003, 1_002, 1_001]));

                let second = Rc::new(RefCell::new(None::<TestTimer>));
                let second_for_tick = Rc::clone(&second);
                let tokens_for_second = Rc::clone(&tokens_for_tick);
                let cleanups_for_second = Rc::clone(&cleanups_for_tick);
                let outcome_for_second = Rc::clone(&outcome_for_tick);
                match TestTimer::one_shot(Duration::from_millis(100), move || {
                    second_for_tick.borrow_mut().take();
                    let after = tokens_for_second.borrow();
                    let result =
                        check_reordered_rows(&before, &after, &cleanups_for_second.borrow());
                    *outcome_for_second.borrow_mut() = Some(result);
                    terminate_host();
                }) {
                    Ok(timer) => *second.borrow_mut() = Some(timer),
                    Err(error) => {
                        *outcome_for_tick.borrow_mut() = Some(Err(error.to_string()));
                        terminate_host();
                    }
                }
            },
        )?);
        Ok(())
    })
    .unwrap();

    if let Err(error) = outcome.borrow_mut().take().unwrap() {
        panic!("{error}");
    }
}

fn check_reordered_rows(
    before: &BTreeMap<u64, usize>,
    after: &BTreeMap<u64, usize>,
    cleanups: &[u64],
) -> Result<(), String> {
    for key in [1_001, 1_002, 1_003, 1_004] {
        if let Some(before) = before.get(&key)
            && after.get(&key) != Some(before)
        {
            return Err(format!("row state changed for key {key}"));
        }
    }
    if before.contains_key(&1_000) && !cleanups.contains(&1_000) {
        return Err("removed row state was not cleaned up".to_string());
    }
    if let (Some(inserted), Some(removed)) = (after.get(&2_000), before.get(&1_000))
        && inserted == removed
    {
        return Err("inserted row reused removed row state".to_string());
    }
    Ok(())
}

use super::*;
use Threading::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Barrier, Mutex};

const THREADS: usize = 8;
const CALLS: usize = 1000;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    // The widget is created on this thread but called directly from threads in other apartments.
    for apartment in [COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED] {
        let (thread, current) = std::thread::scope(|scope| {
            scope
                .spawn(|| unsafe {
                    CoInitializeEx(None, apartment as u32).ok().unwrap();
                    let result = (GetCurrentThreadId(), widget.CurrentThread().unwrap());
                    CoUninitialize();
                    result
                })
                .join()
                .unwrap()
        });
        assert_eq!(thread, current);
    }

    // Threads start together so calls overlap, and every call gets a unique count.
    let barrier = Barrier::new(THREADS);
    let counts: Vec<i32> = std::thread::scope(|scope| {
        let threads: Vec<_> = (0..THREADS)
            .map(|_| {
                scope.spawn(|| {
                    barrier.wait();
                    (0..CALLS)
                        .map(|_| widget.Increment().unwrap())
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        threads
            .into_iter()
            .flat_map(|thread| thread.join().unwrap())
            .collect()
    });
    let total = (THREADS * CALLS) as i32;
    assert_eq!(
        counts.iter().copied().collect::<HashSet<_>>(),
        (1..=total).collect()
    );
    assert_eq!(widget.Count()?, total);

    // The component calls handlers on background threads.
    let sum = Arc::new(AtomicI32::new(0));
    let threads = Arc::new(Mutex::new(HashSet::new()));
    let handler = Handler::new({
        let sum = sum.clone();
        let threads = threads.clone();
        move |value| {
            sum.fetch_add(value, Ordering::Relaxed);
            threads
                .lock()
                .unwrap()
                .insert(unsafe { GetCurrentThreadId() });
            Ok(())
        }
    });
    let actions = (1..=THREADS as i32)
        .map(|value| widget.InvokeOnBackground(&handler, value))
        .collect::<Result<Vec<_>>>()?;
    for action in actions {
        action.join()?;
    }
    assert_eq!(sum.load(Ordering::Relaxed), 36);
    assert!(
        !threads
            .lock()
            .unwrap()
            .contains(&unsafe { GetCurrentThreadId() })
    );

    // A handler's error fails the action.
    let failing = Handler::new(|_| Err(E_BOUNDS.into()));
    assert_eq!(
        widget
            .InvokeOnBackground(&failing, 0)?
            .join()
            .map_err(|e| e.code()),
        Err(E_BOUNDS)
    );

    Ok(())
}

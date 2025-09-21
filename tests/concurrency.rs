//! Id generator on concurrt Environment

use snowflake::ProcessUniqueId;
use sonyflake::Sonyflake;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use ulid::Ulid;
use uuid::Uuid;

const THREADS: usize = 10;
const IDS_PER_THREAD: usize = 10_000;

// TODO: Enhance test thread-safe condition

#[test]
fn test_concurrency_snowflake_uniqueness() {
    let all_ids = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = vec![];

    for thread_id in 0..THREADS {
        let ids = Arc::clone(&all_ids);

        let handle = thread::spawn(move || {
            let mut local_ids = Vec::with_capacity(IDS_PER_THREAD);

            for _ in 0..IDS_PER_THREAD {
                local_ids.push(ProcessUniqueId::new());
            }

            let mut global = ids.lock().unwrap();
            for (i, id) in local_ids.into_iter().enumerate() {
                assert!(
                    global.insert(id),
                    "rs-snowflake: Thread {} duplicate at iteration {}: {}",
                    thread_id,
                    i,
                    id
                );
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total = all_ids.lock().unwrap().len();
    assert_eq!(
        total,
        THREADS * IDS_PER_THREAD,
        "snowflake: Expected {} unique IDs, got {}",
        THREADS * IDS_PER_THREAD,
        total
    );
}

#[test]
fn test_concurrency_sonyflake_uniqueness() {
    let generator = Arc::new(Mutex::new(Sonyflake::builder().finalize().unwrap()));
    let all_ids = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = vec![];

    for thread_id in 0..THREADS {
        let g = Arc::clone(&generator);
        let ids = Arc::clone(&all_ids);

        let handle = thread::spawn(move || {
            let mut local_ids = Vec::with_capacity(IDS_PER_THREAD);

            for _ in 0..IDS_PER_THREAD {
                let id = g.lock().unwrap().next_id().unwrap();
                local_ids.push(id);
            }

            let mut global = ids.lock().unwrap();
            for (i, id) in local_ids.into_iter().enumerate() {
                assert!(
                    global.insert(id),
                    "sonyflake: Thread {} duplicate at iteration {}: {}",
                    thread_id,
                    i,
                    id
                );
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total = all_ids.lock().unwrap().len();
    assert_eq!(
        total,
        THREADS * IDS_PER_THREAD,
        "sonyflake: Expected {} unique IDs, got {}",
        THREADS * IDS_PER_THREAD,
        total
    );
}

#[test]
fn test_concurrency_uuid_v7_uniqueness() {
    let all_ids = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = vec![];

    for thread_id in 0..THREADS {
        let ids = Arc::clone(&all_ids);

        let handle = thread::spawn(move || {
            let mut local_ids = Vec::with_capacity(IDS_PER_THREAD);

            for _ in 0..IDS_PER_THREAD {
                local_ids.push(Uuid::now_v7());
            }

            let mut global = ids.lock().unwrap();
            for (i, id) in local_ids.into_iter().enumerate() {
                assert!(
                    global.insert(id),
                    "UUID v7: Thread {} duplicate at iteration {}: {}",
                    thread_id,
                    i,
                    id
                );
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(all_ids.lock().unwrap().len(), THREADS * IDS_PER_THREAD);
}

#[test]
fn test_concurrency_ulid_uniqueness() {
    let all_ids = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = vec![];

    for thread_id in 0..THREADS {
        let ids = Arc::clone(&all_ids);

        let handle = thread::spawn(move || {
            let mut local_ids = Vec::with_capacity(IDS_PER_THREAD);

            for _ in 0..IDS_PER_THREAD {
                local_ids.push(Ulid::new());
            }

            let mut global = ids.lock().unwrap();
            for (i, id) in local_ids.into_iter().enumerate() {
                assert!(
                    global.insert(id),
                    "ULID: Thread {} duplicate at iteration {}: {}",
                    thread_id,
                    i,
                    id
                );
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(all_ids.lock().unwrap().len(), THREADS * IDS_PER_THREAD);
}

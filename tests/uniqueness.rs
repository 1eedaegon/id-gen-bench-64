///! Uniqueness

#[cfg(test)]
mod uniqueness_tests {
    use snowflake::ProcessUniqueId;
    use sonyflake::{self, Sonyflake};
    use std::collections::HashSet;
    use ulid::Ulid;
    use uuid::Uuid;

    // WARN: is not bench or stress testing
    const TEST_COUNT: usize = 1_000_000;

    #[test]
    fn test_uniqueness_snowflake() {
        let mut ids = HashSet::with_capacity(TEST_COUNT);

        for i in 0..TEST_COUNT {
            let id = ProcessUniqueId::new();
            assert!(
                ids.insert(id),
                "rs-snowflake: Duplicate ID at iteration {}: {}",
                i,
                id
            );
        }
    }

    #[test]
    fn test_uniqueness_sonyflake() {
        let generator = Sonyflake::builder().finalize().unwrap();
        let mut ids = HashSet::with_capacity(TEST_COUNT);

        for i in 0..TEST_COUNT {
            let id = generator.next_id().unwrap();
            assert!(
                ids.insert(id),
                "sonyflake: Duplicate ID at iteration {}: {}",
                i,
                id
            );
        }
    }

    #[test]
    fn test_uniqueness_ulid() {
        let mut ids = HashSet::with_capacity(TEST_COUNT);

        for i in 0..TEST_COUNT {
            let id = Ulid::new();
            assert!(
                ids.insert(id),
                "ulid: Duplicate ID at iteration {}: {}",
                i,
                id
            );
        }
    }

    #[test]
    fn test_uniqueness_uuid() {
        let mut ids = HashSet::with_capacity(TEST_COUNT);

        for i in 0..TEST_COUNT {
            let id = Uuid::new_v4();
            assert!(
                ids.insert(id),
                "uuid: Duplicate ID at iteration {}: {}",
                i,
                id
            );
        }
    }
}

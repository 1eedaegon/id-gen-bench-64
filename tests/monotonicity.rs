///! Monocity

#[cfg(test)]
mod test_monotonicity {
    use snowflake::ProcessUniqueId;
    use sonyflake::{self, Sonyflake};
    use ulid::Ulid;

    #[test]
    fn test_monotonicity_snowflake() {
        let mut prev = ProcessUniqueId::new();

        for i in 0..10_000 {
            let current = ProcessUniqueId::new();
            assert!(
                current >= prev,
                "rs-snowflake: Non-monotonic at iteration {}: {} < {}",
                i,
                current,
                prev
            );
            prev = current;
        }
    }

    #[test]
    fn test_monotonicity_sonyflake() {
        let generator = Sonyflake::new().unwrap();
        let mut prev = generator.next_id().unwrap();

        for i in 0..10_000 {
            // On loop
            let current = generator.next_id().unwrap();
            assert!(
                current >= prev,
                "sonyflake: Non-monotonic at iteration {}: {} < {}",
                i,
                current,
                prev
            );
            prev = current;
        }
    }

    // TODO: UUID is not monotonic
    #[test]
    fn test_monotonicity_uuid() {
        let mut prev = uuid::Uuid::new_v4();

        for i in 0..10_000 {
            // On loop
            let current = uuid::Uuid::new_v4();
            assert!(
                current >= prev,
                "UUID: Non-monotonic at iteration {}: {} < {}",
                i,
                current,
                prev
            );
            prev = current;
        }
    }

    #[test]
    fn test_monotonicity_ulid() {
        let mut prev = Ulid::new();

        for i in 0..10_000 {
            let current = Ulid::new();
            assert!(
                current >= prev,
                "ULID: Non-monotonic at iteration {}: {:?} < {:?}",
                i,
                current,
                prev
            );
            prev = current;
        }
    }
}

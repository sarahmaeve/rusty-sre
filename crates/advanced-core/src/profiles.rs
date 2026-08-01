pub fn next_generation(current: u32) -> Option<u32> {
    debug_assert!(current < u32::MAX, "generation exhausted");
    Some(current + 1)
}

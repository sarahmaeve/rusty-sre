macro_rules! cap_sample {
    ($sample:expr, $limit:expr) => {
        if $sample < $limit { $sample } else { $limit }
    };
}

pub fn sample_with_cap(mut sample: impl FnMut() -> u64, limit: u64) -> u64 {
    cap_sample!(sample(), limit)
}

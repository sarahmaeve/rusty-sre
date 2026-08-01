//! Collections own groups of values. Iterators describe lazy traversal and
//! transformation. Closures may borrow, mutate, or take values from their scope.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch08-00-common-collections.html>
//! - <https://doc.rust-lang.org/book/ch13-00-functional-features.html>
//! - <https://doc.rust-lang.org/std/iter/trait.Iterator.html>
//! - <https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html>
//! - Source study: <https://github.com/rust-lang/rust-clippy/tree/master/clippy_lints/src>

use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct Sample {
    service: String,
    latency_ms: u64,
}

fn aggregate(samples: &[Sample]) -> BTreeMap<&str, (u64, usize)> {
    let mut totals = BTreeMap::new();
    for sample in samples {
        let (sum, count) = totals.entry(sample.service.as_str()).or_insert((0, 0));
        *sum += sample.latency_ms;
        *count += 1;
    }
    totals
}

fn main() {
    let samples = vec![
        Sample {
            service: "api".to_owned(),
            latency_ms: 120,
        },
        Sample {
            service: "worker".to_owned(),
            latency_ms: 80,
        },
        Sample {
            service: "api".to_owned(),
            latency_ms: 180,
        },
    ];

    let totals = aggregate(&samples);
    assert_eq!(totals["api"], (300, 2));
    assert_eq!(totals["worker"], (80, 1));

    // Iterator adapters are lazy until a consumer such as `collect` or `sum`.
    let slow_services: Vec<_> = samples
        .iter() // Iterator<Item = &Sample>; borrows the vector.
        .filter(|sample| sample.latency_ms >= 100)
        .map(|sample| sample.service.as_str())
        .collect();
    assert_eq!(slow_services, ["api", "api"]);

    let total_latency: u64 = samples.iter().map(|sample| sample.latency_ms).sum();
    assert_eq!(total_latency, 380);

    let mut calls = 0;
    {
        let mut above = |limit| {
            calls += 1; // Mutable capture makes this closure `FnMut`.
            samples
                .iter()
                .filter(|sample| sample.latency_ms > limit)
                .count()
        };
        assert_eq!(above(100), 2);
        assert_eq!(above(150), 1);
    } // End the mutable borrow of `calls`.
    assert_eq!(calls, 2);

    let labels = ["api".to_owned(), "worker".to_owned()];
    let owns_labels = move || labels.join(",");
    assert_eq!(owns_labels(), "api,worker");

    let mut attempts = HashMap::new();
    for service in ["api", "api", "worker"] {
        *attempts.entry(service).or_insert(0) += 1;
    }
    assert_eq!(attempts.get("api"), Some(&2));

    let owned_names: Vec<String> = samples
        .into_iter() // Iterator<Item = Sample>; consumes the vector.
        .map(|sample| sample.service)
        .collect();
    assert_eq!(owned_names, ["api", "worker", "api"]);
}

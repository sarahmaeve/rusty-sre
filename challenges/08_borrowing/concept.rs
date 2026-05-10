// =============================================================================
// Challenge 01a: Ownership & Borrowing with Vectors — Concept Explainer
// =============================================================================
//
// Vectors are where most Rust learners first encounter the borrow checker in
// a meaningful way. This file covers how ownership, borrowing, and lifetimes
// interact with Vec<T> — and why the compiler stops you from doing things
// that seem reasonable in other languages.
//
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. OWNERSHIP: Moving Vectors
// =============================================================================
// A Vec<T> owns its heap-allocated data. When you assign a Vec to another
// variable or pass it to a function, the ownership MOVES. The original
// variable is no longer usable.
//
// This is different from languages like Python or Java where assignment
// creates a second reference to the same data. In Rust, there is always
// exactly one owner.

#[test]
fn ownership_moves() {
    let servers = vec!["web-1", "web-2", "web-3"];

    // This MOVES ownership from `servers` into `backup_list`.
    let backup_list = servers;

    // `servers` is now invalid — the compiler will reject any use of it:
    //   println!("{:?}", servers);  // ERROR: value used after move
    // But `backup_list` works fine:
    assert_eq!(backup_list.len(), 3);

    // The same applies to function calls. Passing a Vec by value moves it:
    fn count_items(items: Vec<&str>) -> usize {
        items.len()
    }

    let names = vec!["alice", "bob"];
    let n = count_items(names);
    assert_eq!(n, 2);
    // `names` is now invalid — it was moved into count_items().
    // This is why most functions borrow (&) instead of taking ownership.
}

// =============================================================================
// 2. CLONE: Explicit Deep Copy
// =============================================================================
// If you actually need two independent copies of a Vec, call .clone().
// This performs a deep copy — a new heap allocation with duplicated data.
// It's explicit so you always know when you're paying for a copy.

#[test]
fn explicit_cloning() {
    let primary = vec![1, 2, 3];

    // .clone() creates a full independent copy
    let replica = primary.clone();

    // Both are usable and independent
    assert_eq!(primary, replica);
    assert_eq!(primary.len(), 3);

    // Mutating one doesn't affect the other
    let mut replica = replica;
    replica.push(4);
    assert_eq!(primary.len(), 3); // unchanged
    assert_eq!(replica.len(), 4);

    // Clone is O(n) — be mindful with large vectors in hot paths.
    // Often, borrowing (&) is what you actually want instead of cloning.
}

// =============================================================================
// 3. BORROWING: References to Vectors
// =============================================================================
// Instead of moving a Vec into a function, you can lend it via a reference.
//
//   &Vec<T>      — shared (immutable) borrow. Can read, cannot modify.
//   &mut Vec<T>  — exclusive (mutable) borrow. Can read AND modify.
//
// The fundamental rule: you can have EITHER
//   - Any number of &Vec<T> at the same time, OR
//   - Exactly one &mut Vec<T> (and no & borrows while it exists)
// Never both. This prevents data races at compile time.

#[test]
fn shared_borrows() {
    let metrics = vec![100, 200, 300];

    // Multiple shared borrows are fine simultaneously
    let r1 = &metrics;
    let r2 = &metrics;

    // Both references and the owner can read
    assert_eq!(r1.len(), 3);
    assert_eq!(r2[0], 100);
    assert_eq!(metrics[2], 300);
}

#[test]
fn mutable_borrows() {
    let mut queue = vec!["task-a", "task-b"];

    // A mutable borrow grants exclusive access
    let q_ref = &mut queue;
    q_ref.push("task-c");
    q_ref.push("task-d");
    assert_eq!(q_ref.len(), 4);

    // While q_ref is active, we CANNOT use `queue` directly:
    //   queue.push("x");  // ERROR: cannot borrow `queue` as mutable more than once
    //   println!("{}", queue.len());  // ERROR: cannot borrow `queue` as immutable

    // After q_ref's last use, the borrow ends and `queue` is accessible again.
    // (Rust uses "Non-Lexical Lifetimes" — the borrow ends at last use, not
    // at the end of the scope.)
    assert_eq!(queue.len(), 4);
    assert_eq!(queue[3], "task-d");
}

#[test]
fn borrowing_in_functions() {
    // GOOD: borrow with &[T] (a slice) — the most flexible signature.
    // Accepts &Vec<T>, &[T], and fixed-size arrays.
    fn average(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }

    let data = vec![10.0, 20.0, 30.0];
    let avg = average(&data); // borrows, does not move
    assert_eq!(avg, 20.0);
    assert_eq!(data.len(), 3); // `data` still usable

    // GOOD: mutable borrow with &mut Vec<T> when you need to modify
    fn add_default(entries: &mut Vec<f64>, default: f64) {
        if entries.is_empty() {
            entries.push(default);
        }
    }

    let mut data2 = vec![];
    add_default(&mut data2, 42.0);
    assert_eq!(data2, vec![42.0]);
}

// =============================================================================
// 4. THE INVALIDATION PROBLEM: References + Mutation
// =============================================================================
// The most common borrow checker surprise with vectors: you cannot hold a
// reference into a Vec and then modify the Vec. Why? Because push() might
// reallocate the internal buffer, leaving your reference dangling.
//
// In C/C++, this is a real bug (use-after-free / iterator invalidation).
// Rust catches it at compile time.

#[test]
fn invalidation_explained() {
    // Scenario: you want to remember the first element, then push more.
    let mut values = vec![10, 20, 30];

    // BAD (won't compile):
    //   let first = &values[0];   // immutable borrow of `values`
    //   values.push(40);          // mutable borrow of `values` — CONFLICT
    //   println!("{first}");      // tries to use the immutable borrow

    // WHY: push() may reallocate the Vec's buffer. If it does, `first`
    // would point to freed memory. Rust prevents this at compile time.

    // FIX 1: Copy the value out (works for Copy types like i32)
    let first = values[0]; // copies the i32, no borrow held
    values.push(40);
    assert_eq!(first, 10);
    assert_eq!(values.len(), 4);

    // FIX 2: Scope the borrow so it ends before the mutation
    let first_val = {
        let r = &values[0];
        *r // copy out before the borrow scope ends
    };
    values.push(50);
    assert_eq!(first_val, 10);

    // FIX 3: Do all your reading first, then all your writing
    let sum: i32 = values.iter().sum(); // borrow ends after this line
    values.push(60);
    assert_eq!(sum, 10 + 20 + 30 + 40 + 50);
}

// =============================================================================
// 5. ITERATING AND MUTATING: THE CLASSIC TRAP
// =============================================================================
// You cannot iterate over a Vec with iter() and simultaneously push/remove
// elements. The iterator borrows the Vec immutably, blocking mutation.
//
// This section shows common patterns to work around this.

#[test]
fn collect_then_mutate() {
    let mut log_levels = vec!["INFO", "DEBUG", "ERROR", "DEBUG", "WARN", "DEBUG"];

    // BAD (won't compile):
    //   for (i, level) in log_levels.iter().enumerate() {
    //       if *level == "DEBUG" {
    //           log_levels.remove(i);  // can't mutate while iterating!
    //       }
    //   }

    // FIX: Use retain() — the idiomatic Rust way to filter in-place
    log_levels.retain(|&level| level != "DEBUG");
    assert_eq!(log_levels, vec!["INFO", "ERROR", "WARN"]);
}

#[test]
fn build_new_vec_from_old() {
    let readings = vec![5, 12, 3, 18, 7, 25, 1];

    // Instead of removing from the original, build a new filtered Vec.
    // This avoids any borrow conflicts.
    let high_readings: Vec<i32> = readings.iter().copied().filter(|&x| x > 10).collect();

    assert_eq!(high_readings, vec![12, 18, 25]);
    assert_eq!(readings.len(), 7); // original unchanged
}

#[test]
fn index_collection_pattern() {
    // Sometimes you need to process elements based on other elements.
    // Collect indices or values first, then mutate in a second pass.

    let mut scores = vec![85, 42, 91, 38, 77, 95];

    // Step 1: Find indices of failing scores (< 50), no mutation yet
    let failing_indices: Vec<usize> = scores
        .iter()
        .enumerate()
        .filter(|(_, score)| **score < 50)
        .map(|(i, _)| i)
        .collect(); // borrow of `scores` ends here

    // Step 2: Now mutate — set failing scores to 50 (grade floor)
    for &i in &failing_indices {
        scores[i] = 50;
    }

    assert_eq!(scores, vec![85, 50, 91, 50, 77, 95]);
}

#[test]
fn iter_mut_for_in_place_changes() {
    // iter_mut() gives &mut T for each element — you CAN modify elements
    // in place. This works because you're modifying the CONTENTS, not the
    // Vec structure (no push/remove/resize).

    let mut latencies_ms = vec![120, 340, 89, 210, 550];

    // Double every latency (simulating a slowdown)
    for lat in latencies_ms.iter_mut() {
        *lat *= 2;
    }

    assert_eq!(latencies_ms, vec![240, 680, 178, 420, 1100]);

    // You can also use iter_mut() with conditionals
    for lat in latencies_ms.iter_mut() {
        if *lat > 500 {
            *lat = 500; // cap at 500
        }
    }
    assert_eq!(latencies_ms, vec![240, 500, 178, 420, 500]);
}

// =============================================================================
// 6. RETURNING REFERENCES FROM VECTORS: LIFETIMES
// =============================================================================
// When a function returns a reference into a Vec, the compiler needs to know
// that the Vec lives long enough. This is where lifetimes appear.

#[test]
fn returning_references() {
    // When you return a reference to an element, the lifetime of that
    // reference is tied to the lifetime of the Vec.

    // The lifetime 'a says: the returned &str lives as long as the input slice.
    fn find_longest<'a>(entries: &'a [String]) -> Option<&'a str> {
        entries.iter().map(|s| s.as_str()).max_by_key(|s| s.len())
    }

    let logs = vec![
        "short".to_string(),
        "a]much longer entry".to_string(),
        "medium len".to_string(),
    ];

    let longest = find_longest(&logs).unwrap();
    assert_eq!(longest, "a]much longer entry");

    // `longest` borrows from `logs`, so `logs` must stay alive while
    // `longest` is in use. The compiler enforces this.
}

#[test]
fn lifetime_of_vec_elements() {
    // Common pattern: store references from a Vec in another collection.
    // The original Vec must outlive the references.

    let all_servers = vec![
        "web-1".to_string(),
        "web-2".to_string(),
        "db-1".to_string(),
        "db-2".to_string(),
        "cache-1".to_string(),
    ];

    // Borrow specific entries — these references are valid as long as
    // `all_servers` exists and is not modified.
    let web_servers: Vec<&str> = all_servers
        .iter()
        .filter(|s| s.starts_with("web"))
        .map(|s| s.as_str())
        .collect();

    assert_eq!(web_servers, vec!["web-1", "web-2"]);
    assert_eq!(all_servers.len(), 5); // original still intact
}

// =============================================================================
// 7. OWNERSHIP IN ITERATION: iter() vs into_iter()
// =============================================================================
// This distinction is critical and often confusing:
//
//   .iter()       → yields &T      (borrows each element)
//   .iter_mut()   → yields &mut T  (mutably borrows each element)
//   .into_iter()  → yields T       (moves/consumes each element)
//
// A `for` loop calls .into_iter() by default on an owned Vec, but .iter()
// on a &Vec.

#[test]
fn iteration_ownership() {
    let names = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];

    // iter() — borrows. `names` remains usable.
    let lengths: Vec<usize> = names.iter().map(|s| s.len()).collect();
    assert_eq!(lengths, vec![5, 4, 5]);
    assert_eq!(names.len(), 3); // still here

    // into_iter() — consumes. Each String is moved out.
    let upper: Vec<String> = names.into_iter().map(|s| s.to_uppercase()).collect();
    assert_eq!(upper, vec!["ALPHA", "BETA", "GAMMA"]);
    // `names` is gone — moved into the iterator.

    // For Copy types (i32, f64, etc.), iter() gives &i32 and into_iter()
    // gives i32, but since i32 is Copy, you can dereference freely:
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6]);
    assert_eq!(nums.len(), 3); // i32 is Copy, so iter() just copies
}

#[test]
fn for_loop_ownership() {
    // `for item in vec` MOVES the vec (calls into_iter)
    let words = vec!["hello".to_string(), "world".to_string()];
    let mut collected = Vec::new();
    for word in words {
        // `word` is an owned String here (moved out of the vec)
        collected.push(word.to_uppercase());
    }
    // `words` is no longer usable
    assert_eq!(collected, vec!["HELLO", "WORLD"]);

    // `for item in &vec` BORROWS the vec (calls iter)
    let fruits = vec!["apple".to_string(), "banana".to_string()];
    let mut count = 0;
    for _fruit in &fruits {
        // `_fruit` is a &String here
        count += 1;
    }
    assert_eq!(count, 2);
    assert_eq!(fruits.len(), 2); // still available
}

// =============================================================================
// 8. PRACTICAL PATTERNS: drain(), split_off(), swap_remove()
// =============================================================================
// These methods give you fine-grained control over ownership transfer
// without cloning.

#[test]
fn drain_transfers_ownership() {
    let mut buffer = vec![1, 2, 3, 4, 5];

    // drain(range) removes elements from the Vec and yields them as owned values.
    // The Vec shrinks, and you get ownership of the drained items.
    let first_three: Vec<i32> = buffer.drain(..3).collect();

    assert_eq!(first_three, vec![1, 2, 3]);
    assert_eq!(buffer, vec![4, 5]); // remaining elements shift down
}

#[test]
fn split_off_divides_a_vec() {
    let mut front = vec![1, 2, 3, 4, 5];

    // split_off(index) splits the Vec in two at the given index.
    // `front` keeps [0..index), and the returned Vec gets [index..].
    let back = front.split_off(3);

    assert_eq!(front, vec![1, 2, 3]);
    assert_eq!(back, vec![4, 5]);
    // Both are independently owned — no borrowing issues.
}

#[test]
fn swap_remove_for_fast_unordered_removal() {
    let mut alerts = vec!["disk", "cpu", "memory", "network", "io"];

    // remove(index) is O(n) because it shifts all elements after the index.
    // swap_remove(index) is O(1) — it swaps the element with the last one
    // and then pops. Use it when order doesn't matter.

    let removed = alerts.swap_remove(1); // removes "cpu"
    assert_eq!(removed, "cpu");
    // "io" (the last element) took "cpu"'s place
    assert_eq!(alerts, vec!["disk", "io", "memory", "network"]);
}

// =============================================================================
// 9. API DESIGN: &[T] vs &Vec<T>
// =============================================================================
// Prefer &[T] (slice) over &Vec<T> in function signatures. Slices are more
// general — they accept arrays, vectors, and subslices. &Vec<T> only accepts
// vectors.

#[test]
fn prefer_slices_in_apis() {
    // GOOD: accepts any contiguous sequence of i32
    fn sum(values: &[i32]) -> i32 {
        values.iter().sum()
    }

    // Works with a Vec
    let v = vec![1, 2, 3];
    assert_eq!(sum(&v), 6);

    // Works with a fixed-size array
    let a = [4, 5, 6];
    assert_eq!(sum(&a), 15);

    // Works with a subslice
    assert_eq!(sum(&v[1..]), 5);

    // Similarly, prefer &str over &String, and &[u8] over &Vec<u8>.
    // The pattern is: accept the borrowed form of the inner data, not
    // the borrowed form of the container.
}

// =============================================================================
// 10. COMMON MISTAKES SUMMARY
// =============================================================================
// This test documents patterns that look right but won't compile, along
// with their fixes.

#[test]
fn common_mistake_reference_then_push() {
    let mut v = vec![1, 2, 3];
    // MISTAKE: let r = &v[0]; v.push(4); println!("{r}");
    // FIX: copy the value out first
    let val = v[0]; // i32 is Copy — no borrow held
    v.push(4);
    assert_eq!(val, 1);
}

#[test]
fn common_mistake_return_ref_to_local() {
    // MISTAKE (won't compile):
    //   fn make_vec() -> &Vec<i32> {
    //       let v = vec![1, 2, 3];
    //       &v  // ERROR: returns reference to local variable
    //   }

    // FIX: return the owned Vec
    fn make_vec() -> Vec<i32> {
        vec![1, 2, 3] // ownership moves to the caller
    }
    let v = make_vec();
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn common_mistake_multiple_mutable_borrows() {
    let mut data = vec![10, 20, 30, 40];

    // MISTAKE: two mutable borrows at once
    //   let a = &mut data[0];
    //   let b = &mut data[1];  // ERROR: can't borrow `data` mutably twice
    //   *a += 1; *b += 1;

    // FIX: use split_at_mut to get non-overlapping mutable slices
    let (left, right) = data.split_at_mut(2);
    left[0] += 1;   // &mut data[0]
    right[0] += 1;  // &mut data[2]
    assert_eq!(data, vec![11, 20, 31, 40]);
}

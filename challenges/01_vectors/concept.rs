// =============================================================================
// Challenge 01: Vectors in Rust — Concept Explainer
// =============================================================================
//
// Vec<T> is Rust's growable, heap-allocated array type. It is the most commonly
// used collection in Rust. Think of it as a resizable buffer that owns its data.
//
// This file demonstrates the core operations and compares Vec to alternatives.
// Run the tests with:
//     rustc concept.rs --edition 2024 --test && ./concept
// =============================================================================

fn main() {
    println!("Run this file with --test to execute the examples.");
}

// =============================================================================
// 1. CREATING VECTORS
// =============================================================================
// There are two main ways to create a vector:
//   - Vec::new()   — creates an empty vector; you must specify or let Rust infer the type.
//   - vec![...]    — a macro that creates a vector with initial values.
//
// Vectors store their data on the heap. They track three things internally:
//   - A pointer to the heap buffer
//   - The length (number of elements currently stored)
//   - The capacity (how many elements the buffer can hold before reallocating)

#[test]
fn creating_vectors() {
    // Using Vec::new() — type is inferred from the first push
    let mut v: Vec<i32> = Vec::new();
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);

    // Using the vec! macro with initial values
    let v2 = vec![1, 2, 3];
    assert_eq!(v2.len(), 3);

    // vec! with a repeated value: creates 5 elements, all zeros
    let zeros = vec![0; 5];
    assert_eq!(zeros, vec![0, 0, 0, 0, 0]);

    // With capacity pre-allocated (avoids reallocations if you know the size)
    let v3: Vec<String> = Vec::with_capacity(100);
    assert_eq!(v3.len(), 0); // length is 0 — no elements yet
    assert!(v3.capacity() >= 100); // but the buffer is already allocated

    // Push onto the first vector to show it works
    v.push(42);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 42);
}

// =============================================================================
// 2. COMMON OPERATIONS: push, pop, len, is_empty, indexing, get()
// =============================================================================
// - push(value)  — appends to the end. O(1) amortized.
// - pop()        — removes and returns the last element as Option<T>. O(1).
// - len()        — returns the number of elements.
// - is_empty()   — returns true if len() == 0.
// - vec[index]   — direct indexing. PANICS if index is out of bounds!
// - vec.get(i)   — returns Option<&T>. Returns None instead of panicking.

#[test]
fn common_operations() {
    let mut temps = vec![72.0, 68.5, 75.3];

    // push adds to the end
    temps.push(80.1);
    assert_eq!(temps.len(), 4);
    assert_eq!(temps[3], 80.1);

    // pop removes from the end and returns Option<T>
    let last = temps.pop();
    assert_eq!(last, Some(80.1));
    assert_eq!(temps.len(), 3);

    // Popping from an empty vec returns None (does NOT panic)
    let mut empty: Vec<i32> = Vec::new();
    assert_eq!(empty.pop(), None);

    // Direct indexing — be careful, this panics on out-of-bounds!
    assert_eq!(temps[0], 72.0);

    // Safe indexing with get() — returns Option<&T>
    assert_eq!(temps.get(0), Some(&72.0));
    assert_eq!(temps.get(999), None); // no panic, just None

    // Other useful methods
    assert!(!temps.is_empty());
    assert!(temps.contains(&68.5));

    // insert(index, value) — inserts at a position, shifting elements right. O(n).
    temps.insert(1, 70.0);
    assert_eq!(temps, vec![72.0, 70.0, 68.5, 75.3]);

    // remove(index) — removes at a position, shifting elements left. O(n).
    temps.remove(1);
    assert_eq!(temps, vec![72.0, 68.5, 75.3]);

    // retain — keeps only elements that satisfy a predicate
    temps.retain(|&t| t > 70.0);
    assert_eq!(temps, vec![72.0, 75.3]);
}

// =============================================================================
// 3. ITERATION: iter(), iter_mut(), into_iter()
// =============================================================================
// Rust has three ways to iterate over a vector, each with different ownership:
//
//   iter()       — borrows each element as &T. The vector is NOT consumed.
//   iter_mut()   — borrows each element as &mut T. You can modify in place.
//   into_iter()  — takes ownership of each element. The vector IS consumed.
//
// You can also iterate with a for loop, which calls into_iter() by default
// on an owned Vec, or iter() on a &Vec reference.

#[test]
fn iteration() {
    let numbers = vec![10, 20, 30, 40, 50];

    // iter() — immutable references. The vector is still usable after.
    let sum: i32 = numbers.iter().sum();
    assert_eq!(sum, 150);
    assert_eq!(numbers.len(), 5); // numbers is still here

    // iter() with map and collect — transform elements into a new vector
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    assert_eq!(doubled, vec![20, 40, 60, 80, 100]);

    // iter() with filter — select elements matching a condition
    let big: Vec<&i32> = numbers.iter().filter(|&&x| x > 25).collect();
    assert_eq!(big, vec![&30, &40, &50]);

    // iter_mut() — mutable references. Modify elements in place.
    let mut scores = vec![85, 90, 78];
    for score in scores.iter_mut() {
        *score += 5; // add 5 bonus points to each score
    }
    assert_eq!(scores, vec![90, 95, 83]);

    // into_iter() — consumes the vector. Each element is moved out.
    let names = vec![String::from("alice"), String::from("bob")];
    let upper: Vec<String> = names.into_iter().map(|s| s.to_uppercase()).collect();
    assert_eq!(upper, vec!["ALICE", "BOB"]);
    // names is no longer usable here — it was consumed by into_iter()

    // enumerate() — get index alongside each element
    let items = vec!["a", "b", "c"];
    let indexed: Vec<(usize, &&str)> = items.iter().enumerate().collect();
    assert_eq!(indexed[0], (0, &"a"));
    assert_eq!(indexed[2], (2, &"c"));
}

// =============================================================================
// 4. SLICING
// =============================================================================
// A slice (&[T]) is a reference to a contiguous portion of a vector (or array).
// Slices don't own data — they borrow it. They are extremely common in Rust APIs.
//
// Syntax: &vec[start..end]  — elements from `start` up to (not including) `end`
//         &vec[start..]     — from `start` to the end
//         &vec[..end]       — from the beginning up to `end`
//         &vec[..]          — the entire vector as a slice

#[test]
fn slicing() {
    let data = vec![100, 200, 300, 400, 500];

    // A slice of the middle three elements
    let middle: &[i32] = &data[1..4];
    assert_eq!(middle, &[200, 300, 400]);

    // First two elements
    let first_two = &data[..2];
    assert_eq!(first_two, &[100, 200]);

    // Last two elements
    let last_two = &data[3..];
    assert_eq!(last_two, &[400, 500]);

    // The whole vector as a slice
    let all: &[i32] = &data[..];
    assert_eq!(all.len(), 5);

    // Functions that accept &[T] work with both vectors and arrays
    fn sum_slice(s: &[i32]) -> i32 {
        s.iter().sum()
    }
    assert_eq!(sum_slice(&data), 1500); // pass a vec as a slice
    assert_eq!(sum_slice(&[1, 2, 3]), 6); // pass an array as a slice

    // Slices support many of the same methods as Vec
    assert_eq!(middle.first(), Some(&200));
    assert_eq!(middle.last(), Some(&400));
    assert!(middle.contains(&300));
}

// =============================================================================
// 5. SORTING AND DEDUPLICATION
// =============================================================================

#[test]
fn sorting_and_dedup() {
    // sort() — sorts in place, ascending
    let mut vals = vec![5, 3, 8, 1, 3, 5, 2];
    vals.sort();
    assert_eq!(vals, vec![1, 2, 3, 3, 5, 5, 8]);

    // dedup() — removes *consecutive* duplicates (so sort first!)
    vals.dedup();
    assert_eq!(vals, vec![1, 2, 3, 5, 8]);

    // sort_by for custom ordering (e.g., descending)
    vals.sort_by(|a, b| b.cmp(a));
    assert_eq!(vals, vec![8, 5, 3, 2, 1]);

    // For floating-point, use sort_by with partial_cmp
    let mut floats = vec![3.1, 1.4, 2.7];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(floats, vec![1.4, 2.7, 3.1]);
}

// =============================================================================
// 6. Vec vs FIXED-SIZE ARRAYS [T; N]
// =============================================================================
// Arrays have a fixed size known at compile time. They live on the stack.
// Use arrays when:
//   - The size is known at compile time and won't change
//   - You want stack allocation for performance
//   - You're working with small, fixed collections (e.g., RGB color = [u8; 3])
//
// Use Vec when:
//   - The size is determined at runtime
//   - Elements will be added or removed
//   - The collection could be large (stack space is limited)

#[test]
fn vec_vs_array() {
    // Fixed-size array — size is part of the type
    let rgb: [u8; 3] = [255, 128, 0];
    assert_eq!(rgb.len(), 3);
    // rgb.push(50); // ERROR: arrays don't have push!

    // Vec — dynamic size
    let mut colors: Vec<[u8; 3]> = Vec::new();
    colors.push(rgb);
    colors.push([0, 0, 255]);
    assert_eq!(colors.len(), 2);

    // Both can be passed as slices to the same function
    fn first_element(s: &[u8]) -> u8 {
        s[0]
    }
    assert_eq!(first_element(&rgb), 255);

    let flat_colors: Vec<u8> = vec![10, 20, 30];
    assert_eq!(first_element(&flat_colors), 10);
}

// =============================================================================
// 7. Vec vs VecDeque (double-ended queue)
// =============================================================================
// VecDeque supports efficient push/pop at BOTH ends (front and back).
// Vec is only efficient at the back — pushing to the front is O(n).
//
// Use VecDeque when:
//   - You need a queue (FIFO) or deque
//   - You frequently push_front or pop_front
//
// Use Vec when:
//   - You only add/remove at the end
//   - You need contiguous memory (VecDeque may wrap around internally)

#[test]
fn vec_vs_vecdeque() {
    use std::collections::VecDeque;

    // Vec: push_front is O(n) — it shifts every element
    let mut v = vec![2, 3, 4];
    v.insert(0, 1); // this is how you "push_front" on a Vec — expensive!
    assert_eq!(v, vec![1, 2, 3, 4]);

    // VecDeque: push_front is O(1)
    let mut dq = VecDeque::new();
    dq.push_back(2);
    dq.push_back(3);
    dq.push_front(1); // efficient!
    assert_eq!(dq, VecDeque::from([1, 2, 3]));

    // VecDeque as a queue (FIFO): push to back, pop from front
    let mut queue = VecDeque::new();
    queue.push_back("first");
    queue.push_back("second");
    queue.push_back("third");
    assert_eq!(queue.pop_front(), Some("first"));
    assert_eq!(queue.pop_front(), Some("second"));
}

// =============================================================================
// 8. Vec vs HashSet (unique elements and fast lookups)
// =============================================================================
// HashSet stores unique elements with O(1) average lookup time.
// Vec has O(n) lookup (linear search) and allows duplicates.
//
// Use HashSet when:
//   - You need to ensure uniqueness
//   - You frequently check "does this collection contain X?"
//   - Order doesn't matter
//
// Use Vec when:
//   - Order matters
//   - Duplicates are allowed or expected
//   - You need indexed access

#[test]
fn vec_vs_hashset() {
    use std::collections::HashSet;

    // Vec allows duplicates
    let with_dupes = vec![1, 2, 2, 3, 3, 3];
    assert_eq!(with_dupes.len(), 6);

    // Convert to HashSet to remove duplicates
    let unique: HashSet<i32> = with_dupes.into_iter().collect();
    assert_eq!(unique.len(), 3);
    assert!(unique.contains(&1));
    assert!(unique.contains(&2));
    assert!(unique.contains(&3));

    // HashSet is great for membership checks
    let mut seen = HashSet::new();
    let data = vec!["apple", "banana", "apple", "cherry", "banana"];

    let mut first_occurrences = Vec::new();
    for item in &data {
        if seen.insert(*item) {
            // insert returns true if the value was NOT already present
            first_occurrences.push(*item);
        }
    }
    assert_eq!(first_occurrences, vec!["apple", "banana", "cherry"]);

    // If you need both uniqueness AND sorted order, collect to Vec and sort
    let mut sorted_unique: Vec<i32> = unique.into_iter().collect();
    sorted_unique.sort();
    assert_eq!(sorted_unique, vec![1, 2, 3]);
}

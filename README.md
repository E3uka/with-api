# With-api

A simple set of macros for the ergonomic shinking of scope.

## Why

When doing common operations such as acquiring a Mutex lock or opening a file
it can be easy to forget that the lock is held or the file is open for the 
entirety of the following blocks scope. This crate provides a simple set of 
macros, influenced by the with statement in Python, that aims to:

* Shrink 'critical' sections of code ensuring resources are adequately released.
* Provide a familiar API that follows Rusts ownership rules.
* Be easy to remember and thus use.

## Examples

### Owned
```rust
use with_api::with;

let mine = "all mine".to_string();

with!(mine, |greedy| {
	assert!(!greedy.is_empty());
});

// The below fails as its been moved into with!
// println!("{:?}", mine);
```

### Borrowed
```rust
use with_api::bor_with;

let immutable: usize = 27;

bor_with!(immutable, |num| {
	assert!(*num == 27);

	// The below fails as it cannot be mutated.
	// *num = 28;
});
```

### Exclusively Borrowed
```rust
use with_api::mut_with;

let protec: Mutex<HashMap<usize, String>> =
    Mutex::new(Vec::new().into_iter().collect());

mut_with!(protec.lock().unwrap(), |db| {
    let _ = db.insert(42, "meaning of life".to_string());
    assert!(!db.is_empty());

	// lock released at end of scope.
});
```

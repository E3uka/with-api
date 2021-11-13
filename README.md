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

// the below fails as its been moved into with!
// println!("{:?}", mine);
}
```

see the [docs](https://docs.rs/with-api/) for more examples.


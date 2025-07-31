# JH-Rust

A simple singly linked list implementation in Rust for learning purposes.

## What it does

- Basic linked list operations (push, pop, peek)
- Iterator support
- Reverse operation
- Interleave two lists
- Apply function to transform elements
- Display and equality traits

## Usage

```rust
use jh_rust::singly_linked_list::List;

let mut list = List::new();
list.push(1);
list.push(2);
list.push(3);

assert_eq!(list.pop(), Some(3));
assert_eq!(list.pop(), Some(2));
assert_eq!(list.pop(), Some(1));
```

## Testing

```bash
cargo test
```

## About

This is my project for learning Rust. I'm implementing data structures to understand Rust's ownership system, traits, and memory management.

# HashSet Demo in Rust

A simple demonstration project showcasing the usage of `HashSet` in Rust. This project illustrates the basic operations and functionality of the `HashSet` collection type from Rust's standard library.

## Introduction

`HashSet` is a collection that contains unique elements with no guaranteed order. This project demonstrates:

- Creating a new HashSet
- Adding elements
- Checking for element existence
- Removing elements
- Getting the set size
- Iterating through elements

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust installation)

..

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/warathepj/rust-hashset-u.git
   cd rust-hashset-u
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Usage

Run the demo:

```bash
cargo run
```

### Expected Output

```
Initial set: {}
Set after insertions: {"apple", "orange", "banana"}
'banana' is in the set!
'grape' is NOT in the set.
Set after removing 'banana': {"apple", "orange"}
Number of elements in the set: 2
Elements in the set:
- apple
- orange
```

Note: The order of elements in the output may vary since `HashSet` doesn't guarantee any specific order.

## Key HashSet Operations

| Operation  | Method                | Description                                         |
| ---------- | --------------------- | --------------------------------------------------- |
| Creation   | `HashSet::new()`      | Creates a new empty HashSet                         |
| Insertion  | `insert()`            | Adds an element to the set (if not already present) |
| Membership | `contains()`          | Checks if an element exists in the set              |
| Removal    | `remove()`            | Removes an element from the set                     |
| Size       | `len()`               | Returns the number of elements in the set           |
| Iteration  | `for element in &set` | Iterates through all elements                       |


# Swordfish

Swordfish is a Data-Oriented Design (DOD) Logic Layer for large scale social based simulations / games. It's designed for fast query / discrete graphs processing performance and the ease of use.

---

## Features

* **Macro-driven Table Creation**: Define your data tables with a simple macro.
* **Efficient Data Storage**: Data is stored in parallel vectors, allowing for cache-friendly iteration and operations.
* **Fast Insertions**: Reuses IDs from deleted entries to minimize memory reallocations and maintain contiguous data.
* **Easily Querying**: Iterate over your table data easily with `query!` and `query_mut!` macros, providing direct access to fields.

---

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
swordfish = "0.9.0" # Or the latest version
```
## Usage
Use the `create_table!` macro to define your archetype.
```rust
use swordfish::{create_table, query, query_mut};

create_table!(
    /// Represents a game entity with position and health.
    EntityTable, //Define archetype name
    position_x: f32,
    position_y: f32,
    health: i32
);

fn main() {
    // ...
}
```
This will generate a `struct` named `EntityTable` with fields for `position_x`, `position_y`, and `health`, along with management methods.

## Creating a Table Instance
```rust
use swordfish::{create_table, query, query_mut};

create_table!(
      /// Represents a game entity with position and health.
      EntityTable,
      position_x: f32,
      position_y: f32,
      health: i32
);

fn main() {
    let mut entities = EntityTable::new();
    // ...
}
```

## Inserting Data
Use the `insert` method to add new entries to your table. It returns the `id` of the newly inserted entry.
```rust
use swordfish::{create_table, query, query_mut};

create_table!(
      /// Represents a game entity with position and health.
      EntityTable,
      position_x: f32,
      position_y: f32,
      health: i32
);
 
fn main() {
    let mut entities = EntityTable::new();

    let entity_id_1 = entities.insert(10.0, 20.0, 100);
    let entity_id_2 = entities.insert(5.0, 15.0, 75);

    println!("Entity 1 ID: {}", entity_id_1); // Output: Entity 1 ID: 0
    println!("Entity 2 ID: {}", entity_id_2); // Output: Entity 2 ID: 1
}
```
## Querying Data
The `query!` macro allows you to iterate over your table's data, providing immutable references to the fields.
```rust
query!(entities, id, pos_x: position_x, pos_y: position_y, hp: health, 
    {
    println!("Entity ID: {}, Pos: ({}, {}), Health: {}", id, pos_x, pos_y, hp);
    }
);
```

## Mutating Data
The `query_mut!` macro is similar to `query!`, but it provides mutable references to the fields, allowing you to modify them within the loop.
```rust
query_mut!(entities, id, hp: health, 
    {
        *hp -= 10;     
    }
);
```

## Deleting Data
You can mark entries for deletion using `delete_by_id`. Work safe ONLY in `query_mut` (or `query`) statement.
```rust
query_mut!(entities, id1, hp: health, {
    entities.delete_by_id(id1);
});
```




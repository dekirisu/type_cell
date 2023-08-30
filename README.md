<p align="center">
    <a href="https://github.com/dekirisu/type_cell" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu/type_cell-ee6677">
    </a>
    <a href="https://crates.io/crates/type_cell" style="position:relative">
        <img src="https://img.shields.io/crates/v/type_cell">
    </a>
    <a href="https://docs.rs/type_cell" style="position:relative">
        <img src="https://img.shields.io/docsrs/type_cell">
    </a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative">
        <img src="https://img.shields.io/discord/515100001903312898">
    </a>
</p>
Macro to attach <a href="https://github.com/matklad/once_cell">OnceCell</a> to a Type using getter/setter methods. This is mainly useful for variables which will be set at the start and be accessed read-only.
Initially developed for use with the Bevy-Engine to easily access Handles (smart pointers to assets) globally.

- The value can only be set once.
- The value has to be set before getting.

## Simplified Usage

- `$typeOn > $typeStore: $name_1, $name_n;` <br>
  **$typeOn:** The Type to implement the getter/setter methods in.<br>
  **$typeStore:** The Type, stored in the cell.<br>
  **$names:** The Type, stored in the cell.
```rust
use type_cell::*;
type_cell!{
    // #clone // the getters return the values cloned
    // store a vec of bools on the bool type
    bool > Vec<bool>: bools;
    // store a u8 on the u8 type
    u8 > u8: app_id, seed;
}
fn main () {
    // set global on startup
    bool::set_bools(vec![true,false]);
    u8::set_app_id(100);
    u8::set_seed(111);
    // get anywhere
    assert_eq!(&vec![true,false], bool::get_bools());
    assert_eq!(&100, u8::get_app_id());
    assert_eq!(&111, u8::get_seed());
}
```
## Query Usage
Queries are the base of this crate, they consist of the following parts and are separated by **|** : <br>
`on $typeOn > store $typeStore | set $typeIn | get $typeOut | $name`
- `on $typeOn > store $typeStore` <br>
  **$typeOn:** The Type to implement the getter/setter methods in.<br>
  **$typeStore:** The Type, stored in the cell.
- `set $typeIn[.methods(val:type)]` <br>
  **$typeIn:** Input-Parameter of the setter method. <br>
  **.methods(val:type):** Methods applied on $typeIn to fit $typeStore. Parameters of those will be added to the setter method and forwarded.
- `get $paramOut[.methods(val:type)]`<br>
  **$typeOut:** Output-Type of the getter method.<br>
  **.methods(val:type):** Methods applied on $typeStore to fit $typeOut. Parameters of those will be added to the getter method and forwarded.
- `$name` <br>
  **$name:** Name of the value, method names will be: `get_$name` and `set_$name`.

### Example 1:
Store bool on bool, set bool directly and get a reference to it.
```rust
use type_cell::*;
type_cell!(on bool > store bool | set bool | get &'static bool | test);
fn main () {
    bool::set_test(true);
    assert_eq!(&true,bool::get_test());
}
```

### Example 2:
Store bool on bool, set bool directly and get a clone of it.
```rust
use type_cell::*;
type_cell!(on bool > store bool | set bool | get bool.clone() | test);
fn main () {
    bool::set_test(true);
    assert_eq!(true,bool::get_test());
}
```
## Prefabs
Currently there are simplifications for Vec and HashMap indicated by a **@**:
```rust
use type_cell::*;
type_cell!{
    @Vec #unwrap
    u8: vec;
}
fn main () {
    u8::set_vec(vec![50,100,150,200]);
    assert_eq!(&150, u8::get_vec(2));
}
```
```rust
use type_cell::*;
use std::collections::HashMap;
type_cell!{
    @HashMap<usize> #unwrap #clone
    bool: map1, map2;
    u8: map3, map4;
}
fn main () {
    u8::set_map3(HashMap::from([
        (11,50), (22,100), (33,150), (44,200)
    ]));
    assert_eq!(150, u8::get_map3(&33));
}
```

## Mutability
Mutability can be achieved using Mutex, RwLock or other locks, just make sure you know what you're doing!

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
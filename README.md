<p align="center">
    <img src="https://user-images.githubusercontent.com/78398528/282165324-e99cae4c-ce93-402c-949f-3f48708a716b.gif">
</p>
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
</p>

Macro to 'attach' values statically to a type using static getter and setter methods.
```toml
[dependencies]
type_cell = "0.3"
```
```rust
use type_cell::*;
// Simple Preview 
tycell!{u32:[a_number];} 
u32::set_a_number(6);
assert_eq!(&6u32,u32::get_a_number());
```

## 🧱 Basic Usage
- Use the macro: `tycell!{...}`
- Which type should the value be 'attached' on? `u32 {...}`
- Which type does the value have? `static u32:`
    - Which settings will it use?<br>
    🌟 `once_read` Set it once. Get it read-only! (combine with Mutex/RwLock/... for mutability)<br>
    🏁 `once_write` Set it once. Get it mutable, but risk race conditions! (be sure you win the race!)<br>
    🦥 `lazy_read` Like `once_read` but set lazy inside the macro!<br>
    👹 `lazy_write`Like `once_write` but set lazy inside the macro!! 
    - examples: `static u32: once_read;` or `static String: lazy_read;`
- What's the name of the default setter method? `set_type()`
- What's the name of the default getter method? `get_type()`

```rust
// Basic Usage 
tycell!{ bool {
    static Vec<bool>: once_read;
    set set_vec();
    get vec();
}}
// Set it somewhere once:
bool::set_vec(Vec::from([true,false,true]));
// Get it anywhere afterwards:
assert_eq!(&[true,false,true],bool::vec().as_slice());
```
The default setter parameter is a dynamic `Into<..>` and will use `.into()`.<br>
This means in this example you could also set it like this:
```rust
bool::set_vec([true,false,true]);
assert_eq!(&[true,false,true],bool::vec().as_slice());
```
## ⚗ Advanced Usage
Multiple Setter and Getter with different parameters and return types can be defined! <br>
There are two ways of doing it:
- **Methods:** 
    - Use inline methods for simple conversions!
    - `set set_bool(Option<usize>): do.is_some();`
    - `get get_bool() -> bool: static.clone();`
- **Function:** 
    - Use a function with correct parameters/return types and is accessible in the same file!
    - Use `=` before the function meta!
    - `set =set_base_fn(a:Option<usize>);` 
    - `get =get_base_fn() -> bool;`
```rust
// Advanced Usage 
fn set_by_function (a:Option<usize>) -> bool {a.is_some()}
fn get_by_function (a:&bool) -> bool {a.clone()}
tycell!{ bool {
    static bool: once_read;
    set set_raw();
    set set_by_methods(Option<usize>): do.is_some();
    set =set_by_function(a:Option<usize>);
    get get_raw();
    get get_by_methods() -> bool: static.clone();
    get =get_by_function() -> bool;
}}
bool::set_by_methods(None);
assert_eq!(false,bool::get_by_methods());
```
Methods with parameters are supported in two different ways:
- **Constants:**
    - Using `=` before a constant value!
    - `set set_number(u32): do.clamp(=0,=100);`
    - `get get_number() -> bool: static.clamp(=0,=100);`
- **Pass Through:**
    - Naming the values with its types will pass it into the function!
    - `set set_number(u32): do.clamp(min:u32,max:u32);`
    - `get get_number() -> bool: static.clamp(min:u32,max:u32);`
```rust
// Advanced Usage 
tycell!{ u32 {
    static u32: once_read;
    set set_raw();
    set set_by_methods(u32): do.clamp(=0,=100);
    set set_pass(u32): do.clamp(min:u32,max:u32);
    get get_raw();
    get get_by_methods() -> u32: static.add(=5);
    get get_pass() -> u32: static.add(val:u32);
}}
// Sets value to 1000.clamp(0,123) = 123
u32::set_pass(1000,0,123); 
// Gets 123.add(5) = 128
assert_eq!(128,u32::get_by_methods());
```
## 🧊 Constant
You can also set const values!
```rust
// Constant
tycell!{ u32 {
    const u32 = 100;
    get number();
}}
// Gets 10!
assert_eq!(10,u32::number());
```

## 👹 Risky Mutable Options
⚠`Only use this if you're sure there are no race conditions (or they don't matter) or for debug purposes!`<br>
To make the static value mutable, use `once_write` or `lazy_write`.
```rust
// Risky Mutable
tycell!{ u32 {
    static u32: risky_write;
    set set_number();
    get number();
}}
// Set it somewhere once:
u32::set_number(5u32);
// Default getter is mutable already
*u32::number() = 10;
// Gets 10!
assert_eq!(10,*u32::number());
```

## 🦥 As Lazy Static
To create a lazy static value, use the `lazy_read` option and use a block instead of the setter function!
```rust
// Lazy Static
tycell!{ u32 {
    static HashMap<u32,String>: lazy_read;
    set {
        let mut map = HashMap::new();
        for i in 0..100 {
            map.insert(i,i.to_string());
        }
        map
    }
    get get_lazy_map();
    get get_lazy() -> Option<&String>: static.get(id:&u32);
}}
// Gets Some("3":&String)
assert_eq!(&"3",&u32::get_lazy(&3).unwrap());
```
## ➡ Simple Mapping
If you only need the default getter and setters, there is a short form:
```rust
// Simple Usage
tycell!{
    // store a vec of bools on the bool type
    // a single specifier inside [..] will use once_read
    // adding 'mut' before it sets it to once_write
    // adding a block {} after the specifier will use lazy_.. instead of once_..
    bool > Vec<bool>: [bools] [mut more_bools] [lazy_bools{vec![true,false]}];
    // adding '= value' after the specifier will set a constant value
    bool > u32: [number=100];
}
bool::set_bools([true,false]);
bool::set_more_bools([true,false]);
```
If you only attach values of the same type as their parent:
```rust
// Simplest Usage
tycell!{
    // Same as bool > bool: [is_nice];
    bool: [is_nice];
}
```
If you want to attach a type to its single generic type, e.g. `u32 > Vec<u32>` you can use `!Vec<u32>`.<br>
Increase the number of `!` to set the level, e.g. `u32 > Vec<Vec<u32>>` <=> `!!Vec<Vec<u32>>`.<br>
```rust
tycell!{
    !Vec<bool>:[is_nice];
}
```

You can't mix different types of left-handed syntax, unless wrapped in `{}`
```rust
// working
tycell!{
    {!Vec<bool>}:[is_nice];
    {bool>Vec<bool>}:[is_v_nice];
    {bool}:[is_x_nice];
}
// NOT working
tycell!{
    !Vec<bool>:[is_nice];
    bool>Vec<bool>:[is_v_nice];
    bool:[is_x_nice];
}
```

## ➡ Simple (Hash)Maps and Vecs
Ease up getting values from a HasmMap-esque types, by using <key> after the name.<br>
if no key is provided, the type is set to a Vec<..> instead.
```rust
// uses anythng named TyMap for flaxibility
use std::collections::HashMap as TyMap;
tycell!{
    // same as above, but <keytype> after the specifier
    bool > bool: [bools<u8>] [mut more_bools<u8>] [lazy_bools<u8>{[(1,true)]}];
}
bool::set_bools([(1,true)]);
bool::set_more_bools([(1,true)]);
```
## 🔗 Related Projects
- <a href="https://crates.io/crates/bevy_cell">bevy_cell</a> - Attach bevy Handle and Entity to types.
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
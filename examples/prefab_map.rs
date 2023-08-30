use type_cell::*;
use std::collections::HashMap;

type_cell!{
    @HashMap<usize> #unwrap #clone
    bool: map1, map2;
    u8: map3, map4;
}

fn main () {
    // set global on startup
    u8::set_map3(HashMap::from([
        (11,50), (22,100), (33,150), (44,200)
    ]));
    // get anywhere
    assert_eq!(150,u8::get_map3(&33));
}
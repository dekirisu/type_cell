use type_cell::*;

type_cell!{
    // store a vecs of bools on the bool type
    bool > Vec<bool>: bools;
    // store a vecs of u8s on the u8 type
    u8 > u8: app_id, seed;
}

fn main () {
    // set global on startup
    bool::set_bools(vec![true,false]);
    u8::set_app_id(100);
    u8::set_seed(111);
    // get anywhere
    assert_eq!(&vec![true,false],bool::get_bools());
    assert_eq!(&100,u8::get_app_id());
    assert_eq!(&111,u8::get_seed());
}
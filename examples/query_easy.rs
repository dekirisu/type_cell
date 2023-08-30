use type_cell::*;

type_cell!(on bool > store bool | set bool | get &'static bool | base);

fn main () {
    // set global on startup
    bool::set_base(true);
    // get anywhere
    assert_eq!(&true,bool::get_base());
}
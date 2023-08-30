use type_cell::*;

type_cell!{
    @Vec #unwrap
    bool: vec1, vec2;
    u8: vec3, vec4;
}
fn main () {
    // set global on startup
    u8::set_vec3(vec![50,100,150,200]);
    // get anywhere
    assert_eq!(&150,u8::get_vec3(2));
}
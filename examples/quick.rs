use type_cell::*;

tycell!{u8:[con=8][once][lazy{8}][mut oncem][mut lazym{8}];}


fn main () {

    println!("{}",u8::con());

    u8::set_once(8);
    println!("{}",u8::once());
    println!("{}",u8::lazy());

    u8::set_oncem(8);
    println!("{}",u8::oncem());
    println!("{}",u8::lazym());
    *u8::lazym() = 100;
    println!("{}",u8::lazym());

}
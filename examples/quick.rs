use type_cell::*;
use std::collections::HashMap as TyMap;

tycell!{
    {u8} [con=8] [once] [lazy{8}] [mut oncem] [mut lazym{8}]
    {u32} [once.clone()] [lazy.clone()->u32{8u32}] [mut oncem.clone()->u32] [mut lazym.clone()->u32{8u32}] [mut lazym2 {8u32}.clone()->u32 ]
    {u16>u8} [once<u8>] [lazy<u8>{[(5,100)]}] [mut oncem<u8>] [mut lazym<u8>{[(1,200)]}] [mut lazyv<>{[200]}]
    {!!!!Vec<Vec<Vec<Vec<bool>>>>} [lazy<u8>{[(0,vec![vec![vec![vec![true]]]])]}]
}

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

    println!("{}",u16::lazy(&5));
    println!("{}",u16::lazym(&1));
    
    println!("{}",u16::lazyv_mut(0));

    println!("{}",bool::lazy(&0)[0][0][0][0]);

}
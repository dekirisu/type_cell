use std::{collections::HashMap, ops::Add};
use type_cell::*;

/* ------------------------------- Basic Usage ------------------------------ */

    type_cell!{ bool {
        static Vec<bool>: once!
        set set_vec(..);
        get vec();
    }}

    fn basic_usage (){
        bool::set_vec([true,false,true]);
        assert_eq!(&[true,false,true],bool::vec().as_slice());
    }

/* ---------------------------- Advanced Usage 1 ---------------------------- */

    fn set_by_function (a:Option<usize>) -> bool {a.is_some()}
    fn get_by_function (a:&bool) -> bool {a.clone()}
    type_cell!{ bool {
        static bool: once!
        set set_raw(..);
        set set_by_methods(Option<usize>): do.is_some();
        set =set_by_function(a:Option<usize>);
        get get_raw();
        get get_by_methods() -> bool: static.clone();
        get =get_by_function(..) -> bool;
    }}

    fn advanced_usage_1 (){
        bool::set_by_methods(None);
        assert_eq!(false,bool::get_by_methods());
    }

/* ---------------------------- Advanced Usage 2 ---------------------------- */

    type_cell!{ u32 {
        static u32: once!
        set set_raw(..);
        set set_by_methods(u32): do.clamp(=0,=100);
        set set_pass(u32): do.clamp(min:u32,max:u32);
        get get_raw();
        get get_by_methods() -> u32: static.add(=5);
        get get_pass() -> u32: static.add(val:u32);
    }}

    fn advanced_usage_2 (){
        u32::set_pass(1000,0,123); 
        assert_eq!(128,u32::get_by_methods());
    }

/* ------------------------------ Risky Mutable ----------------------------- */

    type_cell!{ u32 {
        static u32: risky!
        set set_number(..);
        get number();
    }}

    fn risky_mutable (){
        u32::set_number(5u32);
        *u32::number() = 10;
        assert_eq!(10,*u32::number());
    }

/* ------------------------------- Lazy Static ------------------------------ */

    type_cell!{ u32 {
        static HashMap<u32,String>: lazy!
        set {
            let mut map = HashMap::new();
            for i in 0..100 {
                map.insert(i,i.to_string());
            }
            map
        }
        get get_lazy(..) -> Option<&String>: static.get(id:&u32);
    }}

    fn lazy_static (){
        assert_eq!(&"3",&u32::get_lazy(&3).unwrap());
    }

/* ------------------------------ Simple Usage ------------------------------ */

    type_cell!{
        once! bool > Vec<bool>: bools, more_bools;
        risky! u8 > u8: id, seed;
        unsafe! String > &'static str: app_name;
    }

    fn simple_usage (){
        bool::set_bools([true,false]);
        bool::set_more_bools([true,false]);
        u8::set_id(100);
        u8::set_seed(100);
        unsafe{String::set_app_name("Name")};
    }

/* ----------------------------- Simplest Usage ----------------------------- */

    type_cell!{
        once! bool: is_nice;
        risky! u16: id, seed;
        unsafe! String: app_slug;
    }

    fn simplest_usage (){
        bool::set_is_nice(true);
        u16::set_id(100u16);
        u16::set_seed(100u16);
        unsafe{String::set_app_slug("Name")};
    }

/* ---------------------------------- Main ---------------------------------- */

    fn main (){
        basic_usage();
        advanced_usage_1();
        advanced_usage_2();
        risky_mutable();
        lazy_static();
        simple_usage();
        simplest_usage();
    }

/* ------------------------ Why are you reading this? ----------------------- */
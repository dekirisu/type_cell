use std::sync::{Mutex, TryLockResult, MutexGuard};
use type_cell::*;
type_cell!(on u8 > store Mutex<u8> | set Mutex<u8> | get.try_lock() TryLockResult<MutexGuard<'static,u8>> | mutex);

fn main () {
    // set global on startup
    u8::set_mutex(Mutex::new(100));
    // get anywhere
    if let Ok(ref mut num) = u8::get_mutex() {
        assert_eq!(100,**num);
        // change through mutex
        **num = 200;
    } 
    if let Ok(ref mut num) = u8::get_mutex() {
        assert_eq!(200,**num);
    } 
}
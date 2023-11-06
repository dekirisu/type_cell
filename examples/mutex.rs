use std::sync::{Mutex, TryLockResult, MutexGuard};
use type_cell::*;

type_cell!(u8 {
    static Mutex<u8>: once!
    set set_mutex(..);
    get get_mutex_ref();
    get get_mutex() -> TryLockResult<MutexGuard<'static,u8>>: static.try_lock();

});

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
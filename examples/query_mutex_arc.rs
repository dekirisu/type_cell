use std::sync::{Arc, Mutex, TryLockResult, MutexGuard};
use type_cell::*;

trait ToArcMutex <T> { fn to_arcmut(self) -> Arc<Mutex<T>>;}
impl ToArcMutex<u8> for u8 { fn to_arcmut(self) -> Arc<Mutex<u8>> {Arc::new(Mutex::new(self))} }
type_cell!(on u8 > store Arc<Mutex<u8>> | set.to_arcmut() u8 | get.try_lock() TryLockResult<MutexGuard<'static,u8>> | arcmut);

fn main () {
    // set global on startup
    u8::set_arcmut(100);
    // get chaeg tru mutex
    if let Ok(ref mut num) = u8::get_arcmut() {
        assert_eq!(100,**num);
        **num = 200;
    } 
    // get anywhere
    if let Ok(ref mut num) = u8::get_arcmut() {
        assert_eq!(200,**num);
    } 
}
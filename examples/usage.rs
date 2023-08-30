use type_cell::*;

/* ---------------------------------- Base ---------------------------------- */

    type_cell!(on bool > store bool | set bool | get.clone() bool | base);
    type_cell!(on bool > store Arc<Mutex<bool>> | set Arc<Mutex<bool>> | get.try_lock() TryLockResult<MutexGuard<'static,bool>> | mutex);

    trait ToArcMutex <T> { fn to_arcmut(self) -> Arc<Mutex<T>>;}
    impl ToArcMutex<bool> for bool { fn to_arcmut(self) -> Arc<Mutex<bool>> {Arc::new(Mutex::new(self))} }
    type_cell!(on bool > store Arc<Mutex<bool>> | set.to_arcmut() bool | get.try_lock() TryLockResult<MutexGuard<'static,bool>> | mutex2);

/* --------------------------------- Single --------------------------------- */

    type_cell!{
        bool > Vec<bool>: clone, clone2;
        u8 > u8: clone3, clone4;
    }
    type_cell!{
        #clone
        bool: access; 
        u8: byte, byte2;
    }

/* ----------------------------------- Vec ---------------------------------- */

    type_cell!{
        @Vec #unwrap #clone
        bool: vec1, vec2;
        u8: vec3, vec4;
    }

/* --------------------------------- HashMap -------------------------------- */

    use std::{collections::HashMap, sync::{Arc, Mutex, TryLockResult, MutexGuard}};
    type_cell!{
        @HashMap<usize> #unwrap #clone
        bool: map1, map2;
        u8: map3, map4;
    }

/* ------------------------------------ . ----------------------------------- */

fn main () {
    // set global on startup
    bool::set_mutex(Arc::new(Mutex::new(true)));
    // get anywhere
    if let Ok(ref mut bool) = bool::get_mutex() {
        **bool = false;
    } 
}
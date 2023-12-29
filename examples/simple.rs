use type_cell::*;
use std::collections::HashMap as TyMap;

tycell!{{u8}
/* ---------------------------------- Lazy ---------------------------------- */
    // Base
    [lazy{50}]
    // with Methods - Pre-Block
    [lazy_pre.clone()->u8{50}]
    // with Methods - Post-Block
    [lazy_post{50}.clone()->u8]
    // with Methods & Self Return Type - Pre-Block
    [lazy_pre_self.clone(){50}]
    // with Methods & Self Return Type - Post-Block
    [lazy_post_self{50}.clone()]

/* ------------------------------ Mutable Lazy ------------------------------ */
    // Base
    [mut mlazy{50}]
    // with Methods - Pre-Block
    [mut mlazy_pre.clone()->u8{50}]
    // with Methods - Post-Block
    [mut mlazy_post{50}.clone()->u8]
    // with Methods & Self Return Type - Pre-Block
    [mut mlazy_pre_self.clone(){50}]
    // with Methods & Self Return Type - Post-Block
    [mut mlazy_post_self{50}.clone()]

/* ---------------------------------- Once ---------------------------------- */
    // Base
    [once]
    // with Methods
    [once_pre.clone()->u8]
    // with Methods & Self Return Type
    [once_pre_self.clone()]

/* ------------------------------ Mutable Once ------------------------------ */
    // Base
    [mut monce]
    // with Methods
    [mut monce_pre.clone()->u8]
    // with Methods & Self Return Type
    [mut monce_pre_self.clone()]

/* -------------------------------- Constant -------------------------------- */
    // Base
    [constant=50]

/* ------------------------------- Lazy TyMap ------------------------------- */
    // Base
    [map_lazy<u8>{[(2,1)]}]
    // with Methods - Pre-Block
    [map_lazy_pre<u8>.clone()->u8{[(2,1)]}]
    // with Methods - Post-Block
    [map_lazy_post<u8>{[(2,1)]}.clone()->u8]
    // with Methods & Self Return Type - Pre-Block
    [map_lazy_pre_self<u8>.clone(){[(2,1)]}]
    // with Methods & Self Return Type - Post-Block
    [map_lazy_post_self<u8>{[(2,1)]}.clone()]

/* --------------------------- Mutable Lazy TyMap --------------------------- */
    // Base
    [mut mmap_lazy<u8>{[(2,1)]}]
    // with Methods - Pre-Block
    [mut mmap_lazy_pre<u8>.clone()->u8{[(2,1)]}]
    // with Methods - Post-Block
    [mut mmap_lazy_post<u8>{[(2,1)]}.clone()->u8]
    // with Methods & Self Return Type - Pre-Block
    [mut mmap_lazy_pre_self<u8>.clone(){[(2,1)]}]
    // with Methods & Self Return Type - Post-Block
    [mut mmap_lazy_post_self<u8>{[(2,1)]}.clone()]

/* ------------------------------- Once TyMap ------------------------------- */
    // Base
    [map_once<u8>]
    // with Methods
    [map_once_pre<u8>.clone()->u8]
    // with Methods & Self Return Type
    [map_once_pre_self<u8>.clone()]

/* --------------------------- Mutable Once TyMap --------------------------- */
    // Base
    [mut mmap_once<u8>]
    // with Methods
    [mut mmap_once_pre<u8>.clone()->u8]
    // with Methods & Self Return Type
    [mut mmap_once_pre_self<u8>.clone()]

/* -------------------------------- Lazy Vec -------------------------------- */
    // Base
    [vec_lazy<>{[1]}]
    // with Methods - Pre-Block
    [vec_lazy_pre<>.clone()->u8{[1]}]
    // with Methods - Post-Block
    [vec_lazy_post<>{[1]}.clone()->u8]
    // with Methods & Self Return Type - Pre-Block
    [vec_lazy_pre_self<>.clone(){[1]}]
    // with Methods & Self Return Type - Post-Block
    [vec_lazy_post_self<>{[1]}.clone()]

/* ---------------------------- Mutable Lazy Vec ---------------------------- */
    // Base
    [mut mvec_lazy<>{[1]}]
    // with Methods - Pre-Block
    [mut mvec_lazy_pre<>.clone()->u8{[1]}]
    // with Methods - Post-Block
    [mut mvec_lazy_post<>{[1]}.clone()->u8]
    // with Methods & Self Return Type - Pre-Block
    [mut mvec_lazy_pre_self<>.clone(){[1]}]
    // with Methods & Self Return Type - Post-Block
    [mut mvec_lazy_post_self<>{[1]}.clone()]

/* -------------------------------- Once Vec -------------------------------- */
    // Base
    [vec_once<>]
    // with Methods
    [vec_once_pre<>.clone()->u8]
    // with Methods & Self Return Type
    [vec_once_pre_self<>.clone()]

/* ---------------------------- Mutable Once Vec ---------------------------- */
    // Base
    [mut mvec_once<>]
    // with Methods
    [mut mvec_once_pre<>.clone()->u8]
    // with Methods & Self Return Type
    [mut mvec_once_pre_self<>.clone()]

}

fn main () {}
//! Macro to 'attach' values statically to a type using getter/setter methods. 
pub use once_cell;
pub use paste;

#[macro_export]
macro_rules! type_cell {
/* -------------------------------- Read Only ------------------------------- */

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: once!
        set $sbname:ident(..);
        $(set $smname:ident($smmain:ty): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),*);)*
        get $gbname:ident();
        $(get $gname:ident() -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{type_cell::paste::paste!{
        static [<T Y C E _ $sbname:upper _ $on:upper>]: type_cell::once_cell::sync::OnceCell<$store> = type_cell::once_cell::sync::OnceCell::new();
        pub trait [<TypeCell $sbname:camel $on:camel>] {
            // Set
            fn $sbname (set:impl Into<$store>);
            $(fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*);)*
            $(fn $sfname ($($sfvar:$sfvarty),*);)*
            // Get
            fn $gbname () -> &'static $store;
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $sbname:camel $on:camel>] for $on<$($gen),*> {
            // Set
            fn $sbname (set:impl Into<$store>)
                {[<T Y C E _ $sbname:upper _ $on:upper>].set(set.into()).unwrap();}
            $(fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*) 
                {[<T Y C E _ $sbname:upper _ $on:upper>].set(set$(.$smeth($($smvar),*  $($sconst),*))*).unwrap();})*
            $(fn $sfname ($($sfvar:$sfvarty),*)
                {[<T Y C E _ $sbname:upper _ $on:upper>].set($sfname($($sfvar),*)).unwrap();})*
            // Get  
            fn $gbname () -> &'static $store 
                {[<T Y C E _ $sbname:upper _ $on:upper>].wait()}
            $(fn $gfname () -> $gfret 
                {$gfname([<T Y C E _ $sbname:upper _ $on:upper>].wait())})*
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                [<T Y C E _ $sbname:upper _ $on:upper>].wait()$(.$gmeth($($gvar),*  $($gconst),*))*
            })*
        }
    }};

/* ----------------------------- unsafe! Mutable ----------------------------- */

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: unsafe!
        set $sbname:ident(..);
        $(set $smname:ident($smmain:ty): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),*);)*
        get $gbname:ident();
        $(get $gname:ident() -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{type_cell::paste::paste!{
        static mut [<T Y C E _ $sbname:upper _ $on:upper>]: Option<$store> = None;
        pub trait [<TypeCell $sbname:camel $on:camel>] {
            // Set
            unsafe fn $sbname (set:impl Into<$store>);
            $(unsafe fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*);)*
            $(unsafe fn $sfname ($($sfvar:$sfvarty),*);)*
            // Get
            unsafe fn $gbname () -> &'static mut $store;
            $(unsafe fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(unsafe fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $sbname:camel $on:camel>] for $on<$($gen),*> {
            // Set
            unsafe fn $sbname (set:impl Into<$store>)
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>] = Some(set.into());}}
            $(unsafe fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*) 
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>] = Some(set$(.$smeth($($smvar),*  $($sconst),*))*);}})*
            $(unsafe fn $sfname ($($sfvar:$sfvarty),*)
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>] = Some($sfname($($sfvar),*));}})*
            // Get
            unsafe fn $gbname () -> &'static mut $store {
                if let Some(o) = unsafe {&mut [<T Y C E _ $sbname:upper _ $on:upper>]}
                    {o} else {panic!()}
            }
            $(unsafe fn $gfname () -> $gfret {
                if let Some(o) = unsafe {&mut [<T Y C E _ $sbname:upper _ $on:upper>]}
                {$gfname(o)} else {panic!()}
            })*
            $(unsafe fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                if let Some(o) = unsafe {&mut [<T Y C E _ $sbname:upper _ $on:upper>]}
                {o$(.$gmeth($($gvar),*  $($gconst),*))*} else {panic!()}
            })*
        }
    }};

/* ------------------------------ risky! Mutable ----------------------------- */

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: risky!
        set $sbname:ident(..);
        $(set $smname:ident($smmain:ty): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),*);)*
        get $gbname:ident();
        $(get $gname:ident() -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{type_cell::paste::paste!{
        static mut [<T Y C E _ $sbname:upper _ $on:upper>]: Option<$store> = None;
        pub trait [<TypeCell $sbname:camel $on:camel>] {
            // Set
            fn $sbname (set:impl Into<$store>);
            $(fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*);)*
            $(fn $sfname ($($sfvar:$sfvarty),*);)*
            // Get
            fn $gbname () -> &'static mut $store;
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $sbname:camel $on:camel>] for $on<$($gen),*> {
            // Set
            fn $sbname (set:impl Into<$store>)
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>] = Some(set.into());}}
            $(fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*) 
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>] = Some(set$(.$smeth($($smvar),*  $($sconst),*))*);}})*
            $(fn $sfname ($($sfvar:$sfvarty),*)
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>] = Some($sfname($($sfvar),*));}})*
            // Get
            fn $gbname () -> &'static mut $store {
                if let Some(o) = unsafe {&mut [<T Y C E _ $sbname:upper _ $on:upper>]}
                    {o} else {panic!()}
            }
            $(fn $gfname () -> $gfret {
                if let Some(o) = unsafe {&mut [<T Y C E _ $sbname:upper _ $on:upper>]}
                {$gfname(o)} else {panic!()}
            })*
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                if let Some(o) = unsafe {&mut [<T Y C E _ $sbname:upper _ $on:upper>]}
                {o$(.$gmeth($($gvar),* $($gconst),*))*} else {panic!()}
            })*
        }
    }};

/* ---------------------------------- lazy! ---------------------------------- */

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: lazy!
        set $lazy:block
        get $gbname:ident();
        $(get $gname:ident(..) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{type_cell::paste::paste!{
        static [<T Y C E _ $gbname:upper _ $on:upper>]: type_cell::once_cell::sync::Lazy<$store> = type_cell::once_cell::sync::Lazy::new(||$lazy);
        pub trait [<TypeCell $gbname:camel $on:camel>] {
            fn $gbname () -> &'static $store;
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $gbname:camel $on:camel>] for $on<$($gen),*> {
            fn $gbname () -> &'static $store 
                {&*[<T Y C E _ $gbname:upper _ $on:upper>]}
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                (&*[<T Y C E _ $gbname:upper _ $on:upper>])$(.$gmeth($($gvar),* $($gconst),*))*
            })*
            $(fn $gfname () -> $gfret 
                {$gfunc(&*[<T Y C E _ $gbname:upper _ $on:upper>])})*
        }
    }};

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: lazy!
        set $lazy:block
        get $gbname:ident(..) -> $gbret:ty: static$(.$gbmeth:ident( $($gbvar:ident:$gbvarty:ty),* $(=$gbconst:expr),*))*;
        $(get $gname:ident(..) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{type_cell::paste::paste!{
        static [<T Y C E _ $gbname:upper _ $on:upper>]: type_cell::once_cell::sync::Lazy<$store> = type_cell::once_cell::sync::Lazy::new(||$lazy);
        pub trait [<TypeCell $gbname:camel $on:camel>] {
            fn $gbname ($($($gbvar:$gbvarty),*)*) -> $gbret;
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $gbname:camel $on:camel>] for $on<$($gen),*> {
            fn $gbname ($($($gbvar:$gbvarty),*)*) -> $gbret {
                (&*[<T Y C E _ $gbname:upper _ $on:upper>])$(.$gbmeth($($gbvar),* $($gbconst),*))*
            }
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                (&*[<T Y C E _ $gbname:upper _ $on:upper>])$(.$gmeth($($gvar),* $($gconst),*))*
            })*
            $(fn $gfname () -> $gfret 
                {$gfunc(&*[<T Y C E _ $gbname:upper _ $on:upper>])})*
        }
    }};
    
/* -------------------------------------------------------------------------- */
/*                                  Variation                                 */
/* -------------------------------------------------------------------------- */

    ($on:ident{
        static $store:ty: $opt:ident!
        set $block:block
        get $gbname:ident(..) -> $gbret:ty: static$(.$gbmeth:ident( $($gbvar:ident:$gbvarty:ty),* $(=$gbconst:expr),*))*;
        $(get $gname:ident(..) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{
        type_cell!{$on<>{
            static $store: $opt!
            set $block
            get $gbname(..) -> $gbret: static$(.$gbmeth($($gbvar:$gbvarty),*  $(=$gbconst),*))*;
            $(get $gname(..) -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname(..) -> $gfret;)*
        }}
    };

    ($on:ident{
        static $store:ty: $opt:ident!
        set $block:block
        get $gbname:ident();
        $(get $gname:ident(..) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{
        type_cell!{$on<>{
            static $store: $opt!
            set $block
            get $gbname();
            $(get $gname(..) -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname(..) -> $gfret;)*
        }}
    };

    ($on:ident{
        static $store:ty: $opt:ident!
        set $sbname:ident(..);
        $(set $smname:ident($smmain:ty): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),*);)*
        get $gbname:ident();
        $(get $gname:ident() -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident(..) -> $gfret:ty;)*
    })=>{
        type_cell!{$on<>{
            static $store: $opt!
            set $sbname(..);
            $(set $smname($smmain): do$(.$smeth($($smvar:$smvarty),* $(=$sconst),*))*;)*
            $(set =$sfname($($sfvar:$sfvarty),*);)*
            get $gbname();
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname(..) -> $gfret;)*
        }}
    };

/* --------------------------------- Simple --------------------------------- */

    ($($opt:ident! $on:ty > $ty:ty: $($name:ident),*;)*)=>{type_cell::paste::paste!{
        $($(type_cell!{ $on {
            static $ty: $opt!
            set [<set_ $name>](..);
            get [<get_ $name>]();
        }})*)*
    }}; 

    ($($opt:ident! $on:ty: $($name:ident),*;)*)=>{type_cell::paste::paste!{
        type_cell!{ $( $opt! $on > $on: $($name),*;)*}
    }};

/* ------------------------------------ - ----------------------------------- */
}
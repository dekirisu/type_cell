//! Macro to 'attach' values statically to a type using getter/setter methods. 
pub use once_cell::sync::{OnceCell,Lazy};
pub use paste::paste;


#[macro_export]
macro_rules! tycell {
/* -------------------------------- Read Only ------------------------------- */

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: once_read;
        set $sbname:ident($($_0:tt),*);
        $(set $smname:ident($smmain:ty $(,$_1:tt)*): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),* $(,$_2:tt)*);)*
        $(get $gname:ident($($_3:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_4:tt),*) -> $gfret:ty;)*
    })=>{paste!{
        static [<T Y C E _ $sbname:upper _ $on:upper>]: OnceCell<$store> = OnceCell::new();
        pub trait [<TypeCell $sbname:camel $on:camel>] {
            // Set
            fn $sbname (set:impl Into<$store>);
            $(fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*);)*
            $(fn $sfname ($($sfvar:$sfvarty),*);)*
            // Get
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
            $(fn $gfname () -> $gfret 
                {$gfname([<T Y C E _ $sbname:upper _ $on:upper>].wait())})*
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                [<T Y C E _ $sbname:upper _ $on:upper>].wait()$(.$gmeth($($gvar),*  $($gconst),*))*
            })*
        }
    }};

/* ------------------------------ risky! Mutable ----------------------------- */

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: once_write;
        set $sbname:ident($($_0:tt),*);
        $(set $smname:ident($smmain:ty $(,$_1:tt)*): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),* $(,$_2:tt)*);)*
        $(get $gname:ident($($_3:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_4:tt),*) -> $gfret:ty;)*
    })=>{paste!{
        static mut [<T Y C E _ $sbname:upper _ $on:upper>]: OnceCell<$store> = OnceCell::new();
        pub trait [<TypeCell $sbname:camel $on:camel>] {
            // Set
            fn $sbname (set:impl Into<$store>);
            $(fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*);)*
            $(fn $sfname ($($sfvar:$sfvarty),*);)*
            // Get
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $sbname:camel $on:camel>] for $on<$($gen),*> {
            // Set
            fn $sbname (set:impl Into<$store>)
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>].set(set.into()).unwrap()};}
            $(fn $smname (set:$smmain $($(,$smvar:$smvarty)*)*) 
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>].set(set$(.$smeth($($smvar),*  $($sconst),*))*).unwrap()};})*
            $(fn $sfname ($($sfvar:$sfvarty),*)
                {unsafe{[<T Y C E _ $sbname:upper _ $on:upper>].set($sfname($($sfvar),*)).unwrap();}})*
            // Get  
            $(fn $gfname () -> $gfret 
                {$gfname(unsafe{[<T Y C E _ $sbname:upper _ $on:upper>]}.get_mut().unwrap())})*
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                unsafe{[<T Y C E _ $sbname:upper _ $on:upper>].get_mut().unwrap()$(.$gmeth($($gvar),*  $($gconst),*))*}
            })*
        }
    }};

/* ---------------------------------- lazy! ---------------------------------- */

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: lazy_read;
        set $lazy:block
        get $gbname:ident($($_0:tt),*);
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{ $on<$($gen),*> {
            static $store: lazy_read;
            set $lazy
            get $gbname() -> &'static $store: static;
            $(get $gname() -> $gret: static$(.$gmeth( $($gvar:$gvarty),* $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }
    }};


    ( $on:ident<$($gen:ty),*> {
        static $store:ty: lazy_read;
        set $lazy:block
        get $gbname:ident($($_0:tt),*) -> $gbret:ty: static$(.$gbmeth:ident( $($gbvar:ident:$gbvarty:ty),* $(=$gbconst:expr),*))*;
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{paste!{
        static [<T Y C E _ $gbname:upper _ $on:upper>]: Lazy<$store> = Lazy::new(||$lazy.into());
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

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: lazy_write;
        set $lazy:block
        get $gbname:ident($($_0:tt),*);
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{ $on<$($gen),*> {
            static $store: lazy_write;
            set $lazy
            get $gbname() -> &'static mut $store: static;
            $(get $gname() -> $gret: static$(.$gmeth( $($gvar:$gvarty),* $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }
    }};

    ( $on:ident<$($gen:ty),*> {
        static $store:ty: lazy_write;
        set $lazy:block
        get $gbname:ident($($_0:tt),*) -> $gbret:ty: static$(.$gbmeth:ident( $($gbvar:ident:$gbvarty:ty),* $(=$gbconst:expr),*))*;
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{paste!{
        static mut [<T Y C E _ $gbname:upper _ $on:upper>]: Lazy<$store> = Lazy::new(||$lazy.into());
        pub trait [<TypeCell $gbname:camel $on:camel>] {
            fn $gbname ($($($gbvar:$gbvarty),*)*) -> $gbret;
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $gbname:camel $on:camel>] for $on<$($gen),*> {
            fn $gbname ($($($gbvar:$gbvarty),*)*) -> $gbret {
                unsafe{(&mut *[<T Y C E _ $gbname:upper _ $on:upper>])$(.$gbmeth($($gbvar),* $($gbconst),*))*}
            }
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                unsafe{(&mut *[<T Y C E _ $gbname:upper _ $on:upper>])$(.$gmeth($($gvar),* $($gconst),*))*}
            })*
            $(fn $gfname () -> $gfret 
                {unsafe{$gfunc(&mut *[<T Y C E _ $gbname:upper _ $on:upper>])}
            })*
        }
    }};

/* ---------------------------------- const --------------------------------- */
    
    ( $on:ident<$($gen:ty),*> {
        const $store:ty = $lazy:expr;
        get $gbname:ident($($_0:tt),*);
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{ $on<$($gen),*> {
            const $store = $lazy;
            get $gbname() -> &'static $store: static;
            $(get $gname() -> $gret: static$(.$gmeth( $($gvar:$gvarty),* $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }
    }};

    ( $on:ident<$($gen:ty),*> {
        const $store:ty = $const:expr;
        get $gbname:ident($($_0:tt),*) -> $gbret:ty: static$(.$gbmeth:ident( $($gbvar:ident:$gbvarty:ty),* $(=$gbconst:expr),*))*;
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident( $($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{paste!{
        const [<T Y C E _ $gbname:upper _ $on:upper>]: $store = $const;
        pub trait [<TypeCell $gbname:camel $on:camel>] {
            fn $gbname ($($($gbvar:$gbvarty),*)*) -> $gbret;
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret;)*
            $(fn $gfname () -> $gfret;)*
        }
        impl [<TypeCell $gbname:camel $on:camel>] for $on<$($gen),*> {
            fn $gbname ($($($gbvar:$gbvarty),*)*) -> $gbret {
                (&[<T Y C E _ $gbname:upper _ $on:upper>])$(.$gbmeth($($gbvar),* $($gbconst),*))*
            }
            $(fn $gname ($($($gvar:$gvarty),*)*) -> $gret {
                (&[<T Y C E _ $gbname:upper _ $on:upper>])$(.$gmeth($($gvar),* $($gconst),*))*
            })*
            $(fn $gfname () -> $gfret 
                {$gfunc([&<T Y C E _ $gbname:upper _ $on:upper>])})*
        }
    }};
/* -------------------------------------------------------------------------- */
/*                                  Variation                                 */
/* -------------------------------------------------------------------------- */

    ($on:ident{
        const $store:ty = $const:expr;
        get $gbname:ident($($tt:tt)* $(,$_0:tt)*) -> $gbret:ty: static$(.$gbmeth:ident( $($gbvar:ident:$gbvarty:ty),* $(=$gbconst:expr),*))*;
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<>{
            const $store = $const;
            get $gbname() -> $gbret: static$(.$gbmeth($($gbvar:$gbvarty),*  $(=$gbconst),*))*;
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

    ($on:ident{
        const $store:ty = $const:expr;
        get $gbname:ident($($_0:tt),*);
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<>{
            const $store = $const;
            get $gbname();
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

    ($on:ident{
        static $store:ty: $opt:ident;
        set $block:block
        get $gbname:ident($($tt:tt)* $(,$_0:tt)*) -> $gbret:ty: static$(.$gbmeth:ident( $($gbvar:ident:$gbvarty:ty),* $(=$gbconst:expr),*))*;
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<>{
            static $store: $opt;
            set $block
            get $gbname() -> $gbret: static$(.$gbmeth($($gbvar:$gbvarty),*  $(=$gbconst),*))*;
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

    ($on:ident{
        static $store:ty: $opt:ident;
        set $block:block
        get $gbname:ident($($_0:tt),*);
        $(get $gname:ident($($_1:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_2:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<>{
            static $store: $opt;
            set $block
            get $gbname();
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

    ($on:ident{
        static $store:ty: $opt:ident;
        set $sbname:ident($($_0:tt),*);
        $(set $smname:ident($smmain:ty $(,$_1:tt)*): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),* $(,$_2:tt)*);)*
        $(get $gname:ident($($_3:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_4:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<>{
            static $store: $opt;
            set $sbname();
            $(set $smname($smmain): do$(.$smeth($($smvar:$smvarty),* $(=$sconst),*))*;)*
            $(set =$sfname($($sfvar:$sfvarty),*);)*
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

    ($on:ident{
        static $store:ty: $opt:ident;
        set $sbname:ident($($_0:tt),*);
        $(set $smname:ident($smmain:ty $(,$_1:tt)*): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),* $(,$_2:tt)*);)*
        get $gbname:ident($($_3:tt),*);
        $(get $gname:ident($($_4:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_5:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<>{
            static $store: $opt;
            set $sbname();
            $(set $smname($smmain): do$(.$smeth($($smvar:$smvarty),* $(=$sconst),*))*;)*
            $(set =$sfname($($sfvar:$sfvarty),*);)*
            get $gbname();
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

/* -------------------------------- Specifics ------------------------------- */

    ($on:ident<$($gen:ty),*> {
        static $store:ty: once_read;
        set $sbname:ident($($_0:tt),*);
        $(set $smname:ident($smmain:ty $(,$_1:tt)*): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),* $(,$_2:tt)*);)*
        get $gbname:ident($($_3:tt),*);
        $(get $gname:ident($($_4:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_5:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<$($gen),*>{
            static $store: once_read;
            set $sbname();
            $(set $smname($smmain): do$(.$smeth($($smvar:$smvarty),* $(=$sconst),*))*;)*
            $(set =$sfname($($sfvar:$sfvarty),*);)*
            get $gbname() -> &'static $store:static;
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

    ($on:ident<$($gen:ty),*> {
        static $store:ty: once_write;
        set $sbname:ident($($_0:tt),*);
        $(set $smname:ident($smmain:ty $(,$_1:tt)*): do$(.$smeth:ident($($smvar:ident:$smvarty:ty),* $(=$sconst:expr),*))*; )*
        $(set =$sfname:ident($($sfvar:ident:$sfvarty:ty),* $(,$_2:tt)*);)*
        get $gbname:ident($($_3:tt),*);
        $(get $gname:ident($($_4:tt),*) -> $gret:ty: static$(.$gmeth:ident($($gvar:ident:$gvarty:ty),* $(=$gconst:expr),*))*; )*
        $(get =$gfname:ident($($_5:tt),*) -> $gfret:ty;)*
    })=>{
        tycell!{$on<$($gen),*>{
            static $store: once_write;
            set $sbname();
            $(set $smname($smmain): do$(.$smeth($($smvar:$smvarty),* $(=$sconst),*))*;)*
            $(set =$sfname($($sfvar:$sfvarty),*);)*
            get $gbname() -> &'static mut $store:static;
            $(get $gname() -> $gret: static$(.$gmeth($($gvar:$gvarty),*  $(=$gconst),*))*; )*
            $(get =$gfname() -> $gfret;)*
        }}
    };

/* --------------------------------- Simple --------------------------------- */

    // quick lazy
    (=$on:ty>$ty:ty: $name:ident $lazy:block)=>{paste!{
        tycell!{ $on {
            static $ty: lazy_read;
            set $lazy
            get $name();
        }}
    }};  

    // quick lazy mut
    (=$on:ty>$ty:ty: mut $name:ident $lazy:block)=>{paste!{
        tycell!{ $on {
            static $ty: lazy_write;
            set $lazy
            get $name();
        }}
    }};  

    // quick once
    (=$on:ty>$ty:ty: $name:ident)=>{paste!{
        tycell!{ $on {
            static $ty: once_read;
            set [<set_ $name>]();
            get $name();
        }}
    }};  

    // quick once mut
    (=$on:ty>$ty:ty: mut $name:ident)=>{paste!{
        tycell!{ $on {
            static $ty: once_write;
            set [<set_ $name>]();
            get $name();
        }}
    }}; 

    // quick const
    (=$on:ty>$ty:ty: $name:ident = $val:expr)=>{paste!{
        tycell!{ $on {
            const $ty = $val;
            get $name();
        }}
    }}; 

/* ----------------------------- Simple HashMap ----------------------------- */

    // quick lazy
    (=$on:ty>$ty:ty: $name:ident <$key:ty> $lazy:block)=>{paste!{
        tycell!{ $on {
            static TyMap<$key,$ty>: lazy_read;
            set $lazy
            get [<$name _map>]();
            get $name() -> &'static $ty: static.get(id:&$key).unwrap();
        }}
    }};  

    // quick lazy mut
    (=$on:ty>$ty:ty: mut $name:ident <$key:ty> $lazy:block)=>{paste!{
        tycell!{ $on {
            static TyMap<$key,$ty>: lazy_write;
            set $lazy
            get [<$name _map>]();
            get $name() -> &'static $ty: static.get(id:&$key).unwrap();
            get [<$name _mut>]() -> &'static mut $ty: static.get_mut(id:&$key).unwrap();
        }}
    }};  

    // quick once
    (=$on:ty>$ty:ty: $name:ident <$key:ty>)=>{paste!{
        tycell!{ $on {
            static TyMap<$key,$ty>: once_read;
            set [<set_ $name>]();
            get [<$name _map>]();
            get $name() -> &'static $ty: static.get(id:&$key).unwrap();
        }}
    }};  

    // quick once mut
    (=$on:ty>$ty:ty: mut $name:ident <$key:ty>)=>{paste!{
        tycell!{ $on {
            static TyMap<$key,$ty>: once_write;
            set [<set_ $name>]();
            get [<$name _map>]();
            get $name() -> &'static $ty: static.get(id:&$key).unwrap();
            get [<$name _mut>]() -> &'static mut $ty: static.get_mut(id:&$key).unwrap();
        }}
    }}; 

/* ------------------------------ Simple Merge ------------------------------ */

    // quick
    (=$on:ty:$($full:tt)*)=>{paste!{
        tycell!{=$on>$on:$($full)*}
    }};  

    // merged
    ($($on:ty > $ty:ty: $([$($name:tt)*])*;)*)=>{paste!{
        $($(tycell!{  =$on>$ty:$($name)* })*)*
    }}; 

    // minimal
    ($($on:ty: $([$($name:tt)*])*;)*)=>{paste!{
        $($(tycell!{  =$on>$on:$($name)* })*)*
    }}; 

/* ------------------------------------ - ----------------------------------- */
}
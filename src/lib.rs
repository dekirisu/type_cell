//! Macro to attach [OnceCell] to a Type using getter/setter methods. This is mainly useful for variables which will be set at the start and be accessed read-only.
//! Initially developed for use with the Bevy-Engine to easily access Handles (smart pointers to assets) globally.
//! 
//! - The value can only be set once.
//! - The value has to be set before getting.

pub use once_cell::sync::OnceCell;
pub use paste::paste;

#[macro_export]
macro_rules! type_cell {
    
    /* ---------------------------------- Base ---------------------------------- */
    (on $on:ty > store $store:ty 
    | set$(.$sact:ident($($svar:ident:$svarty:ty),*))* $set:ty 
    | get$(.$act:ident($($var:ident:$varty:ty),*))* $get:ty 
        $(, $gname:ident get$(.$act2:ident($($var2:ident:$varty2:ty),*))* $get2:ty )*
    | $name:ident)=>{paste!{
        static [<T Y C E _ $name:upper _ $on:upper>]: OnceCell<$store> = OnceCell::new();
        pub trait [<TypeCell $name:camel $on:camel>] {
            fn [<set_ $name:snake>] (a: $set $($(,$svar:$svarty)*)*);
            fn [<get_ $name:snake>] ($($($var:$varty),*)*) -> $get;
            $(
                fn [<get_ $name:snake _ $gname>] ($($($var2:$varty2),*)*) -> $get2;
            )*
        }
        impl [<TypeCell $name:camel $on:camel>] for $on {
            fn [<set_ $name:snake>] (a: $set $($(,$svar:$svarty)*)*) 
                {[<T Y C E _ $name:upper _ $on:upper>].set(a$(.$sact($($svar),*))*).unwrap();}
            fn [<get_ $name:snake>] ($($($var:$varty),*)*) -> $get 
                {[<T Y C E _ $name:upper _ $on:upper>].wait()$(.$act($($var),*))*}
            $(
                fn [<get_ $name:snake _ $gname>] ($($($var2:$varty2),*)*) -> $get2
                    {[<T Y C E _ $name:upper _ $on:upper>].wait()$(.$act2($($var2),*))*}
            )*
        }
    }};

    /* --------------------------------- Presets -------------------------------- */
    // Single
    ($( $ty:ty: $($name:ident),*;)*)=>{
        $($(type_cell!(on $ty > store $ty | set $ty | get &'static $ty | $name);)*)*
    };
    (#clone $($ty:ty: $($name:ident),*;)*)=>{
        $($( type_cell!(on $ty > store $ty | set $ty | get.clone() $ty | $name);)*)*
    };

    // Single
    ($( $on:ty > $ty:ty: $($name:ident),*;)*)=>{
        $($(type_cell!(on $on > store $ty | set $ty | get &'static $ty | $name);)*)*
    };
    (#clone $($on:ty > $ty:ty: $($name:ident),*;)*)=>{
        $($( type_cell!(on $on > store $ty | set $ty | get.clone() $ty | $name);)*)*
    };

    // Vec
    (@Vec $( $on:ty > $ty:ty: $($name:ident),*;)*)=>{$(
        $(type_cell!(on $on > store Vec<$ty> | set Vec<$ty> | get.get(id:usize) Option<&'static $ty>, vec get &'static Vec<$ty> | $name);)*
    )*};
    (@Vec #unwrap $( $on:ty > $ty:ty: $($name:ident),*;)*)=>{$(
        $(type_cell!(on $on > store Vec<$ty> | set Vec<$ty> | get.get(id:usize).unwrap() &'static $ty, vec get &'static Vec<$ty> | $name);)*
    )*};
    (@Vec #unwrap #clone $( $on:ty > $ty:ty: $($name:ident),*;)*)=>{$(
        $(type_cell!(on $on > store Vec<$ty> | set Vec<$ty> | get.get(id:usize).unwrap().clone() $ty, vec get &'static Vec<$ty> | $name);)*
    )*};

    (@Vec $( $ty:ty: $($name:ident),*;)*)=>{
        type_cell!(@Vec $( $ty > $ty: $($name),*;)*);
    };
    (@Vec #unwrap $( $ty:ty: $($name:ident),*;)*)=>{
        type_cell!(@Vec #unwrap $( $ty > $ty: $($name),*;)*);
    };
    (@Vec #unwrap #clone $( $ty:ty: $($name:ident),*;)*)=>{
        type_cell!(@Vec #unwrap #clone $( $ty > $ty: $($name),*;)*);
    };

    // HashMap
    (@HashMap<$id:ty> $( $on:ty > $ty:ty: $($name:ident),*;)*)=>{$(
        $(type_cell!(on $on > store HashMap<$id,$ty> | set HashMap<$id,$ty> | get.get(id:&$id) Option<&'static $ty>, map get &'static HashMap<$id,$ty> | $name);)*
    )*};
    (@HashMap<$id:ty> #unwrap $( $on:ty > $ty:ty: $($name:ident),*;)*)=>{$(
        $(type_cell!(on $on > store HashMap<$id,$ty> | set HashMap<$id,$ty> | get.get(id:&$id).unwrap() &'static $ty, map get &'static HashMap<$id,$ty> | $name);)*
    )*};
    (@HashMap<$id:ty> #unwrap #clone $( $on:ty > $ty:ty: $($name:ident),*;)*)=>{$(
        $(type_cell!(on $on > store HashMap<$id,$ty> | set HashMap<$id,$ty> | get.get(id:&$id).unwrap().clone() $ty, map get &'static HashMap<$id,$ty> | $name);)*
    )*};

    (@HashMap<$id:ty> $( $ty:ty: $($name:ident),*;)*)=>{
        type_cell!(@HashMap<$id> $( $ty > $ty: $($name),*;)*);
    };
    (@HashMap<$id:ty> #unwrap $( $ty:ty: $($name:ident),*;)*)=>{
        type_cell!(@HashMap<$id> #unwrap $( $ty > $ty: $($name),*;)*);
    };
    (@HashMap<$id:ty> #unwrap #clone $( $ty:ty: $($name:ident),*;)*)=>{
        type_cell!(@HashMap<$id> #unwrap #clone $( $ty > $ty: $($name),*;)*);
    };

}
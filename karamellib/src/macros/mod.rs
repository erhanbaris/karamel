#[macro_export] 
macro_rules! pop {
    ($options: expr) => {{
        pop_raw!($options).deref()
    }}
}

#[macro_export] 
macro_rules! pop_raw {
    ($options: expr) => {{
        (*$options.current_scope).stack_ptr = (*$options.current_scope).stack_ptr.sub(1);
        karamel_dbg!((*(*$options.current_scope).stack_ptr))
    }}
}

#[macro_export] 
macro_rules! fetch_raw {
    ($options: expr) => {{
        karamel_dbg!(*(*$options.current_scope).stack_ptr.sub(1))
    }}
}

#[macro_export] 
macro_rules! current_raw {
    ($options: expr) => {{
        karamel_dbg!(*(*$options.current_scope).stack_ptr)
    }}
}

#[macro_export] 
macro_rules! get_memory_index {
    ($options: expr) => {{
        karamel_dbg!(((*$options.current_scope).stack_ptr.offset_from((*$options.current_scope).stack.as_mut_ptr())))
    }}
}

#[macro_export] 
macro_rules! inc_memory_index {
    ($options: expr, $count: expr) => {{
        (*$options.current_scope).stack_ptr = karamel_dbg!((*$options.current_scope).stack_ptr.add($count));
    }}
}

#[macro_export] 
macro_rules! dec_memory_index {
    ($options: expr, $count: expr) => {{
        (*$options.current_scope).stack_ptr = (*$options.current_scope).stack_ptr.sub($count);
    }}
}

// The debug version
#[allow(dead_code)]
#[macro_export]
#[cfg(not(feature = "unittest"))]
macro_rules! debug_println {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[allow(dead_code)]
#[macro_export]
#[cfg(feature = "unittest")]
macro_rules! debug_println {
    ($( $args:expr ),*) => {}
}

/*
DEBUG MACROS
*/

#[macro_export] 
#[cfg(any(feature = "dbg", feature = "dbg_level1", feature = "dbg_level2", feature = "dbg_level3"))]
macro_rules! karamel_dbg {
    ($x:expr) => { dbg!($x) }
}

#[macro_export] 
#[cfg(all(not(feature = "dbg"), not(feature = "dbg_level1"), not(feature = "dbg_level2"), not(feature = "dbg_level3")))]
macro_rules! karamel_dbg {
    ($x:expr) => { std::convert::identity($x) }
}

#[macro_export] 
#[cfg(feature = "dbg_level1")]
macro_rules! karamel_dbg_level1 {
    ($x:expr) => { dbg!($x) }
}

#[macro_export] 
#[cfg(not(feature = "dbg_level1"))]
macro_rules! karamel_dbg_level1 {
    ($x:expr) => { std::convert::identity($x) }
}

#[macro_export] 
#[cfg(feature = "dbg_level2")]
macro_rules! karamel_dbg_level2 {
    ($x:expr) => { dbg!($x) }
}

#[macro_export] 
#[cfg(not(feature = "dbg_level2"))]
macro_rules! karamel_dbg_level2 {
    ($x:expr) => { std::convert::identity($x) }
}


#[macro_export] 
#[cfg(feature = "dbg_level3")]
macro_rules! karamel_dbg_level3 {
    ($x:expr) => { dbg!($x) }
}

#[macro_export] 
#[cfg(not(feature = "dbg_level3"))]
macro_rules! karamel_dbg_level3 {
    ($x:expr) => { std::convert::identity($x) }
}

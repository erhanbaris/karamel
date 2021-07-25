#[macro_export] 
macro_rules! pop {
    ($options: expr) => {{
        pop_raw!($options).deref()
    }}
}

#[macro_export] 
macro_rules! pop_raw {
    ($options: expr) => {{
        $options.stack_ptr = $options.stack_ptr.sub(1);
        karamel_dbg!(*$options.stack_ptr)
    }}
}

#[macro_export] 
macro_rules! fetch_raw {
    ($options: expr) => {{
        karamel_dbg!(*$options.stack_ptr.sub(1))
    }}
}

#[macro_export] 
macro_rules! current_raw {
    ($options: expr) => {{
        karamel_dbg!(*$options.stack_ptr)
    }}
}

#[macro_export] 
macro_rules! get_memory_index {
    ($options: expr) => {{
        karamel_dbg!($options.stack_ptr.offset_from($options.stack.as_ptr()))
    }}
}

#[macro_export] 
macro_rules! inc_memory_index {
    ($options: expr, $count: expr) => {{
        $options.stack_ptr = karamel_dbg!($options.stack_ptr.add($count));
    }}
}

#[macro_export] 
macro_rules! dec_memory_index {
    ($options: expr, $count: expr) => {{
        $options.stack_ptr = $options.stack_ptr.sub($count);
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
#[cfg(any(feature = "dbg"))]
macro_rules! karamel_dbg {
    ($x:expr) => { dbg!($x) }
}

#[macro_export] 
#[cfg(not(feature = "dbg"))]
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

#[macro_export] 
#[cfg(feature = "dbg_level1")]
macro_rules! karamel_print_level1 {
    ($($arg:tt)*) => (println!("DEBUG1: {}", std::format_args!($($arg)*)));
}

#[macro_export] 
#[cfg(not(feature = "dbg_level1"))]
macro_rules! karamel_print_level1 {
    ($($arg:tt)*) => { }
}

#[macro_export] 
#[cfg(feature = "dbg_level2")]
macro_rules! karamel_print_level2 {
    ($($arg:tt)*) => (println!("DEBUG2: {}", std::format_args!($($arg)*)));
}

#[macro_export] 
#[cfg(not(feature = "dbg_level2"))]
macro_rules! karamel_print_level2 {
    ($($arg:tt)*) => { }
}


#[macro_export] 
#[cfg(feature = "dbg_level3")]
macro_rules! karamel_print_level3 {
    ($($arg:tt)*) => (println!("DEBUG3: {}", std::format_args!($($arg)*)));
}

#[macro_export] 
#[cfg(not(feature = "dbg_level3"))]
macro_rules! karamel_print_level3 {
    ($($arg:tt)*) => { }
}

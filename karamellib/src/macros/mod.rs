#[macro_export] 
macro_rules! current_memory_index {
    ($context: expr) => {{
        $context.stack_ptr.offset_from($context.stack.as_ptr())
    }}
}

#[macro_export]
#[cfg(any(feature = "dbg", feature = "dbg_level3"))]
macro_rules! dump_data {
    ($context: expr, $message: expr) => {{
        let location = current_memory_index!($context);
        println!("stack[{}] = {} ({:?})", location, *$context.stack_ptr, $message);
    }}
}

#[macro_export] 
#[cfg(any(feature = "dbg", feature = "dbg_level1", feature = "dbg_level2", feature = "dbg_level3"))]
macro_rules! karamel_dbg_any {
    ($x:expr) => { dbg!($x) }
}

#[macro_export] 
#[cfg(all(not(feature = "dbg"), not(feature = "dbg_level1"), not(feature = "dbg_level2"), not(feature = "dbg_level3")))]
macro_rules! karamel_dbg_any {
    ($x:expr) => { std::convert::identity($x) }
}

#[macro_export]
#[cfg(all(not(feature = "dbg"), not(feature = "dbg_level3")))]
macro_rules! dump_data {
    ($context: expr, $message: expr) => { }
}

#[macro_export] 
macro_rules! pop {
    ($context: expr, $message: expr) => {{
        pop_raw!($context, $message).deref()
    }}
}

#[macro_export] 
macro_rules! pop_raw {
    ($context: expr, $message: expr) => {{
        $context.stack_ptr = $context.stack_ptr.sub(1);
        dump_data!($context, $message);
        *$context.stack_ptr
    }}
}

#[macro_export] 
macro_rules! fetch_raw {
    ($context: expr) => {{
        karamel_dbg!(*$context.stack_ptr.sub(1))
    }}
}

#[macro_export] 
macro_rules! current_raw {
    ($context: expr) => {{
        karamel_dbg!(*$context.stack_ptr)
    }}
}

#[macro_export] 
macro_rules! get_memory_index {
    ($context: expr) => {{
        karamel_dbg!($context.stack_ptr.offset_from($context.stack.as_ptr()))
    }}
}

#[macro_export] 
macro_rules! inc_memory_index {
    ($context: expr, $count: expr) => {{
        $context.stack_ptr = karamel_dbg!($context.stack_ptr.add($count));
    }}
}

#[macro_export] 
macro_rules! dec_memory_index {
    ($context: expr, $count: expr) => {{
        $context.stack_ptr = $context.stack_ptr.sub($count);
    }}
}

// The debug version
#[allow(dead_code)]
#[macro_export]
#[cfg(not(feature = "unittest"))]
macro_rules! debug_println {
    ($( $args:expr ),*) => { log::info!( $( $args ),* ); }
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
#[cfg(any(feature = "dbg", feature = "dbg_level1"))]
macro_rules! karamel_print_level1 {
    ($($arg:tt)*) => (println!("DEBUG1: {}", std::format_args!($($arg)*)));
}

#[macro_export] 
#[cfg(all(not(feature = "dbg"), not(feature = "dbg_level1")))]
macro_rules! karamel_print_level1 {
    ($($arg:tt)*) => { }
}

#[macro_export] 
#[cfg(any(feature = "dbg", feature = "dbg_level2"))]
macro_rules! karamel_print_level2 {
    ($($arg:tt)*) => (println!("DEBUG2: {}", std::format_args!($($arg)*)));
}

#[macro_export] 
#[cfg(all(not(feature = "dbg"), not(feature = "dbg_level2")))]
macro_rules! karamel_print_level2 {
    ($($arg:tt)*) => { }
}


#[macro_export] 
#[cfg(any(feature = "dbg", feature = "dbg_level3"))]
macro_rules! karamel_print_level3 {
    ($($arg:tt)*) => (println!("DEBUG3: {}", std::format_args!($($arg)*)));
}

#[macro_export] 
#[cfg(all(not(feature = "dbg"), not(feature = "dbg_level3")))]
macro_rules! karamel_print_level3 {
    ($($arg:tt)*) => { }
}

#[macro_export] 
macro_rules! pop {
    ($options: expr) => {{
        $options.current_scope.borrow_mut().memory_index -= 1;
        $options.current_scope.borrow().stack[$options.current_scope.borrow().memory_index].deref()
    }}
}

#[macro_export] 
macro_rules! pop_raw {
    ($options: expr) => {{
        $options.current_scope.borrow_mut().memory_index -= 1;
        $options.current_scope.borrow_mut().stack[$options.current_scope.borrow().memory_index]
    }}
}

#[macro_export] 
macro_rules! fetch_raw {
    ($options: expr) => {{
        *$options.current_scope.borrow().stack[$options.current_scope.borrow().memory_index-1]
    }}
}

#[macro_export] 
macro_rules! current_raw {
    ($options: expr) => {{
        $options.current_scope.borrow().stack[$options.current_scope.borrow().memory_index]
    }}
}

#[macro_export] 
macro_rules! get_memory_index {
    ($options: expr) => {{
        $options.current_scope.borrow().memory_index
    }}
}

#[macro_export] 
macro_rules! inc_memory_index {
    ($options: expr, $count: expr) => {{
        $options.current_scope.borrow_mut().memory_index += $count
    }}
}

#[macro_export] 
macro_rules! dec_memory_index {
    ($options: expr, $count: expr) => {{
        $options.current_scope.borrow_mut().memory_index -= $count
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
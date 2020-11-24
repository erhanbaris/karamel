use crate::types::BramaPrimative;
use crate::compiler::Storage;
use crate::compiler::StaticStorage;

pub type NativeCallResult = Result<(), (&'static str, u32, u32)>;
pub type NativeCall<T> where T: Storage = fn(params: Vec<BramaPrimative>, storage: &T) -> NativeCallResult;

static mut CORE_BUILD_FUNCTIONS: Vec<(&'static str, NativeCall<StaticStorage>)> = Vec::new();

fn dummy<T>(params: Vec<BramaPrimative>, storage: &T) -> NativeCallResult
where T: Storage {
    Ok(())
}

unsafe fn test() {
    CORE_BUILD_FUNCTIONS.push(("dummy", dummy));
}

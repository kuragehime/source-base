//! Valve sdk structs/classes and misc
pub mod cbaseentity;
pub mod dt_recv;
pub mod usercmd;

/// This is used to pretty much emulate a c++ thisptr, the ugly cast chain is due to dyn trait pointers being fat pointers
pub trait GameObject
{
    fn get<T>(&self, offset: usize) -> T
    {
        unsafe { ((self as *const Self as *const () as usize + offset) as *mut T).read() }
        // WOAT
    }
}

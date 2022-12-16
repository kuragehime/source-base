mod paint;
use alloc::collections::BTreeMap;
use lazy_static::lazy_static;
use spin::RwLock;

use crate::{iface, utils::hooking::vmt::VMTHook};
lazy_static! {
    pub static ref HOOKS: RwLock<BTreeMap<&'static str, VMTHook>> = RwLock::new(BTreeMap::new());
}
pub fn init() -> bool { paint::init() }
pub fn destroy() -> bool
{
    paint::destroy();
    true
}

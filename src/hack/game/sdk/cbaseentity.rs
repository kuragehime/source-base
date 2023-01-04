use crate::{cache_offset, hack::game::netvars};

use super::GameObject;

#[allow(non_camel_case_types)]
pub struct C_BaseEntity {}

impl GameObject for C_BaseEntity {}

impl C_BaseEntity
{
    pub fn health(&self) -> i32
    {
        cache_offset!(self, "DT_BasePlayer::m_iHealth");
    }
    pub fn flags(&self) -> u32
    {
        cache_offset!(self, "DT_BasePlayer::m_fFlags");
    }
}

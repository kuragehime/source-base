use vtables::VTable;
use vtables_derive::{has_vtable, virtual_index, VTable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct VGUISurface {}
unsafe impl Sync for VGUISurface {}
#[allow(non_snake_case)]
impl VGUISurface
{
    #[virtual_index(15)]
    pub fn SetDrawColor(&self, r: i32, g: i32, b: i32, a: i32) {}
    #[virtual_index(16)]
    pub fn DrawFilledRect(&self, x: i32, y: i32, x1: i32, y1: i32) {}
    #[virtual_index(18)]
    pub fn DrawOutlinedRect(&self, x: i32, y: i32, x1: i32, y1: i32) {}
    #[virtual_index(19)]
    pub fn DrawLine(&self, x: i32, y: i32, x1: i32, y1: i32) {}
    #[virtual_index(23)]
    pub fn SetTextFont(&self, font: u32) {}
    #[virtual_index(25)]
    pub fn SetTextColor(&self, r: i32, g: i32, b: i32, a: i32) {}
    #[virtual_index(26)]
    pub fn SetTextPos(&self, x: i32, y: i32) {}
    #[virtual_index(28)]
    pub fn DrawRenderText(&self, text: *const i16, len: i32, drawtype: i32) {}
    #[virtual_index(71)]
    pub fn FontCreate(&self) -> u32 {}
    #[virtual_index(72)]
    pub fn SetFontGlyph(
        &self, font: u32, windows_font_name: *const i8, tall: i32, weight: i32, blur: i32,
        scanlines: i32, flags: i32, a: i32, b: i32,
    )
    {
    }
    #[virtual_index(79)]
    pub fn GetTextSize(&self, font: u32, text: *const i16, wide: *mut i32, tall: *mut i32) {}
    #[virtual_index(123)]
    pub fn DrawFilledRectFade(
        &self, x: i32, y: i32, w: i32, h: i32, alpha1: u32, alpha2: u32, horizontal: bool,
    )
    {
    }
}

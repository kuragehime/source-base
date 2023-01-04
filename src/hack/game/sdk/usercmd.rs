use crate::utils::math::vec::Vec3;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CUserCMD
{
    pub destructor:         *const fn(),
    pub command_number:     i32,
    pub tick_count:         i32,
    pub view_angles:        Vec3,
    pub aim_direction:      Vec3,
    pub forward_move:       f32,
    pub side_move:          f32,
    pub up_move:            f32,
    pub buttons:            u32,
    pub impulse:            u8,
    pub weapon_select:      i32,
    pub weapon_subtype:     i32,
    pub random_seed:        i32,
    pub mouse_dx:           i16,
    pub mouse_dy:           i16,
    pub has_been_predicted: bool,
}

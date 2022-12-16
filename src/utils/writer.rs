extern "C" {
    fn printf(s: *const i8, ...) -> i32;
}

const NEWLINE: &str = "\n";

pub const STDOUT: i32 = 1;

pub const STDERR: i32 = 2;

pub struct MiniWriter(i32);

impl core::fmt::Write for MiniWriter
{
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result { minicrt_println(self.0, s) }
}

impl MiniWriter
{
    #[inline]
    pub fn new(handle: i32) -> MiniWriter { MiniWriter(handle) }

    #[inline]
    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result
    {
        core::fmt::Write::write_fmt(self, args)
    }

    #[inline]
    pub fn write_str(&mut self, s: &str) -> core::fmt::Result { minicrt_println(self.0, s) }

    #[inline]
    pub fn write_nl(&mut self) -> core::fmt::Result { minicrt_println(self.0, NEWLINE) }
}

#[inline]
pub fn minicrt_println(handle: i32, msg: &str) -> core::fmt::Result
{
    let msg = msg.as_bytes();
    let cstring = [msg, b"\0"].concat();

    unsafe {
        minicrt_write(handle, &cstring);
    }
    Ok(())
}
unsafe fn minicrt_write(_handle: i32, bytes: &[u8]) -> Option<usize>
{
    usize::try_from(printf(bytes.as_ptr() as _)).ok()
}
#[macro_export]
macro_rules! minicrt_println {
    () => { $crate::minicrt_println!("") };
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::utils::writer::MiniWriter::new($crate::utils::writer::STDOUT);
            stm.write_fmt(format_args!($($arg)*));
			stm.write_nl();
        }
    };
}

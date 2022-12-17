mod createmove;
mod paint;

pub fn init() -> bool
{
    paint::init();
    createmove::init()
}

pub fn destroy() -> bool
{
    paint::destroy();
    createmove::destroy()
}

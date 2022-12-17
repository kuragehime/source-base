mod createmove;
mod paint;
mod painttraverse;

pub fn init() -> bool
{
    paint::init();
    createmove::init();
    painttraverse::init()
}

pub fn destroy() -> bool
{
    paint::destroy();
    createmove::destroy();
    painttraverse::destroy()
}

mod createmove;
mod paint;
mod painttraverse;

pub fn init() -> bool
{
    painttraverse::init();
    paint::init();
    createmove::init();
    true
}

pub fn destroy() -> bool
{
    painttraverse::destroy();
    paint::destroy();
    createmove::destroy();
    true
}

//! Game function hooks
mod createmove;
mod paint;
mod painttraverse;
mod recorduserinput;

/// Init hooks
pub fn init() -> bool
{
    // painttraverse::init();
    createmove::init(); // client movement
    paint::init(); // engine drawing
    recorduserinput::init(); // used for demo files, can be used for demo cleaning (for tf2 comp maybe)

    true
}

/// Destroy hooks when unloading
pub fn destroy() -> bool
{
    // painttraverse::destroy();
    createmove::destroy();
    paint::destroy();
    recorduserinput::destroy();

    true
}

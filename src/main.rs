mod context;
mod system;
mod math;

use context::Context;

fn main() {
    let mut ctx = Context::new("data/config.json");

    while ctx.is_running() {
        ctx.run();
    }
}

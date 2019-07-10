extern crate render_engine as re;

use re::app::App;
use re::exposed_tools::*;

fn main() {
    let mut app = App::new();

    while !app.done {
        app.draw_frame();
    }

    app.print_fps();
}

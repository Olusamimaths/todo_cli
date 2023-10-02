
mod task;
mod task_manager;
mod cmd;

use cmd::App;


fn main() {
    let app = App::new();
    app.init();
}

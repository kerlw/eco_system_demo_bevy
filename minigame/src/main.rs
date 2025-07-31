mod app_setup;

fn main() {
    let mut app = app_setup::create_app();
    app.run();
}

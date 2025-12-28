use client_shared::App;

fn main() {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .run()
        .expect("Should run the App");
}

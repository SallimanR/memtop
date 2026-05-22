mod app;
mod event_loop;
mod info;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "profile-with-tracy")]
    tracy_client::Client::start();
    crate::app::run()
}

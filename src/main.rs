mod api;
mod lib;
mod middleware;
mod model;

use api::wordle;
use model::store::Store;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let app_state = Store::new();

    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Root") });

    app.at("/wordle").nest({
        let mut api = tide::with_state(app_state);

        api.at("/fetch").get(wordle::fetch_all_games);
        api.at("/create").post(wordle::create_game);

        api.at("/attempt").post(wordle::add_attempt);

        api
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

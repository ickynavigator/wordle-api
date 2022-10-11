mod api;
mod model;

use api::wordle;
use model::store::Store;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let app_state = Store {
        games: Default::default(),
    };

    let mut app = tide::with_state(app_state);

    app.at("/create").post(wordle::create_game);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

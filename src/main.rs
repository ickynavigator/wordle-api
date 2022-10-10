mod model;
mod api;

use api::wordle;


#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/wordle").post(wordle::test_game);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
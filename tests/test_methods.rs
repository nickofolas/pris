use pris::{self, Player};

#[tokio::test]
async fn test_methods() -> Result<(), Box<dyn std::error::Error>> {
    let conn = pris::get_connection();
    let mut player = Player::try_new("cmus", &conn).await?;
    // player
    //     .seek_reverse(std::time::Duration::from_secs(15))
    //     .await?;

    let metadata = player.get_metadata().await?;
    let title = match pris::prop_cast::<String>(&metadata, "xesam:title") {
        Some(t) => t.to_string(),
        None => "Unknown title".to_string(),
    };
    println!("{}", title);

    Ok(())
}

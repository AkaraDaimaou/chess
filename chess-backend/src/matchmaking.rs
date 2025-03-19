use sqlx::PgPool;

pub async fn find_match(pool: &PgPool, player_id: i32) -> Result<Option<i32>, sqlx::Error> {
    let opponent = sqlx::query!("SELECT id FROM players WHERE searching = true LIMIT 1")
        .fetch_optional(pool)
        .await?;
    
    if let Some(opponent) = opponent {
        sqlx::query!("UPDATE players SET searching = false WHERE id = $1", opponent.id)
            .execute(pool)
            .await?;
        Ok(Some(opponent.id))
    } else {
        sqlx::query!("UPDATE players SET searching = true WHERE id = $1", player_id)
            .execute(pool)
            .await?;
        Ok(None)
    }
}
pub async fn update_leaderboard(pool: &PgPool, winner_id: i32) {
    sqlx::query!("UPDATE players SET wins = wins + 1 WHERE id = $1", winner_id)
        .execute(pool)
        .await.unwrap();
}

pub async fn get_leaderboard(pool: &PgPool) -> Vec<(i32, i32)> {
    sqlx::query!("SELECT id, wins FROM players ORDER BY wins DESC LIMIT 10")
        .fetch_all(pool)
        .await.unwrap()
        .into_iter()
        .map(|row| (row.id, row.wins))
        .collect()
}

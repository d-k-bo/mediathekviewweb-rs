#[tokio::main(flavor = "current_thread")]
async fn main() -> mediathekviewweb::Result<()> {
    let user_agent = format!(
        "{} Examples ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_REPOSITORY")
    )
    .try_into()
    .unwrap();

    let results = mediathekviewweb::Mediathek::new(user_agent)?
        .query([mediathekviewweb::models::QueryField::Topic], "tagesschau")
        .query([mediathekviewweb::models::QueryField::Title], "20:00 Uhr")
        .duration_min(std::time::Duration::from_secs(10 * 60))
        .duration_max(std::time::Duration::from_secs(30 * 60))
        .include_future(false)
        .sort_by(mediathekviewweb::models::SortField::Timestamp)
        .sort_order(mediathekviewweb::models::SortOrder::Descending)
        .size(2)
        .offset(2)
        .await?;

    println!("{results:#?}");

    Ok(())
}

use error_chain::error_chain;
use scraper::{Html, Selector};
use std::io;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
const FORBIDDEN: [&str; 27] = [
    "Passing, Rushing, & Receiving Table",
    "Passing",
    "Rushing",
    "Receiving",
    "Fumbles",
    "Player",
    "Tm",
    "Cmp",
    "Att",
    "Yds",
    "TD",
    "Int",
    "Sk",
    "Yds",
    "Lng",
    "Rate",
    "Att",
    "Yds",
    "TD",
    "Lng",
    "Tgt",
    "Rec",
    "Yds",
    "TD",
    "Lng",
    "Fmb",
    "FL",
];

#[tokio::main]
async fn main() -> Result<()> {
    // read urls from stdin
    let mut url = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut url)?;

    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("#div_player_offense").unwrap();

    let table = document.select(&selector).next().unwrap();
    let mut text = table.text().collect::<Vec<&str>>();
    text = text
        .into_iter()
        .filter(|s| s.chars().next().unwrap().is_alphanumeric() && !FORBIDDEN.contains(s))
        .collect::<Vec<&str>>();

    for t in &text {
        println!("{}\t", t);
    }

    Ok(())
}

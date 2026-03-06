use color_eyre::eyre::Result as EyreResult;
use core::fmt::Display;
use reqwest::blocking::Client;
use serde::Deserialize;

const TOP_STORIES_API: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

#[must_use]
#[inline]
fn get_story_info_url(id: u64) -> String {
    format!("https://hacker-news.firebaseio.com/v0/item/{id}.json")
}

#[derive(Clone, Debug, Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
}

impl Story {
    fn fetch(client: &Client, id: u64) -> EyreResult<Self> {
        let url = get_story_info_url(id);
        let response = client.get(url).send()?;
        let parsed: Self = response.json()?;
        Ok(parsed)
    }
}

impl Display for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{title} ({points} points by {author})",
            title = self.title,
            points = self.score,
            author = self.by,
        )?;

        if let Some(url) = &self.url {
            write!(f, "   {url}")
        } else {
            write!(f, "   (no URL)")
        }
    }
}

fn get_top_ids(client: &Client) -> EyreResult<Vec<u64>> {
    let response = client.get(TOP_STORIES_API).send()?;
    let parsed: Vec<u64> = response.json()?;
    Ok(parsed)
}

fn main() -> EyreResult<()> {
    color_eyre::install()?;

    println!("🔶 Top 10 Hacker News Stories\n");

    let client = Client::new();
    let top_ids = get_top_ids(&client)?;

    for (i, id) in top_ids.iter().take(10).copied().enumerate() {
        let story = Story::fetch(&client, id)?;

        println!("{}. {story}", i + 1);
    }

    Ok(())
}

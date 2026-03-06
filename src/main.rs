use core::fmt::Display;

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

fn main() {
    println!("🔶 Top 10 Hacker News Stories\n");

    let client = reqwest::blocking::Client::new();

    let top_ids: Vec<u64> = client
        .get(TOP_STORIES_API)
        .send()
        .expect("Failed to fetch top stories")
        .json()
        .expect("Failed to parse story IDs");

    for (i, id) in top_ids.iter().take(10).enumerate() {
        let url = get_story_info_url(*id);

        let story: Story = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");

        println!("{}. {story}", i + 1);
    }
}

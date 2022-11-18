use scraper::{Html, Selector};
use reqwest::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Build Client
    let client = Client::builder()
        .cookie_store(true)
        .build()?;

    // Get Login Page
    let res = client.get("https://google.com").send().await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("\nCookies:\n{:#?}", res.cookies().fold(String::new(), |acc, c| acc + &format!("{}={}; ", c.name(), c.value())));

    // Get Returned HTML
    let html = res.text().await?;

    // Parse Page
    let document = Html::parse_fragment(&html);
    let selector = Selector::parse("title").unwrap();
    let page = document.select(&selector).next().unwrap();

    // Print Page Title
    println!("\nPage Title: {}", page.text().collect::<Vec<_>>().join(""));

    // Return
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the Applicaiton Runs
    #[test]
    fn test_all_ok() {
        main().expect("Failed to run main");
    }
}

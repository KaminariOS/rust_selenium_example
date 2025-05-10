use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let mut caps = DesiredCapabilities::chrome();
     caps.set_binary("/nix/store/kfah02kcpl8wgf85wchhcmjdqamm27b6-google-chrome-135.0.7049.52/bin/google-chrome-stable")?;
     // caps.add_arg("--headless=new")?;

     let driver = WebDriver::new("http://localhost:40213", caps).await?;

     // Navigate to https://wikipedia.org.
     driver.goto("https://www.tradingview.com/symbols/ETHUSD?exchange=CRYPTO").await?;
     // Locate the element using CSS selector
     let parent_element = driver.query(By::ClassName("js-symbol-header-ticker")).first().await?;

    let child_element = parent_element.find(By::ClassName("js-symbol-last")).await?;
    loop {
       use strip_tags::strip_tags; 
    let number = child_element.inner_html().await?;
    let number = strip_tags(&number);
    let number: f64 = number 
    .replace(",", "") // Remove commas if present
    .parse().unwrap();

     println!("Extracted number: {:?}", number);
    use tokio::time::Duration;
    tokio::time::sleep(Duration::from_millis(100)).await;
    }

     // Always explicitly close the browser.
     driver.quit().await?;

     Ok(())
}

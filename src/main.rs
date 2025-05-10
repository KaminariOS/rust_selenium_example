use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let mut caps = DesiredCapabilities::chrome();
     caps.set_binary("/nix/store/kfah02kcpl8wgf85wchhcmjdqamm27b6-google-chrome-135.0.7049.52/bin/google-chrome-stable")?;
     caps.add_arg("--headless=new")?;

     let driver = WebDriver::new("http://localhost:40213", caps).await?;

     // Navigate to https://wikipedia.org.
     driver.goto("https://www.tradingview.com/symbols/XAUUSD").await?;
     // Locate the element using CSS selector
     let parent_element = driver.find(By::Css("div.js-symbol-header-ticker[data-symbol='OANDA:XAUUSD']")).await?;

    let child_element = parent_element.find(By::ClassName("js-symbol-last")).await?;

     let span_element = child_element.find(By::Tag("span")).await?;

    // Retrieve the text content
    let text = span_element.text().await?;
    let number: f64 = text
    .replace(",", "") // Remove commas if present
    .parse().unwrap();

     println!("Extracted number: {}", number);
     //
     // // Look for header to implicitly wait for the page to load.
     // driver.find(By::ClassName("firstHeading")).await?;
     // assert_eq!(driver.title().await?, "Selenium - Wikipedia");
    
     // Always explicitly close the browser.
     driver.quit().await?;

     Ok(())
}

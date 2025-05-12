use thirtyfour::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let args: Vec<String> = env::args().collect();
     let symbol = &args[1];
     let mut caps = DesiredCapabilities::chrome();
     // caps.set_binary("/nix/store/jz2idxb3f0lwf14b21kvn4h3ayjc5rb1-google-chrome-136.0.7103.59/bin/google-chrome-stable")?;
     caps.add_arg("--headless=new")?;
     caps.add_arg("--incognito");
     let port = env::var("DRIVER_PORT").unwrap();

     let driver = WebDriver::new(format!("http://localhost:{port}"), caps).await?;

     // Navigate to https://wikipedia.org.
     driver.goto(format!("https://www.tradingview.com/symbols/{symbol}")).await?;
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

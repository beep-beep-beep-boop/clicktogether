use crossterm::{
    event::{Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use tokio::runtime::Runtime;

pub fn start_client(address: String, username: String) -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    //
    // Spawn the root task
    rt.block_on(async {
        println!("starting client!");
        println!("press escape to exit!!");
        println!("press space to send a click!!");

        send_join(&address, &username).await?;

        enable_raw_mode()?;

        let mut reader = EventStream::new();

        loop {
            let event = reader.next().await;

            match event {
                Some(Ok(event)) => {
                    if event == Event::Key(KeyCode::Char(' ').into()) {
                        println!("sending click!\r");
                        send_click(&address, &username).await?;
                    }

                    if event == Event::Key(KeyCode::Esc.into()) {
                        break;
                    }
                }
                Some(Err(e)) => println!("Error: {:?}\r", e),
                None => break,
            }
        }

        disable_raw_mode()?;

        println!("exiting...");

        send_leave(&address, &username).await?;

        Ok(())
    })
}

async fn send_join(address: &String, username: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/join/{}", address, username);
    let client = reqwest::Client::new();
    client.put(url).send().await?;

    Ok(())
}

async fn send_leave(address: &String, username: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/leave/{}", address, username);
    let client = reqwest::Client::new();
    client.put(url).send().await?;

    Ok(())
}

async fn send_click(address: &String, username: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/click/{}", address, username);
    let client = reqwest::Client::new();
    client.post(url).send().await?;

    Ok(())
}
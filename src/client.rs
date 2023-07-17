use crossterm::{
    event::{Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use reqwest::StatusCode;

pub async fn start_client(
    address: String,
    username: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("connecting to server...");

    match send_join(&address, &username).await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("! could not connect to server >< ! {}", err);
            std::process::exit(1);
        }
    };

    println!("connected! :3");
    println!("press escape to exit!!");
    println!("press space to send a click!!");

    enable_raw_mode()?;

    let mut reader = EventStream::new();

    loop {
        let event = reader.next().await;

        match event {
            Some(Ok(event)) => {
                if event == Event::Key(KeyCode::Char(' ').into()) {
                    println!("sending click!\r");
                    match send_click(&address, &username).await {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("! could not send click to server >< ! {}\r", err);
                        }
                    };
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

    match send_leave(&address, &username).await {
        Ok(_) => {}
        Err(err) => {
            eprintln!(
                "! could not tell the server we were leaving >< ! is it down? {}",
                err
            );
        }
    };

    Ok(())
}

async fn send_join(address: &String, username: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/join/{}", address, username);
    let client = reqwest::Client::new();
    let res = client.put(url).send().await?;

    if res.status() == StatusCode::ALREADY_REPORTED {
        eprintln!("! this username is already connected! joining as the same user.");
    }

    Ok(())
}

async fn send_leave(address: &String, username: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/leave/{}", address, username);
    let client = reqwest::Client::new();
    let res = client.put(url).send().await?;

    if res.status() == StatusCode::CONFLICT {
        eprintln!("! tried to leave, but this user was not in the server to begin with!");
    }

    Ok(())
}

async fn send_click(address: &String, username: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/click/{}", address, username);
    let client = reqwest::Client::new();
    let res = client.post(url).send().await?;

    if res.status() == StatusCode::CONFLICT {
        eprintln!("! the server said a user with your username has not joined! joining...\r");
        send_join(address, username).await?;
        println!("! successfully joined! try to send the click again!\r");
    }

    Ok(())
}

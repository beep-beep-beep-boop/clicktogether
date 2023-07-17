use std::net::TcpListener;

use enigo::Key;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;

type User = String;

#[derive(Debug, Clone)]
pub struct ServerState {
    connected_users: HashSet<User>,

    // clicks we have had so far. reset to 0 when it reaches the
    // number of connected users
    clicks: u32,

    // the key the server will click
    key: Key,
}

type State = Arc<Mutex<ServerState>>;

mod filters {
    use super::handlers;
    use super::State;
    use warp::Filter;

    pub fn filters(
        state: State,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        user_join(state.clone())
            .or(user_leave(state.clone()))
            .or(click(state.clone()))
    }

    /// PUT /join/:username
    pub fn user_join(
        state: State,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("join" / String)
            .and(warp::put())
            .and(with_state(state))
            .and_then(handlers::user_join)
    }

    /// PUT /leave/:username
    pub fn user_leave(
        state: State,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("leave" / String)
            .and(warp::put())
            .and(with_state(state))
            .and_then(handlers::user_leave)
    }

    /// POST /click/:username
    pub fn click(
        state: State,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("click" / String)
            .and(warp::post())
            .and(with_state(state))
            .and_then(handlers::click)
    }

    fn with_state(
        state: State,
    ) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || state.clone())
    }
}

mod handlers {
    use super::State;
    use enigo::{Enigo, KeyboardControllable};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn user_join(username: String, state: State) -> Result<impl warp::Reply, Infallible> {
        let mut lock = state.lock().await;

        println!("user joining... ({})", username);

        if lock.connected_users.insert(username) {
            Ok(StatusCode::OK)
        } else {
            Ok(StatusCode::ALREADY_REPORTED)
        }
    }

    pub async fn user_leave(
        username: String,
        state: State,
    ) -> Result<impl warp::Reply, Infallible> {
        let mut lock = state.lock().await;

        println!("user leaving... ({})", username);

        if lock.connected_users.remove(&username) {
            Ok(StatusCode::OK)
        } else {
            // if the user wasn't joined to begin with
            Ok(StatusCode::CONFLICT)
        }
    }

    pub async fn click(username: String, state: State) -> Result<impl warp::Reply, Infallible> {
        let mut lock = state.lock().await;

        println!("user submitting click... ({})", username);

        if lock.connected_users.contains(&username) {
            // do the click !

            lock.clicks += 1;
            if usize::try_from(lock.clicks).unwrap() >= lock.connected_users.len() {
                let mut enigo = Enigo::new();

                enigo.key_click(lock.key);

                lock.clicks = 0;

                println!("server clicking!")
            }

            Ok(StatusCode::OK)
        } else {
            // that user wasnt joined???
            eprintln!(
                "user {} tried to click but hadn't joined the room/server!",
                username
            );
            Ok(StatusCode::CONFLICT)
        }
    }
}

pub fn start_server(listener: TcpListener, key: Key) -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
        // create the initial state
        let state: State = Arc::new(Mutex::new(ServerState {
            connected_users: HashSet::new(),
            clicks: 0,
            key,
        }));

        let api = filters::filters(state);

        let listener = tokio::net::TcpListener::from_std(listener)?;
        let addr = listener.local_addr()?;

        println!("server listening on {}", addr);

        let stream = TcpListenerStream::new(listener);

        warp::serve(api).run_incoming(stream).await;

        Ok(())
    })
}

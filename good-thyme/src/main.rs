use dotenvy::dotenv;
use http_body_util::{combinators::BoxBody, BodyExt};
use http_body_util::{Empty, Full};
use hyper::body::{Body, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn echo(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/narration/level") => {
            request_to_chat("Generate a thoughtful, concise, one sentence narration in the tone of a low fantasy novel. The perspective is from a watcher in the woods: a wise, forest druid unknown to the subject. The setting is a dense, dark, misty forest. The player is delving deep into the forest to gather ingredients to bake a cake for a Great British Bake Off style competition. I will provide you stats representing events that occur on each level. The narrator always gets the player's name slightly and humorously wrong.".to_string(), req).await
        }
        (&Method::POST, "/narration/dead") => {
            request_to_chat("Generate a thoughtful, concise, one sentence narration in the tone of a low fantasy novel. The perspective is from a watcher in the woods: a wise, forest druid unknown to the subject. The setting is a dense, dark, misty forest. The player WAS delving deep into the forest to gather ingredients to bake a cake for a Great British Bake Off style competition, but unfortunately they have just died during the perilous journey. I will provide you stats representing events that occur on each level. The narrator always gets the player's name slightly and humorously wrong.".to_string(), req).await
        }
        (&Method::POST, "/narration/baked") => {
            request_to_chat("Generate a thoughtful, concise, one sentence narration in the tone of a low fantasy novel. The perspective is from a watcher in the woods: a wise, forest druid unknown to the subject. The setting is a dense, dark, misty forest. The player WAS delving deep into the forest to gather ingredients to bake a cake for a Great British Bake Off style competition, and they have just done it! I will provide you stats representing events that occur on each level, as well as stats about the cake they have just baked -- include a ranking against 3 other cakes based on these stats, and bias the player to 1st or 2nd place unless it is inedible. There are 4 judges: Mr. Hollywood, Ms. Goodberry, Myserious Figure, and Sir Fields. The narrator always gets the player's name slightly and humorously wrong. You MUST end the narration with '... and we had a wild thyme'.".to_string(), req).await
        }
        (&Method::POST, "/narration/garden") => {
            request_to_chat("Generate a thoughtful, concise, one sentence narration in the tone of a low fantasy novel. The perspective is from a watcher in the woods: a wise, forest druid unknown to the subject. The setting is an oasis within a dense, dark, misty forest. The player is delving deep into the forest to gather ingredients to bake a cake for a Great British Bake Off style competition. They need GOOD THYME and at least 3 ingredients to bake a successful cake. Currently, they see a beautiful garden with a lively spring, and a squinty-eyed forest druid frolicing among butterflies and toads, with mushrooms growing everywhere. The druid is happy to see the player and happy to help. I will provide you stats representing events that occur on each level. The narrator always gets the player's name slightly and humorously wrong, and the narrator is the druid, who has been magically watching the player for quite some time now.".to_string(), req).await
        }

        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

async fn request_to_chat(
    system_prompt: String,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    // Protect our server from massive bodies.
    let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if upper > 1024 * 64 {
        let mut resp = Response::new(full("Body too big"));
        *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
        return Ok(resp);
    }

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(system_prompt),
        name: None,
        function_call: None,
    }];

    // Await the whole body to be collected into a single `Bytes`...
    let whole_body = req.collect().await?.to_bytes();

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(String::from_utf8(whole_body.to_vec()).unwrap_or(
            "a dark fog covers the land, and the protagonist's actions are unknown...".to_string(),
        )),
        name: None,
        function_call: None,
    });
    let chat_completion = ChatCompletion::builder("gpt-4-1106-preview", messages.clone())
        .create()
        .await
        .unwrap();
    let message;
    if let Some(first_result) = chat_completion.choices.first() {
        message = first_result
            .message
            .clone()
            .content
            .clone()
            .unwrap_or("A mysterious fog has covered the land...".to_string());
    } else {
        message = "The sky trembles and cuts with a misty darkness, but you find it peaceful..."
            .to_string();
    }
    let response = serde_json::to_string(&json!({
        "narration": message,
    }));
    Ok(Response::new(full(
        response.ok().expect("all edge cases should be handled"),
    )))
}

// We create some utility functions to make Empty and Full bodies
// fit our broadened Response body type.
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(echo))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

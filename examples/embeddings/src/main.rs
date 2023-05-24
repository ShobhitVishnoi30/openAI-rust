use std::error::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use async_openai::{types::CreateEmbeddingRequestArgs, Client};
use dotenv::dotenv;
use serde_json::json;
use serde::Deserialize;


pub fn initialize() {
    dotenv().ok();
}

#[derive(Debug, Deserialize)]
struct RequestData {
    name: String,
    // Add other fields as needed
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world aa!")
}

#[post("/getEmbeddings")]
async fn getEmbeddings(req_body: web::Json<RequestData>) -> impl Responder {
     let client = Client::new();
    let data = req_body.0; // Access the deserialized JSON data
    let name = data.name; // Access the name field
    
    // Process the request data as needed
    println!("Name: {}", name);
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input(name.to_string())
        .build();

    match request {
        Ok(request) => {
            let response = client.embeddings().create(request).await;

            match response {
                Ok(response) => {
                    for data in &response.data {
                        println!(
                            "[{}]: has embedding of length {}",
                            data.index,
                            data.embedding.len()
                        )
                    }
                   HttpResponse::Ok()
                    .content_type("application/json") // Set the response content type to JSON
                    .body(json!(response.data).to_string())
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    HttpResponse::InternalServerError().body("Internal Server Error")
                }
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
   
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize();
    HttpServer::new(|| {
        App::new()
           .service(web::scope("/app").service(hello))
            .service(getEmbeddings)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}




// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     initialize();
//     let client = Client::new();

//     // An embedding is a vector (list) of floating point numbers.
//     // The distance between two vectors measures their relatedness.
//     // Small distances suggest high relatedness and large distances suggest low relatedness.

//     let request = CreateEmbeddingRequestArgs::default()
//         .model("text-embedding-ada-002")
//         .input([
//             "Why do programmers hate nature? It has too many bugs.",
//             "Why was the computer cold? It left its Windows open.",
//         ])
//         .build()?;

//     let response = client.embeddings().create(request).await?;

//     for data in response.data {
//         println!(
//             "[{}]: has embedding of length {}",
//             data.index,
//             data.embedding.len()
//         )
//     }

//     Ok(())
// }
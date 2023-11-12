#![allow(dead_code)]
#![allow(non_snake_case)]

use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]

//Oggetto Todo
struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Url API
    let url: &str = "https://jsonplaceholder.typicode.com/todos/99";
    // Eseguire la richiesta GET
    let response: reqwest::Response = reqwest::get(url).await?;
    // Preleviamo il body in json dalla response
    let json_result: Todo = response.json().await?;
    // Print della risposta
    println!("My result is: {}, and is marked as: {}", json_result.title, json_result.completed);
    // Restituiamo OK per terminare l'esecuzione con successo
    Ok(())
}













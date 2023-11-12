use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]

struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/todos/99";
    
    // Eseguire la richiesta GET
    let response = reqwest::get(url).await?;
    let json_result: Todo = response.json().await?;
    println!("My result is: {}, and is marked as: {}", json_result.title, json_result.completed);

    Ok(())
}













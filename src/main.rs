use todo_list::todo_list_server::{TodoList, TodoListServer};
use todo_list::{AddRequest, Empty, ListResponse};
use tonic::{Request, Response, Result, Status, transport::Server};
pub mod todo_list {
    tonic::include_proto!("todo");
}

#[derive(Debug, Default)]
pub struct TodoListService {}

#[tonic::async_trait]
impl TodoList for TodoListService {
    async fn add(&self, request: Request<AddRequest>) -> Result<Response<Empty>, Status> {
        println!("Got a request: {:?}", request);
        Ok(Response::new(Empty {}))
    }

    async fn list(&self, request: Request<Empty>) -> Result<Response<ListResponse>, Status> {
        println!("Got a request: {:?}", request);
        Ok(Response::new(ListResponse {
            list: vec!["hello".to_string()],
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let todo_list = TodoListService::default();

    Server::builder()
        .add_service(TodoListServer::new(todo_list))
        .serve(addr)
        .await?;

    Ok(())
}

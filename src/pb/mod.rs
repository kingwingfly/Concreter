mod error;
mod sym;

// pub mod helloworld {
//     tonic::include_proto!("helloworld");
// }

// use tonic::transport::server::Routes;
// use tonic::transport::Server;
// use tonic::{Request, Response, Status};

// pub use helloworld::greeter_server::{Greeter, GreeterServer};
// pub use helloworld::{HelloReply, HelloRequest};

// struct GreeterImpl;

// #[tonic::async_trait]
// impl Greeter for GreeterImpl {
//     async fn say_hello(
//         &self,
//         request: Request<HelloRequest>,
//     ) -> Result<Response<HelloReply>, Status> {
//         println!("Got a request: {:?}", request);

//         let reply = HelloReply {
//             message: format!("Hello {}!", request.into_inner().name).into(),
//         };

//         Ok(Response::new(reply))
//     }
// }

// pub fn init_greeter() -> Routes {
//     let greeter = GreeterServer::new(GreeterImpl {});
//     Server::builder().add_service(greeter).into_service()
// }

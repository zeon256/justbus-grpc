#[macro_use]
extern crate lazy_static;

use crate::cache::Cache;
use crate::models::buses::just_bus_server::{JustBus, JustBusServer};
use crate::models::buses::*;
use lta::prelude::*;
use lta::r#async::bus::get_arrival;
use lta::r#async::lta_client::LTAClient;
use std::env::var;
use std::time::Duration;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod cache;
mod models;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

lazy_static! {
    static ref CLIENT: LTAClient = {
        let api_key = var("API_KEY").expect("API_KEY not found!");
        LTAClient::with_api_key(api_key)
    };
    static ref CACHE: Cache<u32, BusTimings> = {
        let duration = Duration::from_secs(15);
        Cache::with_ttl(duration)
    };
}

#[derive(Default)]
pub struct MyJustBus;

#[tonic::async_trait]
impl JustBus for MyJustBus {
    async fn timings(&self, request: Request<BusStopId>) -> Result<Response<BusTimings>, Status> {
        let bus_stop_id = request.into_inner().bus_stop_id;
        let in_cache = CACHE.get(bus_stop_id);
        match in_cache {
            Some(v) => Ok(Response::new(v)),
            None => {
                let res = get_arrival(&CLIENT, bus_stop_id, None)
                    .await
                    .map_err(|_| Status::internal("Client Error!"))?
                    .services;

                let data = BusTimings::from(res);
                CACHE.insert(bus_stop_id, data.clone());

                Ok(Response::new(data))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    println!("Started server on {:?}", &addr);

    let just_bus = MyJustBus::default();
    Server::builder()
        .add_service(JustBusServer::new(just_bus))
        .serve(addr)
        .await?;

    Ok(())
}

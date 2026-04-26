use rand::Rng;
use resonate::prelude::*;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Identify this process as a client (its own group). The client doesn't
    // execute durable functions — it just dispatches them and exits.
    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        group: Some("client".into()),
        ..Default::default()
    });

    let id = Uuid::new_v4().to_string();
    let compute_cost: u64 = rand::thread_rng().gen_range(1..=10);

    // Fire-and-forget RPC. `.spawn()` starts the execution and returns a
    // handle without awaiting the result, so this script exits immediately.
    // The `poll://any@workers` target tells the Resonate Server to dispatch
    // to any one available worker in the `workers` group — that's the load
    // balancer.
    let _handle = resonate
        .rpc::<_, ()>(&id, "compute_something", (id.clone(), compute_cost))
        .target("poll://any@workers")
        .spawn()
        .await
        .expect("rpc spawn failed");

    println!("dispatched {id} (compute_cost={compute_cost}s)");
    resonate.stop().await.ok();
}

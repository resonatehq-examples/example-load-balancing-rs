use resonate::prelude::*;
use std::time::Duration;

/// A function that simulates a computation taking some seconds.
///
/// Every worker process registers this same function under the same group
/// (`workers`). When the client dispatches an invocation with
/// `target("poll://any@workers")`, the Resonate Server picks one available
/// worker in the group and routes the work to it. Run multiple workers and
/// you'll see the executions spread across them automatically.
#[resonate::function]
async fn compute_something(ctx: &Context, id: String, compute_cost: u64) -> Result<()> {
    println!("{id} starting computation");
    // Durable sleep simulates a time-consuming task. Survives restarts —
    // if this worker crashes mid-sleep, another worker resumes it on the
    // remaining time.
    ctx.sleep(Duration::from_secs(compute_cost)).await?;
    println!("{id} computed something that cost {compute_cost} seconds");
    Ok(())
}

#[tokio::main]
async fn main() {
    // Identify this process as a member of the `workers` group. Every
    // worker instance you start with this same group becomes part of the
    // load-balancing pool.
    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        group: Some("workers".into()),
        ..Default::default()
    });

    resonate.register(compute_something).unwrap();

    println!("worker is running...");
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
}

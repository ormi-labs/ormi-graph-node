use std::io::Write;
use std::sync::Arc;

use graph::futures01::Stream as _;
use graph::futures03::compat::Future01CompatExt;
use graph::{
    components::store::SubscriptionManager as _,
    prelude::{serde_json, Error},
};
use graph_store_postgres::SubscriptionManager;

async fn listen(mgr: Arc<SubscriptionManager>) -> Result<(), Error> {
    let events = mgr.subscribe();
    println!("press ctrl-c to stop");
    let res = events
        .inspect(move |event| {
            serde_json::to_writer_pretty(std::io::stdout(), event)
                .expect("event can be serialized to JSON");
            writeln!(std::io::stdout()).unwrap();
            std::io::stdout().flush().unwrap();
        })
        .collect()
        .compat()
        .await;

    match res {
        Ok(_) => {
            println!("stream finished")
        }
        Err(()) => {
            eprintln!("stream failed")
        }
    }
    Ok(())
}

pub async fn assignments(mgr: Arc<SubscriptionManager>) -> Result<(), Error> {
    println!("waiting for assignment events");
    listen(mgr).await?;

    Ok(())
}

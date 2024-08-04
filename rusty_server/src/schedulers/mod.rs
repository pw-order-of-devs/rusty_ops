use messaging::mq_client::MqClient;
use persist::db_client::DbClient;

pub mod agent_ttl;
pub mod pipeline_cleanup;
pub mod pipeline_logs;

/// initialization of schedulers
pub fn init(db: &DbClient, mq: &MqClient) {
    // scheduler for agent expiry - remove agent reference after expiration
    let db_agents = db.clone();
    tokio::spawn(async move {
        agent_ttl::schedule(&db_agents).await;
    });

    // scheduler for pipelines with unknown agent - clean up status if assigned to nonexistent agent
    let db_pipelines = db.clone();
    tokio::spawn(async move {
        pipeline_cleanup::schedule(&db_pipelines).await;
    });

    // scheduler for pipeline logs - read from mq, push to db
    let db_pipelines = db.clone();
    let mut mq_pipelines = mq.clone();
    tokio::spawn(async move {
        pipeline_logs::schedule(&db_pipelines, &mut mq_pipelines).await;
    });
}

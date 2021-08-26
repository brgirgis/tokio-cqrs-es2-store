use cqrs_es2::Error;

use tokio_cqrs_es2_store::{
    redis_store::{
        EventStorage,
        QueryStorage,
        RedisEventStore,
        RedisQueryStore,
    },
    Repository,
};

use crate::cqrs::db_connection;

use super::super::{
    aggregate::BankAccount,
    commands::BankAccountCommand,
    dispatchers::LoggingDispatcher,
    events::BankAccountEvent,
    queries::BankAccountQuery,
};

type ThisEventStore = RedisEventStore<
    BankAccountCommand,
    BankAccountEvent,
    BankAccount,
>;

type ThisQueryStore = RedisQueryStore<
    BankAccountCommand,
    BankAccountEvent,
    BankAccount,
    BankAccountQuery,
>;

type ThisRepository = Repository<
    BankAccountCommand,
    BankAccountEvent,
    BankAccount,
    ThisEventStore,
>;

pub async fn get_event_store() -> Result<ThisRepository, Error> {
    Ok(ThisRepository::new(
        ThisEventStore::new(
            EventStorage::new(db_connection().await.unwrap()),
            true,
        ),
        vec![
            Box::new(get_query_store().await.unwrap()),
            Box::new(LoggingDispatcher::new()),
        ],
    ))
}

pub async fn get_query_store() -> Result<ThisQueryStore, Error> {
    Ok(ThisQueryStore::new(QueryStorage::new(
        db_connection().await.unwrap(),
    )))
}

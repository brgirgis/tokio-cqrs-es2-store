use async_trait::async_trait;
use log::info;

use cqrs_es2::{
    Error,
    EventContext,
};

use tokio_cqrs_es2_store::IEventDispatcher;

use super::super::{
    commands::BankAccountCommand,
    events::BankAccountEvent,
};

pub struct LoggingDispatcher {}

impl LoggingDispatcher {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IEventDispatcher<BankAccountCommand, BankAccountEvent>
    for LoggingDispatcher
{
    async fn dispatch(
        &mut self,
        aggregate_id: &str,
        events: &Vec<
            EventContext<BankAccountCommand, BankAccountEvent>,
        >,
    ) -> Result<(), Error> {
        for event in events {
            let payload =
                serde_json::to_string_pretty(&event.payload).unwrap();
            info!(
                "dispatching event '{}' for aggregate '{}' with \
                 sequence '{}'",
                payload, aggregate_id, event.sequence
            );
        }

        Ok(())
    }
}

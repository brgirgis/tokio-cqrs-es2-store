use async_trait::async_trait;
use std::collections::HashMap;

use cqrs_es2::{
    AggregateContext,
    Error,
    EventContext,
    IAggregate,
    ICommand,
    IEvent,
};

/// The abstract central source for loading past events and committing
/// new events.
#[async_trait]
pub trait IEventStore<C: ICommand, E: IEvent, A: IAggregate<C, E>> {
    /// Load all events for a particular `aggregate_id`
    async fn load_events(
        &mut self,
        aggregate_id: &str,
    ) -> Result<Vec<EventContext<C, E>>, Error>;

    /// Load aggregate at current state
    async fn load_aggregate(
        &mut self,
        aggregate_id: &str,
    ) -> Result<AggregateContext<C, E, A>, Error>;

    /// Commit new events
    async fn commit(
        &mut self,
        events: Vec<E>,
        context: AggregateContext<C, E, A>,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventContext<C, E>>, Error>;

    /// Wrap a set of events with the additional metadata
    /// needed for persistence and publishing
    fn wrap_events(
        &self,
        aggregate_id: &str,
        current_sequence: usize,
        events: Vec<E>,
        metadata: HashMap<String, String>,
    ) -> Vec<EventContext<C, E>> {
        let mut sequence = current_sequence;

        let mut wrapped_events = Vec::new();

        for payload in events {
            sequence += 1;

            wrapped_events.push(EventContext::new(
                aggregate_id.to_string(),
                sequence,
                payload,
                metadata.clone(),
            ));
        }

        wrapped_events
    }
}

use async_trait::async_trait;
use std::sync::{
    Arc,
    RwLock,
};

use cqrs_es2::{
    example_impl::*,
    Error,
    EventContext,
};

use crate::IEventDispatcher;

pub struct CustomDispatcher {
    events: Arc<
        RwLock<Vec<EventContext<CustomerCommand, CustomerEvent>>>,
    >,
}

impl CustomDispatcher {
    pub fn new(
        events: Arc<
            RwLock<Vec<EventContext<CustomerCommand, CustomerEvent>>>,
        >
    ) -> Self {
        CustomDispatcher { events }
    }
}

#[async_trait]
impl IEventDispatcher<CustomerCommand, CustomerEvent>
    for CustomDispatcher
{
    async fn dispatch(
        &mut self,
        _aggregate_id: &str,
        events: &[EventContext<CustomerCommand, CustomerEvent>],
    ) -> Result<(), Error> {
        for event in events {
            let mut event_list = self.events.write().unwrap();
            event_list.push(event.clone());
        }

        Ok(())
    }
}

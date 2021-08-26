use std::collections::HashMap;

use redis::Client;

use cqrs_es2::{
    example_impl::*,
    AggregateContext,
    Error,
    EventContext,
};

use crate::{
    redis_store::{
        EventStorage,
        RedisEventStore,
    },
    repository::IEventStore,
};

use super::common::*;

type ThisEventStore =
    RedisEventStore<CustomerCommand, CustomerEvent, Customer>;

type ThisAggregateContext =
    AggregateContext<CustomerCommand, CustomerEvent, Customer>;

type ThisEventContext = EventContext<CustomerCommand, CustomerEvent>;

pub fn metadata() -> HashMap<String, String> {
    let now = "2021-03-18T12:32:45.930Z".to_string();
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), now);
    metadata
}

async fn commit_and_load_events(
    with_snapshots: bool
) -> Result<(), Error> {
    let client = match Client::open(CONNECTION_STRING) {
        Ok(x) => x,
        Err(e) => {
            return Err(Error::new(e.to_string().as_str()));
        },
    };

    let conn = match client.get_connection() {
        Ok(x) => x,
        Err(e) => {
            return Err(Error::new(e.to_string().as_str()));
        },
    };

    let storage = EventStorage::new(conn);

    let mut store = ThisEventStore::new(storage, with_snapshots);

    let id = uuid::Uuid::new_v4().to_string();

    // loading nonexisting stream defaults to a vector with zero
    // length
    assert_eq!(
        0,
        store
            .load_events(id.as_str(), false)
            .await
            .unwrap()
            .len()
    );

    // loading nonexisting aggregate returns default construction
    let context = store
        .load_aggregate(id.as_str())
        .await
        .unwrap();

    assert_eq!(
        context,
        ThisAggregateContext::new(id.clone(), Customer::default(), 0)
    );

    // apply a couple of events
    let events = vec![
        CustomerEvent::NameAdded(NameAdded {
            changed_name: "test_event_A".to_string(),
        }),
        CustomerEvent::EmailUpdated(EmailUpdated {
            new_email: "test A".to_string(),
        }),
        CustomerEvent::AddressUpdated(AddressUpdated {
            new_address: "test B".to_string(),
        }),
    ];

    store
        .commit(
            vec![events[0].clone(), events[1].clone()],
            context,
            metadata(),
        )
        .await
        .unwrap();

    let contexts = store
        .load_events(id.as_str(), false)
        .await
        .unwrap();

    // check stored events are correct
    assert_eq!(
        contexts,
        vec![
            ThisEventContext::new(
                id.to_string(),
                1,
                events[0].clone(),
                Default::default()
            ),
            ThisEventContext::new(
                id.to_string(),
                2,
                events[1].clone(),
                Default::default()
            ),
        ]
    );

    let context = store
        .load_aggregate(id.as_str())
        .await
        .unwrap();

    // check stored aggregate is correct
    assert_eq!(
        context,
        ThisAggregateContext::new(
            id.clone(),
            Customer {
                customer_id: "".to_string(),
                name: "test_event_A".to_string(),
                email: "test A".to_string(),
                addresses: Default::default()
            },
            2
        )
    );

    store
        .commit(
            vec![events[2].clone()],
            context,
            metadata(),
        )
        .await
        .unwrap();

    let contexts = store
        .load_events(id.as_str(), false)
        .await
        .unwrap();

    // check stored events are correct
    assert_eq!(
        contexts,
        vec![
            ThisEventContext::new(
                id.to_string(),
                1,
                events[0].clone(),
                Default::default()
            ),
            ThisEventContext::new(
                id.to_string(),
                2,
                events[1].clone(),
                Default::default()
            ),
            ThisEventContext::new(
                id.to_string(),
                3,
                events[2].clone(),
                Default::default()
            ),
        ]
    );

    let context = store
        .load_aggregate(id.as_str())
        .await
        .unwrap();

    // check stored aggregate is correct
    assert_eq!(
        context,
        ThisAggregateContext::new(
            id.clone(),
            Customer {
                customer_id: "".to_string(),
                name: "test_event_A".to_string(),
                email: "test A".to_string(),
                addresses: vec!["test B".to_string()]
            },
            3
        )
    );

    Ok(())
}

#[test]
fn test_with_snapshots() {
    tokio_test::block_on(commit_and_load_events(true)).unwrap();
}

#[test]
fn test_no_snapshots() {
    tokio_test::block_on(commit_and_load_events(false)).unwrap();
}

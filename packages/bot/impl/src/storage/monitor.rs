use std::{cell::RefCell, future::Future};
use ic_stable_structures::BTreeMap;
use crate::{
    memory::{get_monitors_memory, Memory}, 
    types::monitor::{Monitor, MonitorId}
};

pub struct MonitorStorage;

thread_local! {
    static MONITORS: RefCell<BTreeMap<MonitorId, Monitor, Memory>> = RefCell::new(
        BTreeMap::init(
            get_monitors_memory()
        )
    );
}

impl MonitorStorage {
    pub fn save(
        id: MonitorId,
        monitor: Monitor
    ) {
        MONITORS.with_borrow_mut(|monitors| {
            monitors.insert(id, monitor)
        });
    }

    #[allow(unused)]
    pub fn load(
        id: &MonitorId
    ) -> Option<Monitor> {
        MONITORS.with_borrow(|monitors| {
            monitors.get(&id)
        })
    }

    pub async fn for_each<F, R>(
        fun: F
    ) where F: Fn(MonitorId, Monitor) -> R,
        R: Future<Output = ()> {
        let futs = MONITORS.with_borrow(|monitors| {
            monitors.iter()
                .map(|(id, m)| fun(id, m))
                .collect::<Vec<_>>()
        });

        futures::future::join_all(futs).await;
    }
}
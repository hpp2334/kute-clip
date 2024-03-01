use kute_clip_core::{
    app_setup::{Clipboard, ClipboardLoop},
    libs::common::{AppActivator, ClipboardPaster},
    misty::build_state_manager,
    repositories::{DatabaseService, DatabaseServiceImpl},
    views::{root::build_view_manager, RootView},
};
use misty_vm::{
    client::SingletonMistyClientPod, controllers::ControllerRet, resources::ResourceUpdateAction,
    services::MistyServiceManager, signals::MistySignal,
};
use serde::Serialize;
use tauri::Manager;

use crate::service_impls::{
    clipboard::{ClipboardImpl, ClipboardLoopImpl},
    common::{AppActivatorImpl, ClipboardPasterImpl},
};

pub static CLIENT: SingletonMistyClientPod<RootView> = SingletonMistyClientPod::new();

#[derive(Serialize)]
pub struct ResourceChange {
    id: u64,
    buf: Option<Vec<u8>>,
}

#[derive(Serialize)]
pub struct CommandRet {
    changed_view: Option<RootView>,
    changed_resources: Vec<ResourceChange>,
}

pub fn apply_ret<E>(ret: Result<ControllerRet<RootView>, E>) -> CommandRet
where
    E: std::fmt::Debug,
{
    let ret = ret.unwrap();

    let changed_resources = ret
        .changed_resources
        .into_iter()
        .map(|action| match action {
            ResourceUpdateAction::Insert(id, buf) => ResourceChange {
                id: *id,
                buf: Some(buf),
            },
            ResourceUpdateAction::Remove(id) => ResourceChange { id: *id, buf: None },
        })
        .collect();

    CommandRet {
        changed_view: ret.changed_view,
        changed_resources,
    }
}

fn build_service_manager() -> MistyServiceManager {
    MistyServiceManager::builder()
        .add(DatabaseService::new(DatabaseServiceImpl::new()))
        .add(Clipboard::new(ClipboardImpl))
        .add(ClipboardLoop::new(ClipboardLoopImpl))
        .add(ClipboardPaster::new(ClipboardPasterImpl))
        .add(AppActivator::new(AppActivatorImpl))
        .build()
}

pub fn setup_misty_client(handle: tauri::AppHandle) {
    let view_manager = build_view_manager();
    let state_manager = build_state_manager();
    let service_manager = build_service_manager();

    CLIENT.reset();
    CLIENT.create(view_manager, state_manager, service_manager);

    CLIENT.on_signal(move |signal| match signal {
        MistySignal::Schedule => {
            handle.emit("signal:schedule", ()).unwrap();
        }
    });
}

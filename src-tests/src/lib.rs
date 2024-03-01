use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use misty_vm_test::{TestApp, TestAppContainer};
use tempdir::TempDir;

use kute_clip_core::{
    actions::{application::action_init_application, keyboard::action_keydown},
    app_setup::{
        app_setup, Clipboard, ClipboardHandler, ClipboardLoop, IClipboard, IClipboardLoop,
    },
    libs::common::{AppActivator, ClipboardPaster, IAppActivator, IClipboardPaster},
    misty::build_state_manager,
    repositories::{DatabaseService, DatabaseServiceImpl},
    shell::IShellHandle,
    views::{root::build_view_manager, RootView},
};
use misty_vm::{
    controllers::MistyController, resources::MistyResourceId, services::MistyServiceManager,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardItem {
    Text(String),
    Image((usize, usize, Vec<u8>)),
}

struct FakeClipboard {
    item: Arc<Mutex<ClipboardItem>>,
    clipboard_handler: Arc<Mutex<Option<Arc<ClipboardHandler>>>>,
}

struct SharedFakeAppStorage {
    app: TestApp<RootView>,
    visible: AtomicBool,
    shortcuts: Arc<Mutex<HashSet<HotKey>>>,
    filepath: Arc<Mutex<Option<PathBuf>>>,
    clipboard: Arc<FakeClipboard>,
    pasted: Arc<Mutex<ClipboardItem>>,
    focus_change_handler: Arc<Mutex<Option<Arc<dyn Fn(bool) + Send + Sync + 'static>>>>,
    shortcut_detect_handler: Arc<Mutex<Option<Arc<dyn Fn(&HotKey) + Send + Sync + 'static>>>>,
}

#[derive(Clone)]
pub struct FakeShellHandle {
    inner: Arc<SharedFakeAppStorage>,
}

pub struct FakeAppRef {
    inner: Arc<SharedFakeAppStorage>,
}

struct FakeClipboardImpl {
    clipboard: Arc<FakeClipboard>,
}
struct FakeClipboardLoopImpl {
    clipboard: Arc<FakeClipboard>,
}
struct FakeAppActivatorImpl;
struct FakeClipboardPasterImpl {
    clipboard: Arc<FakeClipboard>,
    pasted: Arc<Mutex<ClipboardItem>>,
}

impl FakeClipboard {
    fn on_change(&self) {
        let f = self.clipboard_handler.lock().unwrap().clone();
        if let Some(f) = f {
            f.on_change();
        }
    }
}

impl IAppActivator for FakeAppActivatorImpl {
    fn disable_active(&self) {}
    fn enable_active(&self) {}
}

impl IClipboard for FakeClipboardImpl {
    fn text(&self) -> Option<String> {
        let item = self.clipboard.item.lock().unwrap().clone();
        match item {
            ClipboardItem::Text(text) => Some(text),
            ClipboardItem::Image(_) => None,
        }
    }
    fn image(&self) -> Option<(usize, usize, Vec<u8>)> {
        let item = self.clipboard.item.lock().unwrap().clone();
        match item {
            ClipboardItem::Text(_) => None,
            ClipboardItem::Image(tuple) => Some(tuple),
        }
    }
    fn write_text(&self, text: String) {
        self.clipboard.write(ClipboardItem::Text(text));
    }
    fn write_image(&self, width: usize, height: usize, bytes: &[u8]) {
        self.clipboard
            .write(ClipboardItem::Image((width, height, bytes.to_vec())));
    }
}

impl IClipboardLoop for FakeClipboardLoopImpl {
    fn run(&self, handler: ClipboardHandler) {
        let mut w = self.clipboard.clipboard_handler.lock().unwrap();
        *w = Some(Arc::new(handler));
    }
}

impl IClipboardPaster for FakeClipboardPasterImpl {
    fn paste(&self) {
        let current = self.clipboard.get_current();
        let mut w = self.pasted.lock().unwrap();
        *w = current;
    }
}

impl FakeClipboard {
    fn get_current(&self) -> ClipboardItem {
        self.item.lock().unwrap().clone()
    }
    fn write(&self, item: ClipboardItem) {
        {
            let mut w = self.item.lock().unwrap();
            *w = item;
        }

        self.on_change();
    }
}

impl IShellHandle for FakeShellHandle {
    fn on_mw_focus_change(&self, on: impl Fn(bool) + Send + Sync + 'static) {
        let inner = self.inner.clone();
        let mut w = inner.focus_change_handler.lock().unwrap();
        *w = Some(Arc::new(on));
    }

    fn on_shortcut_detect(&self, on: impl Fn(&HotKey) + Send + Sync + 'static) {
        let inner = self.inner.clone();
        let mut w = inner.shortcut_detect_handler.lock().unwrap();
        *w = Some(Arc::new(on));
    }

    fn register_shortcut(&self, shortcut: HotKey) -> bool {
        let inner = self.inner.clone();
        let mut shortcuts = inner.shortcuts.lock().unwrap();

        if !shortcuts.contains(&shortcut) {
            shortcuts.insert(shortcut);
            true
        } else {
            false
        }
    }

    fn unregister_shortcut(&self, shortcut: HotKey) {
        let inner = self.inner.clone();
        let mut shortcuts = inner.shortcuts.lock().unwrap();
        shortcuts.remove(&shortcut);
    }

    fn show(&self) {
        let inner = self.inner.clone();
        let prev = inner
            .visible
            .swap(true, std::sync::atomic::Ordering::SeqCst);
        if prev {
            return;
        }

        let f = inner.focus_change_handler.lock().unwrap().clone();
        if let Some(f) = f {
            f(true);
        }
    }

    fn hide(&self) {
        let inner = self.inner.clone();
        let prev = inner
            .visible
            .swap(false, std::sync::atomic::Ordering::SeqCst);
        if !prev {
            return;
        }

        let f = inner.focus_change_handler.lock().unwrap().clone();
        if let Some(f) = f {
            f(false);
        }
    }

    fn mw_visible(&self) -> bool {
        let inner = self.inner.clone();
        inner.visible.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn blocking_save_file(&self, _file_name: &str) -> Option<std::path::PathBuf> {
        let inner = self.inner.clone();
        let mut filepath = inner.filepath.lock().unwrap();
        filepath.take()
    }
}

fn setup_subscriber() {
    static SETUP_SUBSCRIBER_ONCE: AtomicBool = AtomicBool::new(false);
    let has_setup = SETUP_SUBSCRIBER_ONCE.swap(true, std::sync::atomic::Ordering::SeqCst);
    if has_setup {
        return;
    }

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

impl FakeAppRef {
    pub fn new() -> Self {
        setup_subscriber();
        let app_container = TestAppContainer::<RootView>::new(|recv, state| {
            if let Some(v) = recv.clipboard_history {
                state.clipboard_history = Some(v);
            }
            if let Some(v) = recv.active_clipboard_item {
                state.active_clipboard_item = Some(v);
            }
            if let Some(v) = recv.preference {
                state.preference = Some(v);
            }
        });
        let clipboard_handler = Arc::new(Mutex::new(None));
        let clipboard: Arc<FakeClipboard> = Arc::new(FakeClipboard {
            item: Arc::new(Mutex::new(ClipboardItem::Text(Default::default()))),
            clipboard_handler,
        });
        let pasted = Arc::new(Mutex::new(ClipboardItem::Text(Default::default())));

        let service_manager = MistyServiceManager::builder()
            .add(DatabaseService::new(DatabaseServiceImpl::new()))
            .add(Clipboard::new(FakeClipboardImpl {
                clipboard: clipboard.clone(),
            }))
            .add(ClipboardLoop::new(FakeClipboardLoopImpl {
                clipboard: clipboard.clone(),
            }))
            .add(ClipboardPaster::new(FakeClipboardPasterImpl {
                clipboard: clipboard.clone(),
                pasted: pasted.clone(),
            }))
            .add(AppActivator::new(FakeAppActivatorImpl))
            .build();

        Self {
            inner: Arc::new(SharedFakeAppStorage {
                app: TestApp::new(
                    build_view_manager(),
                    service_manager,
                    build_state_manager(),
                    app_container,
                ),
                visible: AtomicBool::new(true),
                shortcuts: Default::default(),
                filepath: Default::default(),
                clipboard: clipboard.clone(),
                pasted,
                focus_change_handler: Default::default(),
                shortcut_detect_handler: Default::default(),
            }),
        }
    }

    pub fn setup(&self) -> TempDir {
        let tmp_dir = TempDir::new("kute_clip").unwrap();
        self.setup_from_app_dir(&tmp_dir);
        tmp_dir
    }

    pub fn setup_from_app_dir(&self, dir: &TempDir) {
        let inner = self.inner.clone();
        let app_dir = dir.path().to_path_buf();
        app_setup(self.shell(), inner.app.app().accessor()).unwrap();
        inner
            .app
            .app()
            .call_controller(action_init_application, (self.shell(), app_dir));
    }

    pub fn assert_reload_from_app_dir(&self, dir: &TempDir) {
        // before reload
        {
            let history = self.state().clipboard_history.clone().unwrap_or_default();
            if !history.items.is_empty() {
                let ns = self.state().active_clipboard_item.unwrap().clone();
                assert_eq!(ns.item.is_some(), true);
                assert!(history.items.into_iter().any(|v| v.id == ns.id));
            }
        }

        let old_state = self.state();

        let app2 = FakeAppRef::new();
        app2.setup_from_app_dir(dir);
        let new_state = app2.state();

        {
            let lhs = old_state.clipboard_history.clone().unwrap_or_default();
            let rhs = new_state.clipboard_history.clone().unwrap_or_default();
            assert_eq!(lhs.items.len(), rhs.items.len());

            for i in 0..lhs.items.len() {
                let lhs = lhs.items[i].clone();
                let rhs = rhs.items[i].clone();
                assert_eq!(lhs.typ, rhs.typ);
                assert_eq!(lhs.text, rhs.text);
                assert_eq!(lhs.width, rhs.width);
                assert_eq!(lhs.height, rhs.height);
                assert_eq!(lhs.shortcut, rhs.shortcut);
            }
        }

        let new_history = new_state.clipboard_history.clone().unwrap_or_default();
        {
            if !new_history.items.is_empty() {
                let ns = new_state.active_clipboard_item.unwrap().clone();
                assert_eq!(ns.id, new_history.items[0].id);
                assert!(ns.item.is_some());
            }
        }
    }

    pub fn change_focused(&self, focused: bool) {
        let f = self.inner.focus_change_handler.lock().unwrap().clone();
        if let Some(f) = f {
            f(focused);
        }
    }

    pub fn mw_visible(&self) -> bool {
        self.shell().mw_visible()
    }

    pub fn show(&self) {
        self.shell().show();
    }

    pub fn hide(&self) {
        self.shell().hide();
    }

    pub fn pasted(&self) -> ClipboardItem {
        self.inner.pasted.lock().unwrap().clone()
    }

    pub fn trigger_copy(&self, item: ClipboardItem) {
        self.inner.clipboard.write(item);
    }

    pub fn keypress(&self, code: Code, mods: Option<Modifiers>) {
        let f = self.inner.shortcut_detect_handler.lock().unwrap().clone();

        if mods.is_none() {
            self.call_controller(action_keydown, (self.shell(), code));
        }
        if let Some(f) = f {
            let shortcut = HotKey::new(mods, code);
            f(&shortcut);
        }
    }

    pub fn shell(&self) -> FakeShellHandle {
        FakeShellHandle {
            inner: self.inner.clone(),
        }
    }

    pub fn call_controller<Controller, Arg, E>(&self, controller: Controller, arg: Arg)
    where
        Controller: MistyController<Arg, E>,
        E: std::fmt::Debug,
    {
        self.inner.app.app().call_controller(controller, arg)
    }

    pub fn wait(&self) {
        std::thread::sleep(Duration::from_millis(200));
    }

    pub fn state(&self) -> RootView {
        self.inner.app.state()
    }

    pub fn resource(&self, id: u64) -> Vec<u8> {
        self.inner
            .app
            .app()
            .get_resource(MistyResourceId::wrap(id))
            .unwrap()
    }
}

pub fn trigger_copy_text_1_to_10(app: &FakeAppRef) {
    app.trigger_copy(ClipboardItem::Text("1".to_string()));
    app.trigger_copy(ClipboardItem::Text("2".to_string()));
    app.trigger_copy(ClipboardItem::Text("3".to_string()));
    app.trigger_copy(ClipboardItem::Text("4".to_string()));
    app.trigger_copy(ClipboardItem::Text("5".to_string()));
    app.trigger_copy(ClipboardItem::Text("6".to_string()));
    app.trigger_copy(ClipboardItem::Text("7".to_string()));
    app.trigger_copy(ClipboardItem::Text("8".to_string()));
    app.trigger_copy(ClipboardItem::Text("9".to_string()));
    app.trigger_copy(ClipboardItem::Text("10".to_string()));
}

use std::path::PathBuf;

use misty_vm::controllers::MistyControllerContext;

use crate::{repositories::init_repositories, result::KCResult, shell::IShellHandle};

use super::{clipboard_changed::init_clipboard_histories, preference::init_preference};

pub fn action_init_application(
    cx: MistyControllerContext,
    (shell, app_dir): (impl IShellHandle, PathBuf),
) -> KCResult<()> {
    let cx = cx.handle();
    init_repositories(cx, app_dir)?;
    init_preference(cx, shell)?;
    init_clipboard_histories(cx)?;

    Ok(())
}

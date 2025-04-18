use std::sync::OnceLock;

use tokio::runtime::{
    self,
    Runtime,
};

use crate::ui::SETTINGS;

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        runtime::Builder::new_multi_thread()
            .worker_threads(SETTINGS.threads() as usize)
            .enable_all()
            .build()
            .expect("Failed to create runtime")
    })
}

use frontend_common::FrontendCommands;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, FrontendCommands)]
pub struct BackendHandle;

pub fn use_backend() -> BackendHandle {
    BackendHandle
}

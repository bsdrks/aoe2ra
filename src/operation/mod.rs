pub mod action;
pub mod chat;
pub mod operation_type;
pub mod sync;
pub mod view_lock;

pub use {
    action::Action,
    chat::Chat,
    operation_type::OperationType,
    sync::Sync,
    view_lock::ViewLock,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Operation {
    Action(Action),
    Chat(Chat),
    r#Sync(r#Sync),
    ViewLock(ViewLock),
    Unknown,
}

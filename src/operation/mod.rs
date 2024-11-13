pub mod action;
pub mod chat;
pub mod operation_type;
pub mod sync;
pub mod view_lock;

use crate::parser::{
    Parse,
    Parser,
};
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
}

impl Parse for Operation {
    fn parse(parser: &mut Parser) -> Self {
        let operation_type = OperationType::parse(parser);

        match operation_type {
            OperationType::Action => Action::parse(parser).into(),
            OperationType::Sync => r#Sync::parse(parser).into(),
            OperationType::ViewLock => ViewLock::parse(parser).into(),
            OperationType::Chat => Chat::parse(parser).into(),
        }
    }
}

impl From<Action> for Operation {
    fn from(value: Action) -> Self {
        Self::Action(value)
    }
}

impl From<Chat> for Operation {
    fn from(value: Chat) -> Self {
        Self::Chat(value)
    }
}

impl From<r#Sync> for Operation {
    fn from(value: r#Sync) -> Self {
        Self::r#Sync(value)
    }
}

impl From<ViewLock> for Operation {
    fn from(value: ViewLock) -> Self {
        Self::ViewLock(value)
    }
}

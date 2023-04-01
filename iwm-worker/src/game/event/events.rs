#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Events {
    PlayerJoin,
    UpdateEntityPosition,
    UpdateEntityPositionandRotation,
    UpdateEntityRotation,
    PlayerQuit,
}

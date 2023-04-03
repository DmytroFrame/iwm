#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Events {
    PlayerJoin,
    UpdateEntityPosition,
    UpdateEntityPositionAndRotation,
    UpdateEntityRotation,
    PlayerQuit,
}

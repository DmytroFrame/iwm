use std::pin::Pin;

use crate::{
    game::event::manager::EventContext,
    net::protocol::{
        client::play::update_entity_position::UpdateEntityPosition, package_output::OutputPackage,
    },
};

pub(crate) fn update_entity_position_handler<'a>(
    ctx: EventContext<'a>,
) -> Pin<Box<dyn std::future::Future<Output = ()> + 'a + Send>> {
    Box::pin(async move {
        if ctx.event_entity_id == ctx.handler_entity_id {
            return;
        }

        let main_session = ctx.process.get_palyer_by_id(ctx.handler_entity_id).unwrap();
        let second_session = ctx.process.get_palyer_by_id(ctx.event_entity_id).unwrap();

        main_session
            .stream
            .output
            .send(OutputPackage::UpdateEntityPosition(
                UpdateEntityPosition::from_session(&second_session),
            ))
            .await
            .unwrap();
    })
}

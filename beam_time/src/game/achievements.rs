use beam_logic::level::Level;
use uuid::{uuid, Uuid};

use crate::app::App;

const BASIC_ROUTING: Uuid = uuid!("58fc60ca-3831-4f27-a29a-b4878a5dd68a");
const NOT_GATE: Uuid = uuid!("cafeb123-66dc-4b04-b560-5cf80868cae4");

pub fn award_campaign_achievements(app: &App, level: &Level) {
    match level.id {
        BASIC_ROUTING => app.award_achievement("its_beam_time"),
        NOT_GATE => app.award_achievement("turing_complete"),
        _ => {}
    }
}

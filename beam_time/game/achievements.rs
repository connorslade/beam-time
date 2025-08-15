use uuid::{Uuid, uuid};

use crate::app::App;

const BASIC_ROUTING: Uuid = uuid!("58fc60ca-3831-4f27-a29a-b4878a5dd68a");
const NOT_GATE: Uuid = uuid!("cafeb123-66dc-4b04-b560-5cf80868cae4");

pub fn award_campaign_achievements(app: &App, id: Uuid) {
    match id {
        BASIC_ROUTING => app.steam.award_achievement("its_beam_time"),
        NOT_GATE => app.steam.award_achievement("turing_complete"),
        _ => {}
    }
}

pub fn award_sandbox_playtime_achievements(app: &App, playtime: u64) {
    const HOUR: u64 = 60 * 60;

    if playtime >= HOUR {
        app.steam.award_achievement("sandbox_1hr");
    }

    if playtime >= 24 * HOUR {
        app.steam.award_achievement("sandbox_24hr");
    }
}

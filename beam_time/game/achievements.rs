use uuid::{Uuid, uuid};

use crate::app::App;

pub fn award_campaign_achievements(app: &App, id: Uuid, (cost, latency): (u32, u32)) {
    let award = |name| app.integrations.award_achievement(name);

    const BASIC_ROUTING: Uuid = uuid!("58fc60ca-3831-4f27-a29a-b4878a5dd68a");
    const NOT_GATE: Uuid = uuid!("cafeb123-66dc-4b04-b560-5cf80868cae4");
    const XOR_GATE: Uuid = uuid!("3eb940dd-1f76-46c5-8aea-800ae0951364");
    const BASIC_OSCILLATOR: Uuid = uuid!("aa28086a-564e-46d3-9233-894c157d92fe");

    match id {
        BASIC_ROUTING => award("its_beam_time"),
        NOT_GATE => award("turing_complete"),
        XOR_GATE if cost <= 2000 => award("cheapest_xor"),
        BASIC_OSCILLATOR if latency <= 2 => award("low_latency_oscillator"),
        _ => {}
    }
}

pub fn award_sandbox_playtime_achievements(app: &App, playtime: u64) {
    const HOUR: u64 = 60 * 60;

    if playtime >= HOUR {
        app.integrations.award_achievement("sandbox_1hr");
    }

    if playtime >= 24 * HOUR {
        app.integrations.award_achievement("sandbox_24hr");
    }
}

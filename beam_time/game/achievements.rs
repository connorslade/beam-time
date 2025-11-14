use uuid::{Uuid, uuid};

use crate::app::App;

pub fn award_campaign_achievements(app: &App, id: Uuid, (cost, latency): (u32, u32)) {
    let award = |name| app.integrations.award_achievement(name);

    const BASIC_ROUTING: Uuid = uuid!("58fc60ca-3831-4f27-a29a-b4878a5dd68a");
    const NOT_GATE: Uuid = uuid!("cafeb123-66dc-4b04-b560-5cf80868cae4");
    const XOR_GATE: Uuid = uuid!("3eb940dd-1f76-46c5-8aea-800ae0951364");
    const BASIC_OSCILLATOR: Uuid = uuid!("aa28086a-564e-46d3-9233-894c157d92fe");
    const RS_LATCH: Uuid = uuid!("f4444a1c-d2a8-4a7a-b311-ec39322c1776");
    const SERIAL_BUS: Uuid = uuid!("151094cc-afcf-4825-87e2-72a565c18162");
    const MULTIPLIER: Uuid = uuid!("898fede9-9bb5-455e-9671-4671d2bbdae3");
    const ADDER_SUBTRACTOR: Uuid = uuid!("597b1d3d-0441-460f-870d-ed77749a07d7");
    const TRIPLE_IT: Uuid = uuid!("c04be8ac-0a2e-44c8-b82b-4f1ef2566244");

    if matches!(id, MULTIPLIER | ADDER_SUBTRACTOR | TRIPLE_IT)
        && app.level_solved(&MULTIPLIER)
        && app.level_solved(&ADDER_SUBTRACTOR)
        && app.level_solved(&TRIPLE_IT)
    {
        award("binary_arithmetic");
    }

    match id {
        BASIC_ROUTING => award("its_beam_time"),
        NOT_GATE => award("turing_complete"),
        XOR_GATE if cost <= 2000 => award("cheapest_xor"),
        BASIC_OSCILLATOR if latency <= 2 => award("low_latency_oscillator"),
        RS_LATCH if cost <= 1500 => award("compact_memory_cell"),
        SERIAL_BUS if cost <= 64000 => award("cheep_serial_bus"),
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

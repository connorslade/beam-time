use log::warn;
use once_cell::sync::Lazy;

use crate::level::Level;

pub macro default_level {
    ($name:expr) => {
        Level::load_slice(include_bytes!(concat!("../../assets/levels/", $name)))
    },
    ($($name:expr),* $(,)?) => {{
        let mut out = Vec::new();
        $(
            match default_level!($name) {
                Ok(x) => out.push(x),
                Err(err) => warn!("Error loading level `{}`: {err}", $name)
            };
        )*
        return out;
    }}
}

pub static DEFAULT_LEVELS: Lazy<Vec<Level>> = Lazy::new(|| {
    default_level!(
        "accumulator.ron",
        "adder.ron",
        "adder_subtractor.ron",
        "and_gate.ron",
        "another_or_gate.ron",
        "barrel_shifter.ron",
        "basic_oscillator.ron",
        "basic_routing.ron",
        "bidirectional_counter.ron",
        "binary_decoder.ron",
        "binary_encoder.ron",
        "bit_reverse.ron",
        "comparator.ron",
        "conway_life.ron",
        "count_ones.ron",
        "counter.ron",
        "data_latch.ron",
        "double_it.ron",
        "edge_detectors.ron",
        "even_oscillators.ron",
        "find_first_set.ron",
        "four_bit_not.ron",
        "full_adder.ron",
        "grey_decode.ron",
        "grey_encode.ron",
        "half_adder.ron",
        "hamming_correction.ron",
        "hamming_generation.ron",
        "imply_gate.ron",
        "large_multiplexer.ron",
        "multiplier.ron",
        "mux_demux.ron",
        "not_gate.ron",
        "one_tick_clock.ron",
        "or_gate.ron",
        "paralel_to_serial.ron",
        "parity_bit.ron",
        "program_counter.ron",
        "pulse_width_modulation.ron",
        "random_access_memory.ron",
        "read_only_memory.ron",
        "rs_latch.ron",
        "seven_segment_driver.ron",
        "shift_register.ron",
        "slightly_less_basic_routing.ron",
        "stack.ron",
        "synchronization.ron",
        "t_flip_flop.ron",
        "triple_it.ron",
        "two_tick_clock.ron",
        "two_way_multiplexer.ron",
        "twos_complement.ron",
        "xor_gate.ron",
    )
});

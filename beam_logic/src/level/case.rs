use serde::Deserialize;

type Bits = Vec<bool>;

#[derive(Clone, Debug, Deserialize)]
pub enum TestCase {
    /// The checker will wait until the board state reaches a state its been at
    /// before. At this point, due the the (hopefully) deterministic nature of
    /// the simulation, only states within the cycle can ever be reached again
    /// without altering the inputs. Then the output states in the cycle states
    /// will be compared to the expected values.
    Cycle { lasers: Bits, detectors: Vec<Bits> },
    /// This checker allows you to define specific outputs that will immediately
    /// cause the case to pass or fail.
    Event {
        lasers: Bits,

        default: EventType,
        pass: Vec<Bits>,
        neutral: Vec<Bits>,
        fail: Vec<Bits>,
    },
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum EventType {
    Pass,
    #[default]
    Neutral,
    Fail,
}

impl TestCase {
    pub fn lasers(&self) -> &Bits {
        match self {
            TestCase::Cycle { lasers, .. } | TestCase::Event { lasers, .. } => lasers,
        }
    }
}

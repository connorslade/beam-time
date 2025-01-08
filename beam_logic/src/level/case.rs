use serde::Deserialize;

use super::{ElementLocation, Level};

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

pub struct CasePreview<'a, 'b> {
    laser: (&'a [bool], &'b [ElementLocation]),
    detector: (&'a [bool], &'b [ElementLocation]),
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

    pub fn preview<'a, 'b>(&'a self, level: &'b Level) -> Option<CasePreview<'a, 'b>> {
        match self {
            TestCase::Cycle { lasers, detectors } if detectors.len() == 1 => Some(CasePreview {
                laser: (lasers, &level.tests.lasers),
                detector: (&detectors[0], &level.tests.detectors),
            }),
            _ => None,
        }
    }
}

impl CasePreview<'_, '_> {
    pub fn elements(&self) -> usize {
        self.detector.0.len() + self.laser.0.len()
    }

    pub fn detector(&self) -> impl Iterator<Item = (&bool, &ElementLocation)> {
        self.detector.0.iter().zip(self.detector.1.iter())
    }

    pub fn laser(&self) -> impl Iterator<Item = (&bool, &ElementLocation)> {
        self.laser.0.iter().zip(self.laser.1.iter())
    }
}

impl EventType {
    pub fn classify(
        pass: &[Vec<bool>],
        neutral: &[Vec<bool>],
        fail: &[Vec<bool>],
        default: Self,
        outputs: &Vec<bool>,
    ) -> Self {
        if pass.contains(outputs) {
            EventType::Pass
        } else if neutral.contains(outputs) {
            EventType::Neutral
        } else if fail.contains(outputs) {
            EventType::Fail
        } else {
            default
        }
    }
}

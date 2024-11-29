use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use clone_macro::clone;
use parking_lot::{Condvar, Mutex, MutexGuard};

use super::state::BeamState;

pub struct SimulationState {
    inner: Arc<(Mutex<InnerSimulationState>, Condvar)>,
}

#[derive(Clone, Copy)]
pub struct RuntimeConfig {
    pub time_per_tick: Duration,
    pub running: bool,
}

pub struct InnerSimulationState {
    pub beam: Option<BeamState>,
    pub runtime: RuntimeConfig,

    kill: bool,
}

impl SimulationState {
    pub fn new() -> Self {
        let inner = Arc::new((
            Mutex::new(InnerSimulationState {
                beam: None,
                runtime: RuntimeConfig {
                    time_per_tick: Duration::from_secs_f32(1.0 / 20.0),
                    running: false,
                },
                kill: false,
            }),
            Condvar::new(),
        ));

        // todo: shutdown logic somehow?
        thread::spawn(clone!([inner], move || {
            let (mutex, cond) = &*inner;

            loop {
                let mut state = mutex.lock();

                // wait for running to be true
                while !state.runtime.running && !state.kill {
                    cond.wait(&mut state);
                }

                if state.kill {
                    break;
                }

                let timestamp = Instant::now();
                if let Some(beam) = &mut state.beam {
                    beam.tick();

                    let runtime = state.runtime;
                    drop(state);

                    if !runtime.running {
                        continue;
                    }

                    let elapsed = timestamp.elapsed();
                    if elapsed < runtime.time_per_tick {
                        thread::sleep(runtime.time_per_tick - elapsed);
                    }
                }
            }
        }));

        Self { inner }
    }

    pub fn get(&self) -> MutexGuard<InnerSimulationState> {
        self.inner.0.lock()
    }

    pub fn notify_running(&self) {
        self.inner.1.notify_all();
    }
}

impl Drop for SimulationState {
    fn drop(&mut self) {
        self.get().kill = true;
        self.notify_running();
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::new()
    }
}

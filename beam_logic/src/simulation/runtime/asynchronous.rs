use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use clone_macro::clone;
use parking_lot::{Condvar, Mutex, MutexGuard};

use crate::simulation::state::BeamState;

pub struct AsyncSimulationState {
    inner: Arc<(Mutex<InnerAsyncSimulationState>, Condvar)>,
}

#[derive(Clone, Copy)]
pub struct RuntimeConfig {
    pub time_per_tick: Duration,
    pub running: bool,
}

pub struct InnerAsyncSimulationState {
    pub beam: Option<BeamState>,
    pub runtime: RuntimeConfig,
    pub tick_length: Duration,

    kill: bool,
}

impl AsyncSimulationState {
    pub fn new() -> Self {
        let inner = Arc::new((
            Mutex::new(InnerAsyncSimulationState {
                beam: None,
                runtime: RuntimeConfig {
                    time_per_tick: Duration::from_secs_f32(1.0 / 20.0),
                    running: false,
                },
                tick_length: Duration::default(),
                kill: false,
            }),
            Condvar::new(),
        ));

        thread::spawn(clone!([inner], move || {
            let (mutex, cond) = &*inner;

            loop {
                let mut state = mutex.lock();

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

                    if !runtime.running {
                        continue;
                    }

                    let elapsed = timestamp.elapsed();
                    state.tick_length = elapsed;
                    drop(state);
                    
                    if elapsed < runtime.time_per_tick {
                        thread::sleep(runtime.time_per_tick - elapsed);
                    }
                }
            }
        }));

        Self { inner }
    }

    pub fn get(&self) -> MutexGuard<InnerAsyncSimulationState> {
        self.inner.0.lock()
    }

    pub fn notify_running(&self) {
        self.inner.1.notify_all();
    }
}

impl Drop for AsyncSimulationState {
    fn drop(&mut self) {
        self.get().kill = true;
        self.notify_running();
    }
}

impl Default for AsyncSimulationState {
    fn default() -> Self {
        Self::new()
    }
}

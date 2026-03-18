use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use tracing::{debug, error, warn};

static TEMPERATURE: OnceLock<Mutex<Temperature>> = OnceLock::new();

#[derive(Debug)]
struct Temperature {
    path: String,
    last_value: Option<LastValue>,
}

#[derive(Debug)]
struct LastValue {
    value: f32,
    time: Instant,
}

impl Temperature {
    const fn new(path: String) -> Self {
        Self {
            path,
            last_value: None,
        }
    }

    fn get(&mut self) -> f32 {
        if let Some(last) = &self.last_value
            && last.time.elapsed() < Duration::from_secs_f32(1.5)
        {
            return last.value;
        }

        loop {
            let temp_str = match std::fs::read_to_string(&self.path) {
                Ok(str) => str,
                Err(err) => {
                    error!(
                        "Read error for temperature file at {}: {err}; retrying in 10 seconds",
                        self.path
                    );
                    std::thread::sleep(Duration::new(10, 0));
                    continue;
                }
            };

            let temp_val: i32 = match temp_str.parse() {
                Ok(val) => val,
                Err(err) => {
                    warn!(
                        "Cannot parse i32 from {temp_str}, read from {}: {err}; retrying in one second",
                        self.path
                    );
                    std::thread::sleep(Duration::new(1, 0));
                    continue;
                }
            };

            #[allow(
                clippy::cast_precision_loss,
                reason = "Value will be small, and precision is not necessary"
            )]
            let fahrenheit = (temp_val as f32 * 0.001).mul_add(1.8, 32.0);

            self.last_value = Some(LastValue {
                value: fahrenheit,
                time: Instant::now(),
            });

            debug!("Read temperature {temp_str}/1000 \u{00B0}C, {fahrenheit}, \u{00B0}F");

            return fahrenheit;
        }
    }
}

/// # Panics
///
/// Will panic if already set.
pub fn init_temperature(path: String) {
    // TODO: Add server info sending here
    TEMPERATURE
        .set(Mutex::new(Temperature::new(path)))
        .expect("Temperature already initialized.");
}

/// # Panics
///
/// Will panic if not initialized.
pub fn get_temperature() -> f32 {
    let mutex = TEMPERATURE.get().expect("Temperature not initialized.");
    let mut guard = mutex.lock().expect("Temperature mutex failed");
    guard.get()
}

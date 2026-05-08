use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Anonymous telemetry events. No PII, no wallet contents, no identifying info.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    AppStart,
    AppShutdown,
    InterviewQuestionAsked,
    InterviewSessionCompleted,
    CandidateGenerated,
    CrackingStarted,
    CrackingFinished,
    RecoverySucceeded,
}

pub trait TelemetrySink: Send + Sync {
    fn emit(&self, event: Event);
}

pub struct Telemetry {
    enabled: bool,
    sink: Arc<dyn TelemetrySink>,
}

impl Telemetry {
    pub fn new<S: TelemetrySink + 'static>(enabled: bool, sink: S) -> Self {
        Self {
            enabled,
            sink: Arc::new(sink),
        }
    }

    pub fn record(&self, event: Event) {
        if self.enabled {
            self.sink.emit(event);
        }
    }
}

/// In-memory sink for tests.
#[derive(Clone, Default)]
pub struct TestSink {
    inner: Arc<Mutex<Vec<Event>>>,
}

impl TestSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> Vec<Event> {
        self.inner.lock().unwrap().clone()
    }
}

impl TelemetrySink for TestSink {
    fn emit(&self, event: Event) {
        self.inner.lock().unwrap().push(event);
    }
}

use phoenix_core::telemetry::{Event, Telemetry, TestSink};

#[test]
fn telemetry_disabled_drops_events() {
    let sink = TestSink::new();
    let telem = Telemetry::new(false, sink.clone());

    telem.record(Event::AppStart);
    telem.record(Event::InterviewQuestionAsked);

    assert_eq!(sink.events(), Vec::<Event>::new());
}

#[test]
fn telemetry_enabled_forwards_events() {
    let sink = TestSink::new();
    let telem = Telemetry::new(true, sink.clone());

    telem.record(Event::AppStart);
    telem.record(Event::InterviewQuestionAsked);

    assert_eq!(
        sink.events(),
        vec![Event::AppStart, Event::InterviewQuestionAsked]
    );
}

#[test]
fn events_are_serializable() {
    let event = Event::AppStart;
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("AppStart"));
}

use unit::event::{self, Event};

#[test]
fn typed_frames_roundtrip_arbitrary_names_and_reject_truncation() {
    let event = Event::Bound {
        unit: "worker\"\nλ",
        task: 22,
        team: 7,
    };
    let frame = event::encode(&event).unwrap();
    assert_eq!(
        event::decode(frame.as_bytes()).unwrap(),
        Event::Bound {
            unit: "worker\"\nλ".to_string(),
            task: 22,
            team: 7
        }
    );
    for end in 0..frame.len() {
        assert!(event::decode(&frame.as_bytes()[..end]).is_err());
    }
    let mut bad = frame.into_bytes();
    bad[6] = b'x';
    assert!(event::decode(&bad).is_err());
}

use rust4ui::StateRegistry;

#[test]
fn test_persistence_roundtrip() {
    let mut state = StateRegistry::new();
    let mut state2 = StateRegistry::new();

    state.set_string("name", "Test".into());
    state.set_f64("volume", 42.0);
    state.set_bool("gpu", true);
    state.set_vec_string("items", vec!["A".into(), "B".into()]);

    let json = state.to_json();
    let restored = StateRegistry::from_json(&json).expect("Десериализация должна работать");

    assert_eq!(restored.get_string("name"), Some("Test"));
    assert_eq!(restored.get_f64("volume"), Some(42.0));
    assert_eq!(restored.get_bool("gpu"), Some(true));
    assert_eq!(
        restored.get_vec_string("items"),
        Some(&vec!["A".to_string(), "B".to_string()])
    );
}

#[test]
fn test_persistence_empty_state() {
    let state = StateRegistry::new();
    let json = state.to_json();
    let restored = StateRegistry::from_json(&json).unwrap();
    assert!(restored.get_string("none").is_none());
}

#[test]
fn test_persistence_multiple_types() {
    let mut state = StateRegistry::new();
    state.set_string("s", "str".into());
    state.set_f64("f", 3.14);
    state.set_i64("i", -42);
    state.set_bool("b", true);
    state.set_usize("u", 99);

    let json = state.to_json();
    let restored = StateRegistry::from_json(&json).unwrap();

    assert_eq!(restored.get_string("s"), Some("str"));
    assert_eq!(restored.get_f64("f"), Some(3.14));
    assert_eq!(restored.get_i64("i"), Some(-42));
    assert_eq!(restored.get_bool("b"), Some(true));
    assert_eq!(restored.get_usize("u"), Some(99));
}

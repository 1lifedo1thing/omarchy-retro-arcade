use omarchy_blast::storage::Settings;
#[test]
fn preferences_records_and_controls_round_trip_without_touching_other_games() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("session.json"), "preserve existing game").unwrap();
    let mut s = Settings::default();
    s.wins[1] = 5;
    s.matches = 8;
    s.bindings[0][0] = "I".into();
    s.save(dir.path()).unwrap();
    let loaded = Settings::load(dir.path()).unwrap();
    assert_eq!(loaded.matches, 8);
    assert_eq!(loaded.wins[1], 5);
    assert_eq!(loaded.bindings[0][0], "I");
    assert_eq!(
        std::fs::read_to_string(dir.path().join("session.json")).unwrap(),
        "preserve existing game"
    );
}
#[test]
fn duplicate_unknown_and_reserved_keys_and_invalid_config_are_rejected() {
    let mut s = Settings::default();
    s.validate().unwrap();
    s.bindings[1][0] = "W".into();
    assert!(s.validate().is_err());
    for key in ["Escape", "Tab", "F1", "nonsense"] {
        s.bindings[1][0] = key.into();
        assert!(s.validate().is_err());
    }
    let mut s = Settings {
        bots: 0,
        ..Settings::default()
    };
    assert!(s.validate().is_err());
    s.bots = 4;
    assert!(s.validate().is_err());
}

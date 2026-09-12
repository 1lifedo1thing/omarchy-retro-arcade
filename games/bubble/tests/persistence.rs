use omarchy_bubble::storage::*;
#[test]
fn progress_best_and_settings_survive_atomic_save_and_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("arcade/bubble.json");
    let mut p = load(&path, 20).unwrap();
    p.unlocked = 8;
    p.selected = 6;
    p.best[6] = 4200;
    p.sound = false;
    p.introduced = true;
    save(&path, &p).unwrap();
    let read = load(&path, 20).unwrap();
    assert_eq!(read.unlocked, 8);
    assert_eq!(read.selected, 6);
    assert_eq!(read.best[6], 4200);
    assert!(!read.sound);
    assert!(read.introduced);
    assert_eq!(
        std::fs::read_dir(path.parent().unwrap()).unwrap().count(),
        1
    );
}
#[test]
fn invalid_save_is_reported_without_rewriting() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bubble.json");
    std::fs::write(&path, "old or corrupt data").unwrap();
    assert!(load(&path, 20).is_err());
    assert_eq!(
        std::fs::read_to_string(path).unwrap(),
        "old or corrupt data"
    );
}
#[test]
fn unknown_version_and_out_of_bounds_progress_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bubble.json");
    let mut p = Progress {
        version: 2,
        ..Default::default()
    };
    save(&path, &p).unwrap();
    assert!(load(&path, 20).is_err());
    p.version = 1;
    p.selected = 19;
    save(&path, &p).unwrap();
    assert!(load(&path, 20).is_err());
}
#[test]
fn appending_levels_preserves_existing_progress() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bubble.json");
    let mut p = Progress {
        unlocked: 19,
        selected: 19,
        ..Default::default()
    };
    p.best[19] = 6000;
    save(&path, &p).unwrap();
    let expanded = load(&path, 21).unwrap();
    assert_eq!(expanded.best.len(), 21);
    assert_eq!(expanded.best[19], 6000);
    assert_eq!(expanded.best[20], 0);
}

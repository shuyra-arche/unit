use unit::GuardSelectionError;

#[path = "../src/guard_selection.rs"]
mod guard_selection;

#[test]
fn guard_selection_rejects_unknown_duplicate_and_unstarted_names() {
    assert_eq!(
        guard_selection::select(&["pci"], &["pci"], |_| true),
        Ok(&["pci"][..])
    );
    assert_eq!(
        guard_selection::select(&["gpu"], &["gpu"], |_| false),
        Err(GuardSelectionError::Unknown)
    );
    assert_eq!(
        guard_selection::select(&["pci"], &[], |_| true),
        Err(GuardSelectionError::Unstarted)
    );
    assert_eq!(
        guard_selection::select(&["pci", "pci"], &["pci"], |_| true),
        Err(GuardSelectionError::Duplicate)
    );
}

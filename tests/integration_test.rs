// Integration test for SigmaOS-automatic-mount-management
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use automatic_mount_management as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}

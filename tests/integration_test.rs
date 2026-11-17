//! Integration tests for SigmaOS-community-contribution-system.

use sigmaos_community_contribution_system_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

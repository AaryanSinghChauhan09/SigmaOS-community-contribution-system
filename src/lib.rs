// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-community-contribution-system: community contribution system component model.
//!
//! Real implementation: systems, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct System {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunityContributionSystemError {
    EmptySystemName,
    DuplicateSystem(String),
    EmptyEventSubject,
}

impl fmt::Display for CommunityContributionSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySystemName => write!(f, "system name must not be empty"),
            Self::DuplicateSystem(id) => write!(f, "duplicate system {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for CommunityContributionSystemError {}

pub struct CommunityContributionSystem {
    pub name: String,
    systems: BTreeMap<String, System>,
    events: BTreeMap<String, Event>,
}

impl CommunityContributionSystem {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            systems: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_system(&mut self, item: System) -> Result<(), CommunityContributionSystemError> {
        if item.name.trim().is_empty() {
            return Err(CommunityContributionSystemError::EmptySystemName);
        }
        let id = item.id.clone();
        if self.systems.contains_key(&id) {
            return Err(CommunityContributionSystemError::DuplicateSystem(id.clone()));
        }
        self.systems.insert(id, item);
        Ok(())
    }

    pub fn system_count(&self) -> usize {
        self.systems.len()
    }

    pub fn get_system(&self, id: &str) -> Option<&System> {
        self.systems.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), CommunityContributionSystemError> {
        if event.subject.trim().is_empty() {
            return Err(CommunityContributionSystemError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_system(id: &str, name: &str) -> System {
        System {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = CommunityContributionSystem::new("CommunityContributionSystem");
        assert_eq!(m.name, "CommunityContributionSystem");
        assert_eq!(m.system_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_system() {
        let mut m = CommunityContributionSystem::new("CommunityContributionSystem");
        m.add_system(sample_system("a1", "Alpha")).unwrap();
        assert_eq!(m.system_count(), 1);
        assert!(m.get_system("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = CommunityContributionSystem::new("CommunityContributionSystem");
        assert!(m.add_system(sample_system("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = CommunityContributionSystem::new("CommunityContributionSystem");
        m.add_system(sample_system("x", "One")).unwrap();
        assert!(m.add_system(sample_system("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = CommunityContributionSystem::new("CommunityContributionSystem");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

use crate::geometry::{ContactEvent, ContactPair, IntersectionEvent};
use crossbeam::channel::Sender;

bitflags::bitflags! {
    #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
    /// Flags affecting the events generated for this collider.
    pub struct ActiveEvents: u32 {
        /// If set, Rapier will call `EventHandler::handle_intersection_event` whenever relevant for this collider.
        const INTERSECTION_EVENTS = 0b0001;
        /// If set, Rapier will call `PhysicsHooks::handle_contact_event` whenever relevant for this collider.
        const CONTACT_EVENTS = 0b0010;
    }
}

impl Default for ActiveEvents {
    fn default() -> Self {
        ActiveEvents::empty()
    }
}

/// Trait implemented by structures responsible for handling events generated by the physics engine.
///
/// Implementors of this trait will typically collect these events for future processing.
pub trait EventHandler: Send + Sync {
    /// Handle an intersection event.
    ///
    /// A intersection event is emitted when the state of intersection between two colliders changes.
    fn handle_intersection_event(&self, event: IntersectionEvent);
    /// Handle a contact event.
    ///
    /// A contact event is emitted when two collider start or stop touching, independently from the
    /// number of contact points involved.
    fn handle_contact_event(&self, event: ContactEvent, contact_pair: &ContactPair);
}

impl EventHandler for () {
    fn handle_intersection_event(&self, _event: IntersectionEvent) {}
    fn handle_contact_event(&self, _event: ContactEvent, _contact_pair: &ContactPair) {}
}

/// A physics event handler that collects events into a crossbeam channel.
pub struct ChannelEventCollector {
    intersection_event_sender: Sender<IntersectionEvent>,
    contact_event_sender: Sender<ContactEvent>,
}

impl ChannelEventCollector {
    /// Initialize a new physics event handler from crossbeam channel senders.
    pub fn new(
        intersection_event_sender: Sender<IntersectionEvent>,
        contact_event_sender: Sender<ContactEvent>,
    ) -> Self {
        Self {
            intersection_event_sender,
            contact_event_sender,
        }
    }
}

impl EventHandler for ChannelEventCollector {
    fn handle_intersection_event(&self, event: IntersectionEvent) {
        let _ = self.intersection_event_sender.send(event);
    }

    fn handle_contact_event(&self, event: ContactEvent, _: &ContactPair) {
        let _ = self.contact_event_sender.send(event);
    }
}

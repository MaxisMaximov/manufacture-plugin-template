// # Events module
// This is where you can define custom Events to send announcements to other Systems and trigger Event Responders
use super::*;

pub struct SampleEvent(pub &'static str);
impl Event for SampleEvent{
    const ID: &'static str = "SampleEvent";
}
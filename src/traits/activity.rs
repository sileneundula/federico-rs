use crate::activity::ActivityTypes;

/// # Activity Trait
/// 
/// Implements common/basic functionality on types.
pub trait Activity {
    /// # Activity Trait
    /// 
    /// Retrieves the activity as an enumerator
    /// 
    /// ## Activity-Types
    /// 
    /// ### Accept
    /// 
    /// #### Properties
    /// 
    /// 1. **id:** `Accept`
    /// 
    /// 2. **properties:** Inherits all properties from Activity.
    /// 
    /// #### Notes
    /// 
    /// > Indicates that the actor accepts the object. The target property can be used in certain circumstances to indicate the context into which the object has been accepted.
    /// 
    /// 
    /// ### Add
    /// 
    /// #### Properties
    /// 
    /// 1. **id:** `Add`
    /// 
    /// 2. **properties:** Inherits all properties from `Activity`
    /// 
    /// #### Notes
    /// 
    /// ### Announce
    /// 
    /// id: `Announce`
    /// 
    /// ### Arrive
    /// 
    /// id: `Arrive`
    /// 
    /// ### Block
    /// 
    /// id: `Block`
    /// 
    /// ### Create
    /// 
    /// id: `Create`
    /// 
    /// ### Delete
    /// 
    /// id: `Delete`
    /// 
    /// ### Dislike
    /// 
    /// id: `Dislike`
    /// 
    /// ### Flag
    /// 
    /// id: `Flag`
    /// 
    /// ### Follow
    /// 
    /// id: `Follow`
    /// 
    /// ### Ignore
    /// 
    /// id: `Ignore`
    /// 
    /// ### Invite
    /// 
    /// id: `Invite`
    /// 
    /// ### Join
    /// 
    /// id: `Join`
    /// 
    /// ### Leave
    /// 
    /// id: `Leave`
    /// 
    /// ### Like
    /// 
    /// id: `Like`
    /// 
    /// ### Listen
    /// 
    /// id: `Listen`
    /// 
    /// ### Move
    /// 
    /// id: `Move`
    /// 
    /// ### Offer
    /// 
    /// id: `Offer`
    /// 
    /// ### Question
    /// 
    /// id: `Question`
    /// 
    /// ### Reject
    /// 
    /// id: `Reject`
    /// 
    /// ### Read
    /// 
    /// id: `Read`
    /// 
    /// ### Remove
    /// 
    /// id: `Remove`
    /// 
    /// ### TentativeReject
    /// 
    /// id: `TentativeReject`
    /// 
    /// ### TentativeAccept
    /// 
    /// id: `TentativeAccept`
    /// 
    /// ### Travel
    /// 
    /// id: `Travel`
    /// 
    /// ### Undo
    /// 
    /// id: `Undo`
    /// 
    /// ### Update
    /// 
    /// id: `Update`
    /// 
    /// ### View
    /// 
    /// id: `View`
    fn activity(&self) -> ActivityTypes;
    /// # Summary Trait
    /// 
    /// Retrieves the summary of the activity
    fn summary(&self) -> String;
    /// # id Trait
    /// 
    /// Retrieves the id of the activity
    fn id(&self) -> String;
}

/// # ActivityStreams Visualize: A Method of Visualizing Data For ActivityPub/ActivityStreams2.0
/// 
/// This trait implements functionality for visualization.
pub trait ActivityStreamsVisualize {
    /// # Visualize ActivityStreams
    /// 
    /// Creates a visualization of the Activity Streams data.
    fn visualize(&self) -> String;
}

pub trait Properties {
    fn add_property(&mut self) -> Self;
    fn property(&mut self);
}
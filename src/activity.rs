//! # Activity
//! 
//! Distributed Event Log (DEL)
//! 
//! 
//! ## HashSet
//! 
//! Use a hashset to create new activity.

use crate::properties::Properties;


pub struct Activity {
    pub activity_type: ActivityTypes, // Semenatics: type
    pub properties: Option<Vec<Properties>>,
}

impl Activity {
    pub fn new(activity_type: ActivityTypes, properties: Vec<Properties>) {
        
    }
}

pub struct ActivityParser {
    parser_version: u16,
    parser_type: ActivityParserType,
}

// pub struct ActivityParserActivityType;

impl ActivityParser {
    pub fn new(activity: Activity) {

    }
    fn parse_activity_type(&self, activity: &Activity) {
        match activity.activity_type {
            ActivityTypes::ACCEPT =>
            ActivityTypes::ADD =>
            ActivityTypes::ANNOUNCE =>
            ActivityTypes::ARRIVE =>
            ActivityTypes::BLOCK =>
            ActivityTypes::CREATE =>
            ActivityTypes::DELETE => 
        }
    }
}

/// # ActivityParserType
/// 
///
pub enum ActivityParserType {
    Federico, // Default
    Other(String),
}

/// # ActivityTypes
/// 
/// Accept: Accepts a follow request, or other data.
pub enum ActivityTypes {
    ACCEPT, // Accept: Indicates the actor accepts the object
    ADD,
    ANNOUNCE,
    ARRIVE,
    BLOCK,
    CREATE,
    DELETE,
    DISLIKE,
    FLAG,
    FOLLOW,
    IGNORE,
    INVITE,
    JOIN,
    LEAVE,
    LIKE,
    LISTEN,
    MOVE,
    OFFER,
    QUESTION,
    REJECT,
    READ,
    REMOVE,
    TENTATIVEREJECT,
    TENTATIVEACCEPT,
    TRAVEL,
    UNDO,
    UPDATE,
    VIEW,

    OTHER(String),
}

/// # ActivityTypesProducer
/// 
/// Deals with formatting and parsing the activity types.
pub struct ActivityTypesProducer {
    ActivityType: ActivityTypes,
}

impl ActivityTypesProducer {
    pub fn new(activity_type: ActivityTypes) -> Self {
        return Self {
            ActivityType: activitytype,
        }
    }
    pub fn format(&self) {
        unimplemented!();
    }
}
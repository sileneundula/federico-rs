
use url::Url;
use crate::actor::Actor;

pub enum Properties {
    ID(Url), // id: 
    TYPE,
    ACTOR(Actor),
    ATTACHMENT,
    ATTRIBUTEDTO,
    AUDIENCE,
    BCC,
    BTO,
    CC,
    CONTEXT,
    CURRENT,
    FIRST,
    GENERATOR,
    ICON,
    IMAGE,
    INREPLYTO,
    INSTRUMENT,
    LAST,
    LOCATION,
    ITEMS,
    ONEOF,
    ANYOF,
    CLOSED,
    ORIGIN,
    NEXT,
    OBJECT,
    PREV,
    PREVIEW,
    RESULT,
    REPLIES,
    TAG,
    TARGET,
    TO,
    URL,
    ACCURACY,
    ALTITUDE,
    CONTENT,
    NAME,
    DURATION,
    HEIGHT,
    HREF,
    HREFLANG,
    PARTOF,
    LATITUDE,
    LONGITUDE,
    MEDIATYPE,
    ENDTIME,
    PUBLISHED,
    STARTTIME,
    RADIUS,
    REL,
    STARTINDEX,
    SUMMARY,
    TOTALITEMS,
    UNITS,
    UPDATED,
    WIDTH,
    SUBJECT,
    RELATIONSHIP,
    DESCRIBES,
    FORMERTYPE,
    DELETED,
    OTHER(String),
}

pub struct AddProperty;

impl AddProperty {
    pub fn new(property: Properties, value: String) {
        match property {
            Properties::ID => 
            _ => panic!("Not Implemented")
        }
    }
    /// URL
    fn id(id: Url) -> Url {
        return id
    }
}
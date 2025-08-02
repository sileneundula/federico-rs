pub enum ObjectTypes {
    ARTICLE,
    AUDIO,
    DOCUMENT,
    EVENT,
    IMAGE,
    NOTE,
    PAGE,
    PLACE,
    PROFILE,
    RELATIONSHIP,
    TOMBSTONE,
    VIDEO,
    OTHER(String)
}

pub enum LinkType {
    MENTION,
}
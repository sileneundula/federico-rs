pub struct ActivityStreams2;

pub enum CoreActivityStreamTypes {
    Object, // all properties are optional
    Link,
    Activity,
    IntransitiveActivity,
    Collection,
    OrderedCollection,
    CollectionPage,
    OrderedCollectionPage,
}
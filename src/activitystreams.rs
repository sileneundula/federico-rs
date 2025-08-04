/// # ActivityStreams 2.0
/// 
/// ActivityStreams is the media content/format that the data is encoded in.
pub struct ActivityStreams2;

/// # CoreActivityStreamTypes
/// 
/// These contain:
/// - Object: Describes an object of any kind. The Object type serves as the base type for most of the other kinds of objects defined in the Activity Vocabulary, including other Core types such as Activity, IntransitiveActivity, Collection, and OrderedCollection.
/// - 
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
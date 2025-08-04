//! # Client To Server (C2S)
//! 
//! Client to server interaction takes place through clients posting Activities to an actor's outbox.
//! 
//! To do this, clients MUST discover the URL of the actor's outbox from their profile and then MUST make an HTTP POST request to this URL with the content-type of:
//! 
//! - application/ld+json; profile="https://www.w3.org/ns/activitystreams"
//! 
//! or
//! 
//! - application/activity+json
//! 
//! The request MUST be authenticated with the credentials of the user to whom the outbox belongs. The body of the `POST` request MUST contain a single Activity (which MAY contain embedded objects), or a single non-Activity object which will be wrapped in a Create Activity by the Server.
//! 
//! If an Activity is submitted with a value in the id property, server MUST ignore this and generate a new `id` for the Activity. Servers MUST return a 201 Created HTTP Code, and unless the activity is transient, MUST include the new `id` in the Location Header

pub struct ClientToServer;

pub struct Client;
//! # Actor
//! 
//! A fundamental primitive component of the social web.
//! 
//! It is similar to, but more powerful and flexible than, the traditional “account” of previous social media efforts
//! 

/// # Actor
/// An Actor represents a user or entity in the ActivityPub protocol.
/// 
/// It includes an id, name, and the inbox/outbox as the main components.
/// 
/// It also includes followers/following.
/// 
/// Finally, it includes preferred_username, summary, icon, url, public_key, and type_
pub struct Actor {
    pub id: String,
    pub name: String,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub preferred_username: String,
    pub summary: String,
    pub icon: String,
    pub url: String,
    pub public_key: String,
    pub type_: String, // e.g., "Person", "Group", "Organization"}
}

pub struct ActorBuilder {
    pub id: String,
    pub name: String,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub preferred_username: String,
    pub summary: String,
    pub icon: String,
    pub url: String,
    pub public_key: String,
    pub type_: String, // e.g., "Person", "Group", "Organization"
}

impl ActorBuilder {
    pub fn new(&mut self) -> Self {
        ActorBuilder {
            id: String::new(),
            name: String::new(),
            inbox: String::new(),
            outbox: String::new(),
            followers: String::new(),
            following: String::new(),
            preferred_username: String::new(),
            summary: String::new(),
            icon: String::new(),
            url: String::new(),
            public_key: String::new(),
            type_: String::new(),
        }
    pub fn id(&mut self, id: String) {
        self.id = id;
    }
    pub fn name(&mut self, name: String) {
        self.id = id;
    }
    pub fn inbox(&mut self, inbox: String) {
        self.id = id;
    }
    pub fn outbox(&mut self, outbox: String) {
        self.outbox = outbox;
    }
    pub fn followers(&mut self, followers: String) {
        self.followers = followers;
    }
    pub fn following(&mut self, following: String) {
        self.following = following;
    }
    pub fn preferred_username(&mut self, preferred_username: String) {
        self.preferred_username = preferred_username;
    }
    pub fn summary(&mut self, summary: String) {
        self.summary = summary;
    }
    pub fn icon(&mut self, icon: String) {
        self.icon = icon;
    }
    pub fn url(&mut self, url: String) {
        self.url = url;
    }
    pub fn public_key(&mut self, public_key: String) {
        self.public_key = public_key;
    }
    pub fn type_(&mut self, type_: String) {
        self.type_ = type_;
    }

    pub fn build(self) -> Actor {
        Actor {
            id: self.id,
            name: self.name,
            inbox: self.inbox,
            outbox: self.outbox,
            followers: self.followers,
            following: self.following,
            preferred_username: self.preferred_username,
            summary: self.summary,
            icon: self.icon,
            url: self.url,
            public_key: self.public_key,
            type_: self.type_,
        }
    }
}

impl Actor {
    pub fn new(
        id: String,
        name: String,
        inbox: String,
        outbox: String,
        followers: String,
        following: String,
        preferred_username: String,
        summary: String,
        icon: String,
        url: String,
        public_key: String,
        type_: String,
    ) -> Self {
        Actor {
            id,
            name,
            inbox,
            outbox,
            followers,
            following,
            preferred_username,
            summary,
            icon,
            url,
            public_key,
            type_,
        }
    }
}

pub enum ActorTypes {
    APPLICATION,
    GROUP,
    ORGANIZATION,
    PERSON,
    SERVICE,
    OTHER(String),
}
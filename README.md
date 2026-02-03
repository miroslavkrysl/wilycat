# wilycat

Note-oriented life organizer: **note-taking**, **calendar**, and **chat**, with an emphasis on effortless search and organization, security, decentralization, and collaboration.

> [!CAUTION]
> WIP - this project is in very early work-in-progress, see [project roadmap](#project-roadmap) for list of implemented and planned features. 

## Vision & Principles

- Main purpose of this application:
  1. **Note-taking**
  2. **Calendar events/reminders**
  3. **Chat**
- _Note-oriented_
  - The main domain concept is a note - called **Note** - whether it is a note, event or chat message
  - Every **Note** is a part of a continuous stream - called **Channel**
  - There is no other structure or hierarchy
- _Powerful searching_
  - Fulltext search
  - Tag-based categorization by topics
  - Searching presets
- _Decentralized_
  - The data can live on multiple clients
  - There is no central server or authority
  - Every client is independent and can act on its own
  - Clients synchronize data between themselves to be up to date
  - Potential conflicts are resolved deterministically or manually by user
- _Local-first_
  - The application works regardless of network connectivity
  - Every client possesses all the data including history
- _Collaborative_
  - Data of one user can be shared with others for reading as well as for modification
- _Encrypted_
  - All data stored on client devices are encrypted
  - All communication between clients is end-to-end encrypted
- _Multiplatform_
  - Support as many of the most widely used platforms as possible
  - Desktop & mobile
- _Simple & Intuitive_
  - Simple and understandable concepts
  - Clean and intuitive UI
  - [Less is more](https://en.wikipedia.org/wiki/Less_is_more), [worse is better](https://en.wikipedia.org/wiki/Worse_is_better)

## Architecture

This outlines the architecture used for this project, viewed from multiple levels.

### Code

The code adheres the principles of [clean architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html) while maintaining the Java/Spring-like component oriented design for services and their interfaces.
This enables a decent level of complexity, modularity and testability.

## Technology Stack

- Rust language for everything possible
- Dioxus for UI ([https://dioxuslabs.com](https://dioxuslabs.com))
- Iroh for P2P networking ([https://www.iroh.computer](https://www.iroh.computer))
- Automerge ([https://automerge.org/](https://automerge.org/)) for [CFRD structures](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)
- ~~redb ([https://github.com/cberner/redb](https://github.com/cberner/redb))~~ or Turso ([https://github.com/tursodatabase/turso](https://github.com/tursodatabase/turso)) for local data storage
  - redb is robust, but maybe too simple, lacking a lot of features by design
  - Turso is an ambitious project with a ton of features trying to replace SQLite, but it does not feel to be production-ready at this time. But...
    - It is good enough to be used in this project.
    - Supports encryption at rest.
    - Supports indexing and fulltext search by [using Tantivy](https://github.com/tursodatabase/turso/blob/main/docs/fts.md) internally.
- ~~Tantivy ([https://github.com/quickwit-oss/tantivy](https://github.com/quickwit-oss/tantivy)) or SeekStorm ([https://seekstorm.com](https://seekstorm.com))~~ for indexing and search
  - Decided not to use extra technology for indexing and search, since this is already [supported by Turso](https://github.com/tursodatabase/turso/blob/main/docs/manual.md#create-index--define-a-new-index)
## Project Roadmap

This roadmap outlines the planned and completed features for **wilycat** life organizer.

### **Done**
![nothing](https://www.meme-arsenal.com/memes/c087580f123e0dbcd96faf1aec85e9b2.jpg)

### **Selected for Development**
- Channels with plain text notes
- Local storage for channels and notes
- Desktop application for Linux with GUI

### Planned Features ###
In no particular order.

#### Features, Plans & Ideas ####
- Adding clients/devices
- Synchronization of data between clients
- Message tags with search
- Search presets with views
- Fulltext search
- Storage encryption
- Events/reminders
- Callendar view
- Basic text formatting (richtext) and editor
- Sharing with others

#### Platforms ####
- Android app
- iOS app
- Windows desktop app
- Web app

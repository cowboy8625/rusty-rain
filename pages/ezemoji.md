# About [EzEmoji](https://crates.io/crates/ezemoji)

This crate has undergone significant changes throughout its development journey. Initially, all groups of emojis were simply enum variants of a Group enum.

Later on, I revamped the enum variants to structure types to imbue them with trait behavior. While this transformation isn't fully completed yet, my goal is to create a derive macro for it, making it incredibly easy for users to create new ones if ezemoji is missing a group. It's a feature I'm eager to implement.

Additionally, I received suggestions to make the crate not allocate on creation. This has sparked a lot of contemplation on how to achieve this.

As this project continues to evolve, so will this page. Be sure to check back to see how things unfold!

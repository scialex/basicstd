# basicstd
A std frontend using only components without runtime requirements.

The only requirement to use this is to have a working compilation of
the core, alloc, collections, unicode, and rand crates. Of these the
only one which would be at all tricky is the alloc crate, which can
be made by compiling it with the external\_crate feature.

Cargo is not really supported. You should normally build this manually.

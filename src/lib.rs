// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/*!
# ears

__ears__ is a simple library to play sounds and music in Rust, built on top of
OpenAL and libsndfile.

* Provides access to the OpenAL spatialization functionality in a simple way.
* Accepts a lot of audio formats thanks to libsndfile.

# Example

```no_run
extern crate ears;
use ears::{Sound, AudioController};

fn main() {
	// Create a new Sound.
	let mut snd = Sound::new("path/to/my/sound.ogg").unwrap();

	// Play the Sound
	snd.play();

	// Wait until the end of the sound
	while snd.is_playing() {}
}
```

# Functionnality

__ears__ provides two ways to play audio files:

* The `Sound` type, which represents light sounds that can share a buffer of
samples with other `Sounds`.
* The `Music` type, which represents a bigger sound and can't share sample buffers.

# Use ears

As mentioned before, __ears__ requires OpenAL and libsndfile, which you will need
to install on your system in order to build it. See README.md for more details.

Once built, __ears__ can be used like any other Rust crate:

```rust
extern crate ears;

use ears::Music;
// or some other type/module

# fn main() {}
```
*/

#![crate_name = "ears"]
//#![desc = "Easy Api in Rust for Sounds"]
//#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![allow(dead_code, unused_attributes)]
//#![feature(macro_rules)]
//#![feature(unsafe_destructor)]

#![allow(unused_imports)]
//#![allow(raw_pointer_derive)]
#![allow(unused_must_use)]
//#![allow(improper_ctypes)]

extern crate libc;
#[macro_use]
extern crate lazy_static;

// Reexport public API
pub use einit::{init, init_in};
pub use music::Music;
pub use sound::Sound;
pub use states::State;
pub use sound_data::SoundData;
pub use audio_controller::AudioController;
pub use audio_tags::{AudioTags, Tags};
pub use recorder::Recorder;
pub use record_context::RecordContext;


// Hidden internal bindings
mod internal;
mod openal;
mod sndfile;

// The public ears API

#[path = "init.rs"]
mod einit;
pub mod listener;
mod sound;
mod music;
mod sound_data;
mod states;
mod audio_controller;
mod audio_tags;
mod recorder;
mod record_context;

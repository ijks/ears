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

//! Play Sounds easily.

use std::rc::Rc;
use std::cell::RefCell;

use internal::OpenAlData;
use sound_data;//::*;//{SoundData};
use sound_data::{SoundData};
use openal::{ffi, al};
use states::State;
use states::State::{Initial, Playing, Paused, Stopped};
use audio_controller::AudioController;
use audio_tags::{AudioTags, Tags};


/**
 * A shorter sound.
 *
 * Sounds are really light objects, the sound's data is entirely loaded into
 * memory and can be shared between multiple `Sound`s.
 *
 * # Examples
 * ```no_run
 * extern crate ears;
 * use ears::{Sound, AudioController};
 *
 * fn main() -> () {
 *    // Create a Sound with the path of the sound file.
 *    let mut snd = Sound::new("path/to/my/sound.ogg").unwrap();
 *
 *    // Play it
 *    snd.play();
 *
 *    // Wait until the sound stopped playing
 *    while snd.is_playing() {}
 * }
 * ```
 */
pub struct Sound {
    /// The internal OpenAl source identifier
    al_source: u32,
    /// The SoundData associated to the `Sound`.
    sound_data: Rc<RefCell<SoundData>>
}

impl Sound {
    /**
     * Loads sound data from a file and creates a new `Sound` from it.
     *
     * # Argument
     * `path` - The path of the sound file to create the SoundData.
     *
     * # Return
     * An Option with Some(Sound) if the Sound is created properly, or None if
     * un error has occured.
     *
     * # Example
     * ```no_run
     * let snd = match ears::Sound::new("path/to/the/sound.ogg") {
     *     Some(snd) => snd,
     *     None      => panic!("Cannot load the sound from a file !")
     * };
     * ```
     */
    pub fn new(path: &str) -> Option<Sound> {
        check_openal_context!(None);

        let s_data = match SoundData::new(path) {
            Some(s_d) => Rc::new(RefCell::new(s_d)),
            None      => return None
        };

        Sound::new_with_data(s_data)
    }

    /**
     * Creates a new `Sound` using the provided `SoundData`.
     *
     * # Argument
     * `sound_data` - The `SoundData` to associate to the `Sound`.
     *
     * # Return
     * An Option with Some(Sound) if the Sound is created properly, or None if
     * un error has occured.
     *
     * # Example
     * ```no_run
     * use ears::{Sound, SoundData, AudioController};
     * use std::rc::Rc;
     * use std::cell::RefCell;
     *
     * let snd_data = match SoundData::new("path/to/the/sound.ogg") {
     *     Some(snd_data) => Rc::new(RefCell::new(snd_data)),
     *     None           => panic!("Cannot create the sound data !")
     * };
     * let mut snd = match Sound::new_with_data(snd_data) {
     *     Some(mut snd) => snd.play(),
     *     None      => panic!("Cannot create a sound using a sound data !")
     * };
     * ```
     */
    pub fn new_with_data(sound_data: Rc<RefCell<SoundData>>) -> Option<Sound> {
        check_openal_context!(None);

        let mut source_id = 0;
        // create the source
        al::alGenSources(1, &mut source_id);
        // set the buffer
        al::alSourcei(source_id,
                      ffi::AL_BUFFER,
                      sound_data::get_buffer(&*sound_data
                                             .borrow_mut()) as i32);

        // Check if there is OpenAL internal error
        match al::openal_has_error() {
            Some(err) => { println!("{}", err); return None; },
            None => {}
        };

        Some(Sound {
            al_source: source_id,
            sound_data: sound_data
        })
    }

    /**
     * Gets the sound data.
     *
     * # Return
     * The SoundData associated to this `Sound`.
     *
     * # Example
     * ```no_run
     * let snd = ears::Sound::new("path/to/the/sound.ogg").unwrap();
     * let snd_data = snd.get_datas();
     * ```
     */
    pub fn get_datas(&self) -> Rc<RefCell<SoundData>> {
        self.sound_data.clone()
    }

    /**
     * Sets the sound data.
     *
     * Doesn't work if the sound is currently playing.
     *
     * # Argument
     * `sound_data` - The new sound_data
     *
     * # Example
     * ```no_run
     * let snd1 = ears::Sound::new("path/to/the/sound.ogg").unwrap();
     * let mut snd2 = ears::Sound::new("other/path/to/the/sound.ogg").unwrap();
     * let snd_data = snd1.get_datas();
     * snd2.set_datas(snd_data);
     * ```
     */
    pub fn set_datas(&mut self, sound_data: Rc<RefCell<SoundData>>) {
        check_openal_context!(());

        if self.is_playing() {
            return;
        }

        // set the buffer
        al::alSourcei(self.al_source,
                      ffi::AL_BUFFER,
                        sound_data::get_buffer(&*sound_data
                                               .borrow()) as i32);

        self.sound_data = sound_data
    }
}

impl AudioTags for Sound {
    /**
     * Gets the tags of a `Sound` value.
     *
     * # Return
     * A borrowed pointer to the internal struct SoundTags
     */
    fn get_tags(&self) -> Tags {
        (*self.sound_data).borrow().get_tags().clone()
    }
}

impl AudioController for Sound {
    /**
     * Plays or resumes the `Sound`.
     *
     * # Example
     * ```no_run
     * use ears::{Sound, AudioController};
     *
     * let mut snd = Sound::new("path/to/the/sound.ogg").unwrap();
     * snd.play();
     * ```
     */
    fn play(&mut self) -> () {
        check_openal_context!(());

        al::alSourcePlay(self.al_source);

        match al::openal_has_error() {
            None => {},
            Some(err) => println!("{}", err)
        }
    }

     /**
      * Pauses the `Sound`.
      *
      * # Example
      * ```no_run
      * use ears::{Sound, AudioController};
      *
      * let mut snd = Sound::new("path/to/the/sound.ogg").unwrap();
      * snd.play();
      * snd.pause();
      * snd.play(); // the sound restarts at the moment of the pause
      * ```
      */
    fn pause(&mut self) -> () {
        check_openal_context!(());

        al::alSourcePause(self.al_source)
    }

    /**
     * Stops the `Sound`.
     *
     * # Example
     * ```no_run
     * use ears::{Sound, AudioController};
     *
     * let mut snd = Sound::new("path/to/the/sound.ogg").unwrap();
     * snd.play();
     * snd.stop();
     * snd.play(); // the sound restart at the begining
     * ```
     */
    fn stop(&mut self) -> () {
        check_openal_context!(());

        al::alSourceStop(self.al_source)
    }

    /**
     * Checks whether the `Sound` is playing.
     *
     * # Return
     * `true` if the Sound is playing, `false` otherwise.
     *
     * # Example
     * ```no_run
     * use ears::{Sound, AudioController};
     *
     * let mut snd = Sound::new("path/to/the/sound.ogg").unwrap();
     * snd.play();
     * if snd.is_playing() {
     *     println!("Sound is Playing !");
     * } else {
     *     println!("Sound is Pause or Stopped !");
     * }
     * ```
     */
    fn is_playing(&self) -> bool {
        match self.get_state() {
            Playing     => true,
            _           => false
        }
    }

    /**
     * Gets the current state of the `Sound`
     *
     * # Return
     * The state of the sound as a variant of the enum State
     *
     * # Example
     * ```no_run
     * use ears::{Sound, State, AudioController};
     *
     * let snd = Sound::new("path/to/the/sound.ogg").unwrap();
     * match snd.get_state() {
     *     State::Initial => println!("Sound has never been played"),
     *     State::Playing => println!("Sound is playing!"),
     *     State::Paused  => println!("Sound is paused!"),
     *     State::Stopped => println!("Sound is stopped!")
     * }
     * ```
     */
    fn get_state(&self) -> State {
        check_openal_context!(Initial);

        // Gets the source state
        let mut state : i32 = 0;
        al::alGetSourcei(self.al_source, ffi::AL_SOURCE_STATE, &mut state);

        match state {
            ffi::AL_INITIAL => Initial,
            ffi::AL_PLAYING => Playing,
            ffi::AL_PAUSED  => Paused,
            ffi::AL_STOPPED => Stopped,
            _               => unreachable!()
        }

    }

    /**
     * Sets the volume of the `Sound`.
     *
     * A value of 1.0 means unattenuated. Each division by 2 equals an
     * attenuation of about -6dB. Each multiplicaton by 2 equals an
     * amplification of about +6dB.
     *
     * # Argument
     * * `volume` - The volume of the Sound, should be between 0. and 1.
     */
    fn set_volume(&mut self, volume: f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_GAIN, volume);
    }

    /**
     * Gets the volume of the `Sound`.
     *
     * # Return
     * The volume of the Sound between 0. and 1.
     */
    fn get_volume(&self) -> f32 {
        check_openal_context!(0.);

        let mut volume : f32 = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_GAIN, &mut volume);
        volume
    }

    /**
     * Sets the minimal volume for a `Sound`.
     *
     * The minimum volume allowed for a source, after distance and cone
     * attenation is applied (if applicable).
     *
     * # Argument
     * * `min_volume` - The new minimal volume of the Sound should be between
     * 0. and 1.
     */
    fn set_min_volume(&mut self, min_volume: f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_MIN_GAIN, min_volume);
    }

    /**
     * Gets the minimal volume of the `Sound`.
     *
     * # Return
     * The minimal volume of the Sound between 0. and 1.
     */
    fn get_min_volume(&self) -> f32 {
        check_openal_context!(0.);

        let mut volume : f32 = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_MIN_GAIN, &mut volume);
        volume
    }

    /**
     * Sets the maximal volume for a `Sound`.
     *
     * The maximum volume allowed for a sound, after distance and cone
     * attenation is applied (if applicable).
     *
     * # Argument
     * * `max_volume` - The new maximal volume of the Sound should be between
     * 0. and 1.
     */
    fn set_max_volume(&mut self, max_volume: f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_MAX_GAIN, max_volume);
    }

    /**
     * Gets the maximal volume of the `Sound`.
     *
     * # Return
     * The maximal volume of the Sound between 0. and 1.
     */
    fn get_max_volume(&self) -> f32 {
        check_openal_context!(0.);

        let mut volume : f32 = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_MAX_GAIN, &mut volume);
        volume
    }

    /**
     * Sets the Sound looping or not
     *
     * The default looping is false.
     *
     * # Arguments
     * `looping` - The new looping state.
     */
    fn set_looping(&mut self, looping: bool) -> () {
        check_openal_context!(());

        match looping {
            true    => al::alSourcei(self.al_source,
                                     ffi::AL_LOOPING,
                                     ffi::ALC_TRUE as i32),
            false   => al::alSourcei(self.al_source,
                                     ffi::AL_LOOPING,
                                     ffi::ALC_FALSE as i32)
        };
    }

    /**
     * Check if the Sound is looping or not
     *
     * # Return
     * true if the Sound is looping, false otherwise.
     */
    fn is_looping(&self) -> bool {
        check_openal_context!(false);

        let mut boolean = 0;
        al::alGetSourcei(self.al_source, ffi::AL_LOOPING, &mut boolean);

        match boolean as i8 {
            ffi::ALC_TRUE  => true,
            ffi::ALC_FALSE => false,
            _              => unreachable!()
        }
    }

    /**
     * Sets the pitch of the source.
     *
     * A multiplier for the frequency (sample rate) of the source's buffer.
     *
     * Default pitch is 1.0.
     *
     * # Argument
     * * `new_pitch` - The new pitch of the sound in the range [0.5 - 2.0]
     */
    fn set_pitch(&mut self, pitch: f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_PITCH, pitch)
    }

    /**
     * Sets the pitch of the source.
     *
     * # Return
     * The pitch of the sound in the range [0.5 - 2.0]
     */
    fn get_pitch(&self) -> f32 {
        check_openal_context!(0.);

        let mut pitch = 0.;
        al::alGetSourcef(self.al_source, ffi::AL_PITCH, &mut pitch);
        pitch
    }

    /**
     * Sets the position of the sound relative to the listener or absolute.
     *
     * Default position is absolute.
     *
     * # Argument
     * `relative` - True to set sound relative to the listener false to set the
     * sound position absolute.
     */
    fn set_relative(&mut self, relative: bool) -> () {
        check_openal_context!(());

        match relative {
            true    => al::alSourcei(self.al_source,
                                     ffi::AL_SOURCE_RELATIVE,
                                     ffi::ALC_TRUE as i32),
            false   => al::alSourcei(self.al_source,
                                     ffi::AL_SOURCE_RELATIVE,
                                     ffi::ALC_FALSE as i32)
        };
    }

    /**
     * Is the sound relative to the listener or not ?
     *
     * # Return
     * True if the sound is relative to the listener false otherwise
     */
    fn is_relative(&mut self) -> bool {
        check_openal_context!(false);

        let mut boolean = 0;
        al::alGetSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, &mut boolean);

        match boolean as i8 {
            ffi::ALC_TRUE  => true,
            ffi::ALC_FALSE => false,
            _              => unreachable!()
        }
    }

    /**
     * Sets the Sound location in three dimensional space.
     *
     * OpenAL, like OpenGL, uses a right handed coordinate system, where in a
     * frontal default view X (thumb) points right, Y points up (index finger),
     * and Z points towards the viewer/camera (middle finger).
     * To switch from a left handed coordinate system, flip the sign on the Z
     * coordinate.
     *
     * Default position is [0., 0., 0.].
     *
     * # Argument
     * * `position` - A three dimensional vector of f32 containing the position
     * of the listener [x, y, z].
     */
    fn set_position(&mut self, position: [f32; 3]) -> () {
        check_openal_context!(());

        al::alSourcefv(self.al_source, ffi::AL_POSITION, &position[0]);
    }

    /**
     * Gets the position of the Sound in three dimensional space.
     *
     * # Return
     * A three dimensional vector of f32 containing the position of the
     * listener [x, y, z].
     */
    fn get_position(&self) -> [f32; 3] {
        check_openal_context!([0.; 3]);

        let mut position : [f32; 3] = [0.; 3];
        al::alGetSourcefv(self.al_source, ffi::AL_POSITION, &mut position[0]);
        position
    }

    /**
     * Sets the direction of the `Sound`.
     *
     * Specifies the current direction in local space.
     *
     * The default direction is: [0., 0., 0.]
     *
     * # Argument
     * `direction` - The new direction of the `Sound`.
     */
    fn set_direction(&mut self, direction: [f32; 3]) -> () {
        check_openal_context!(());

        al::alSourcefv(self.al_source, ffi::AL_DIRECTION, &direction[0]);
    }

    /**
     * Gets the direction of the `Sound`.
     *
     * # Return
     * The current direction of the `Sound`.
     */
    fn get_direction(&self)  -> [f32; 3] {
        check_openal_context!([0.; 3]);

        let mut direction : [f32; 3] = [0.; 3];
        al::alGetSourcefv(self.al_source, ffi::AL_DIRECTION, &mut direction[0]);
        direction
    }

    /**
     * Sets the maximum distance of the `Sound`.
     *
     * The distance above which the source is not attenuated any further with a
     * clamped distance model, or where attenuation reaches 0.0 gain for linear
     * distance models with a default rolloff factor.
     *
     * The default maximum distance is +inf.
     *
     * # Argument
     * `max_distance` - The new maximum distance in the range [0., +inf]
     */
    fn set_max_distance(&mut self, max_distance: f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_MAX_DISTANCE, max_distance);
    }

    /**
     * Gets the maximum distance of the `Sound`.
     *
     * # Return
     * The maximum distance of the Sound in the range [0., +inf]
     */
    fn get_max_distance(&self) -> f32 {
        check_openal_context!(0.);

        let mut max_distance = 0.;
        al::alGetSourcef(self.al_source,
                         ffi::AL_MAX_DISTANCE,
                         &mut max_distance);
        max_distance
    }

    /**
     * Sets the reference distance of the `Sound`.
     *
     * The distance in units that no attenuation occurs.
     * At 0.0, no distance attenuation ever occurs on non-linear attenuation
     * models.
     *
     * The default distance reference is 1.
     *
     * # Argument
     * * `ref_distance` - The new reference distance of the `Sound`.
     */
    fn set_reference_distance(&mut self, ref_distance: f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_REFERENCE_DISTANCE, ref_distance);
    }

    /**
     * Gets the reference distance of the `Sound`.
     *
     * # Return
     * The current reference distance of the `Sound`.
     */
    fn get_reference_distance(&self) -> f32 {
        check_openal_context!(1.);

        let mut ref_distance = 0.;
        al::alGetSourcef(self.al_source,
                         ffi::AL_REFERENCE_DISTANCE,
                         &mut ref_distance);
        ref_distance
    }

    /**
     * Sets the attenuation of a `Sound`.
     *
     * Multiplier to exaggerate or diminish distance attenuation.
     * At 0.0, no distance attenuation ever occurs.
     *
     * The default attenuation is 1.
     *
     * # Arguments
     * `attenuation` - The new attenuation for the sound in the range [0., 1.].
     */
    fn set_attenuation(&mut self, attenuation: f32) -> () {
        check_openal_context!(());

        al::alSourcef(self.al_source, ffi::AL_ROLLOFF_FACTOR, attenuation);
    }

    /**
     * Gets the attenuation of a `Sound`.
     *
     * # Return
     * The current attenuation for the sound in the range [0., 1.].
     */
    fn get_attenuation(&self) -> f32 {
        check_openal_context!(1.);

        let mut attenuation = 0.;
        al::alGetSourcef(self.al_source,
                         ffi::AL_ROLLOFF_FACTOR,
                         &mut attenuation);
        attenuation
    }

}

//#[unsafe_destructor]
impl Drop for Sound {
    /// Destroy all the resources attached to the `Sound`.
    fn drop(&mut self) -> () {
        unsafe {
            ffi::alDeleteSources(1, &mut self.al_source);
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use sound::Sound;
    use states::State::{Playing, Paused, Stopped};
    use audio_controller::AudioController;

    #[test]
    #[ignore]
    fn sound_create_OK() -> () {
        let snd = Sound::new("res/shot.wav");
        println!("YOUHOU");
        match snd {
            Some(_) => {},
            None    => panic!()
        }
    }

    #[test]
    #[ignore]
    fn sound_create_FAIL() -> () {
        let snd = Sound::new("toto.wav");

        match snd {
            Some(_) => panic!(),
            None    => {}
        }
    }

    #[test]
    #[ignore]
    fn sound_play_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.play();
        assert_eq!(snd.get_state() as i32, Playing as i32);
        snd.stop();
    }

    #[test]
    #[ignore]
    fn sound_pause_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.play();
        snd.pause();
        assert_eq!(snd.get_state() as i32, Paused as i32);
        snd.stop();
    }

    #[test]
    #[ignore]
    fn sound_stop_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.play();
        snd.stop();
        assert_eq!(snd.get_state() as i32, Stopped as i32);
        snd.stop();
    }

    #[test]
    #[ignore]
    fn sound_is_playing_TRUE() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.play();
        assert_eq!(snd.is_playing(), true);
        snd.stop();
    }

    #[test]
    #[ignore]
    fn sound_is_playing_FALSE() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        assert_eq!(snd.is_playing(), false);
        snd.stop();
    }

    #[test]
    #[ignore]
    fn sound_set_volume_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_volume(0.7);
        assert_eq!(snd.get_volume(), 0.7);
    }

    // should fail > 1.
    // #[test]
    // #[should_panic]
    // fn sound_set_volume_high_FAIL() -> () {
    //     let mut snd = Sound::new("shot.wav").expect("Cannot create sound");

    //     snd.set_volume(10.9);
    //     assert_eq!(snd.get_volume(), 10.9);
    // }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_volume_low_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_volume(-1.);
        assert_eq!(snd.get_volume(), -1.);
    }

    #[test]
    #[ignore]
    fn sound_set_min_volume_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_min_volume(0.1);
        assert_eq!(snd.get_min_volume(), 0.1);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_min_volume_high_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_min_volume(10.9);
        assert_eq!(snd.get_min_volume(), 10.9);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_min_volume_low_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_min_volume(-1.);
        assert_eq!(snd.get_min_volume(), -1.);
    }

    #[test]
    #[ignore]
    fn sound_set_max_volume_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_max_volume(0.9);
        assert_eq!(snd.get_max_volume(), 0.9);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_max_volume_high_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_max_volume(10.9);
        assert_eq!(snd.get_max_volume(), 10.9);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_max_volume_low_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_max_volume(-1.);
        assert_eq!(snd.get_max_volume(), -1.);
    }

    #[test]
    #[ignore]
    fn sound_is_looping_TRUE() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_looping(true);
        assert_eq!(snd.is_looping(), true);
    }

    #[test]
    #[ignore]
    fn sound_is_looping_FALSE() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_looping(false);
        assert_eq!(snd.is_looping(), false);
    }

    #[test]
    #[ignore]
    fn sound_set_pitch_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_pitch(1.5);
        assert_eq!(snd.get_pitch(), 1.5);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_pitch_too_low_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_pitch(-1.);
        assert_eq!(snd.get_pitch(), -1.);
    }

    // shoud fail > 2.
    // #[test]
    // #[should_panic]
    // fn sound_set_pitch_too_high_FAIL() -> () {
    //     let mut snd = Sound::new("shot.wav").expect("Cannot create sound");

    //     snd.set_pitch(3.);
    //     assert_eq!(snd.get_pitch(), 3.);
    // }

     #[test]
    #[ignore]
    fn sound_set_relative_TRUE() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_relative(true);
        assert_eq!(snd.is_relative(), true);
    }

    #[test]
    #[ignore]
    fn sound_set_relative_FALSE() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_relative(false);
        assert_eq!(snd.is_relative(), false);
    }

    // untill https://github.com/rust-lang/rust/issues/7622 is not fixed, slice comparsion is used

    #[test]
    #[ignore]
    fn sound_set_position_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_position([50f32, 150f32, 250f32]);
        let res = snd.get_position();
        assert_eq!([res[0], res[1], res[2]], [50f32, 150f32, 250f32]);
    }

    #[test]
    #[ignore]
    fn sound_set_direction_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_direction([50f32, 150f32, 250f32]);
        let res = snd.get_direction();
        assert_eq!([res[0], res[1], res[2]], [50f32, 150f32, 250f32]);
    }


    #[test]
    #[ignore]
    fn sound_set_max_distance_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_max_distance(70.);
        assert_eq!(snd.get_max_distance(), 70.);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_max_distance_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_max_distance(-1.);
        assert_eq!(snd.get_max_distance(), -1.);
    }

    #[test]
    #[ignore]
    fn sound_set_reference_distance_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_reference_distance(70.);
        assert_eq!(snd.get_reference_distance(), 70.);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_reference_distance_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_reference_distance(-1.);
        assert_eq!(snd.get_reference_distance(), -1.);
    }

    #[test]
    #[ignore]
    fn sound_set_attenuation_OK() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_attenuation(0.5f32);
        assert_eq!(snd.get_attenuation(), 0.5f32);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn sound_set_attenuation_FAIL() -> () {
        let mut snd = Sound::new("res/shot.wav").expect("Cannot create sound");

        snd.set_attenuation(-1.);
        assert_eq!(snd.get_attenuation(), -1.);
    }
}

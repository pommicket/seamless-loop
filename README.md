# seamless-loop

Make a WAV file seamlessly loopable.

Normally when you just loop an audio file, even if the end
sounds almost identical to the beginning, you'll get a click when the file loops.
This program fades a bit of the end of the file into the beginning
so that there's no click.

For Windows and Linux executables, see the [releases page](https://github.com/pommicket/seamless-loop/releases).


## Q and A

- How do I run this?

You can run it from the command line,
or on Windows you can drag and drop the WAV file onto the executable.

- What types of audio does this work with?

The end of the audio file needs to be very similar to the beginning
for the loop to sound "nice". This program will work
very well with sine waves, white noise, and most synth-generated sounds.
It will probably not work so well with human voice,
due to its natural variation over time.

- Why are only WAV files supported?

It might reduce quality to re-encode lossy files...
and it's easier to test just one format.

If you need to deal with a different format,
you can use ffmpeg or Audacity to convert the file to wav and back.

- Why not make an Audacity plug-in?

It would be convenient, but LADSPA and LV2 are really not designed for nonlocal effects like this.

Perhaps Nyquist can do it, but I'd rather shoot myself in the arm than use Lisp.

- Why does this program load the whole file into memory when it doesn't need to?

I didn't want to write my own WAV parser... And anyways WAV files can only be up to 4GB and most people
have more memory than that nowadays.

libxm
=====

A small XM (FastTracker II Extended Module) player library. Main
features:

* Small size in mind; many features can be disabled at compile-time,
  or are optimized out by the compiler if not used.

* Timing functions for synchronising against specific instruments,
  samples or channels.

* Samples can be loaded and altered at run-time, making it possible to
  use libxm with softsynths or other real-time signal processors.

Written in C11 and released under the WTFPL license, version 2.

Size
====

The playback routine (assuming the non-portable `.libxm` format is
loaded, see `libxmize` below) fits in under 7KiB of compressed
machine code.

~~~
xzcrush libxmtoau
for x in *.xm; do libxmize $x $x.libxm; done
rename .xm.libxm .libxm *.xm.libxm
ls *.{xm,libxm} | xargs -n 1 -P 16 xz -9ekf
wc -c *.*xm *.*xm.xz libxmtoau.crushed

 6810919 an-dream.libxm
 5585551 an-dream.xm
  109423 cerror-expatarimental.libxm
   49240 cerror-expatarimental.xm
  282590 drozerix_-_crush.libxm
  121346 drozerix_-_crush.xm
  581955 heritage.libxm
  438340 heritage.xm

 2855496 an-dream.libxm.xz
 2876184 an-dream.xm.xz
    4180 cerror-expatarimental.libxm.xz
    4820 cerror-expatarimental.xm.xz
   36060 drozerix_-_crush.libxm.xz
   37276 drozerix_-_crush.xm.xz
  201348 heritage.libxm.xz
  201200 heritage.xm.xz

    6833 libxmtoau.crushed
~~~

Binaries crushed with [`xzcrush`](https://gitlab.com/artefact2/xzcrush).

`libxmize` and the non-portable format
======================================

The `libxmize` binary can convert a standard `.xm` file to a
non-standard, non-portable representation of that module. The file
generated by `libxmize` is usually a lot larger than the original
module, but has better compressibility.

In addition, the non-portable format:

* requires less code to load (`xm_create_context_from_libxmize()`
  instead of `xm_create_context()`) and thus can be used to produce
  smaller binaries. (See the size section above.)

* requires significantly less memory to play back modules (only
  kilobytes thanks to `mmap()`, see `libxmtoau` for an example).

The data generated by `libxmize` will not be readable:

* On a different CPU architecture
* On a different libxm commit
* On a libxm compiled with another compiler
* On a libxm compiled with a different compiler version
* On a libxm compiled with different options or CFLAGS

Examples
========

Some example programs are provided.

* `xmgl` is a music visualiser that uses OpenGL and JACK for very
  precise audio synchronisation. See a demo here:
  <https://www.youtube.com/watch?v=zZkhl6XBUVM> and
  <https://www.youtube.com/watch?v=2zBJ5aeOMeE> or something similar
  that runs in the browser:
  [libxm.js](https://artefact2.github.io/libxm.js/).

* `xmprocdemo`: see [README](./examples/xmprocdemo/README.md)

* `xmtoalsa` is a simple player that uses the ALSA library. It
  produces `xmp`-like output while playing. Use `xmtoalsa --help` (or
  check the source) to see the full usage.

  ~~~
  ./xmtoalsa --random **/*.xm
  ~~~

* `xmtowav` will play a module and output a `.wav` file.

  ~~~
  ./xmtowav my_module.xm my_module.wav
  ~~~

* `xmtoau` will play a module and output a `.au` file to standard
  output.

  ~~~
  mpv <(./xmtoau my_module.xm)
  ~~~

* `libxmtoau` is similar to `xmtoau`, except that it loads
  non-portable files generated by `libxmize`.

* `xmbench` is a benchmark program.

Here are some interesting modules, most showcase unusual or advanced
tracking techniques (and thus are a good indicator of a player's
accuracy):

* [Cerror - Expatarimental](http://modarchive.org/module.php?136603)
* [Lamb - Among the stars](http://modarchive.org/module.php?165819)
* [Raina - Cyberculosis](http://modarchive.org/module.php?165308)
* [Raina - Slumberjack](http://modarchive.org/module.php?148721)
* [Strobe - One for all](http://modarchive.org/module.php?161246)
* [Strobe - Paralysicical death](http://modarchive.org/module.php?65817)

Status
======

Effects
-------

~~~
 Status |##| Eff | Info | Description
--------+--+-----+------+------------------------------
DONE    |00|  0  |      | Arpeggio
DONE    |01|  1  |  (*) | Porta up
DONE    |02|  2  |  (*) | Porta down
DONE    |03|  3  |  (*) | Tone porta
DONE    |04|  4  |  (*) | Vibrato
DONE    |05|  5  |  (*) | Tone porta+Volume slide
DONE    |06|  6  |  (*) | Vibrato+Volume slide
DONE    |07|  7  |  (*) | Tremolo
DONE    |08|  8  |      | Set panning
DONE    |09|  9  |      | Sample offset
DONE    |10|  A  |  (*) | Volume slide
DONE    |11|  B  |      | Position jump
DONE    |12|  C  |      | Set volume
DONE    |13|  D  |      | Pattern break
DONE    |14|  E1 |  (*) | Fine porta up
DONE    |--|  E2 |  (*) | Fine porta down
        |--|  E3 |      | Set gliss control
UNTESTED|--|  E4 |      | Set vibrato control
DONE    |--|  E5 |      | Set finetune
DONE    |--|  E6 |      | Set loop begin/loop
UNTESTED|--|  E7 |      | Set tremolo control
DONE    |--|  E9 |      | Retrig note
DONE    |--|  EA |  (*) | Fine volume slide up
DONE    |--|  EB |  (*) | Fine volume slide down
DONE    |--|  EC |      | Note cut
DONE    |--|  ED |      | Note delay
DONE    |--|  EE |      | Pattern delay
DONE    |15|  F  |      | Set tempo/BPM
DONE    |16|  G  |      | Set global volume
DONE    |17|  H  |  (*) | Global volume slide
DONE    |20|  K  |      | Key off              (Also note number 97)
DONE    |21|  L  |      | Set envelope position
DONE    |25|  P  |  (*) | Panning slide
DONE    |27|  R  |  (*) | Multi retrig note
DONE    |29|  T  |  (*) | Tremor
DONE    |33|  X1 |  (*) | Extra fine porta up
DONE    |--|  X2 |  (*) | Extra fine porta down
~~~

Volume effects
--------------

~~~
 Status |  Value  | Meaning
--------+---------+-----------------------------
DONE    | $10-$50 | Set volume (Value-$10)
DONE    | $60-$6f | Volume slide down
DONE    | $70-$7f | Volume slide up
DONE    | $80-$8f | Fine volume slide down
DONE    | $90-$9f | Fine volume slide up
DONE    | $a0-$af | Set vibrato speed
DONE    | $b0-$bf | Vibrato
DONE    | $c0-$cf | Set panning
DONE    | $d0-$df | Panning slide left
DONE    | $e0-$ef | Panning slide right
DONE    | $f0-$ff | Tone porta
~~~

Known issues
------------

* Only loads FastTracker II-compatible XM files.

* Loading a bogus file (that yet has a valid 60-byte header) will
  probably result in a segmentation fault.

* Big endian architectures are not yet supported.

Tests
=====

Some test XM files are in the `tests` directory. Their goal is to test
a certain feature against regressions. A summary of tests (and what
they are supposed to test) is in the table below.

~~~
     Test                      |     Status     |     Tested against     | Extras
-------------------------------+----------------+------------------------+------------------------------------------------
amiga.xm                       | FAIL           | MilkyTracker, xmp      | Should sound identical.
finetune.xm                    | PASS           | MilkyTracker           | Left and right channels should sound identical.
note-delay-ghost.xm            | PASS           | MilkyTracker, FT2      | Should sound identical.
note-delay-retrig.xm           | PASS           | MilkyTracker           | Should sound identical.
pattern-loop-quirk.xm          | PASS           | MilkyTracker           | Should play the same notes at the same time.
ramping.xm                     | PASS           | MilkyTracker           | If XM_RAMPING is ON, no loud clicks should be heard.
ramping2.xm                    | PASS           | MilkyTracker           | If XM_RAMPING is ON, no loud clicks should be heard.
tone-portamento.xm             | PASS           | MilkyTracker           | Should sound identical.
tremolo.xm                     | PASS           | MilkyTracker           | Should sound identical.
tremor.xm                      | PASS           | MilkyTracker           | Should sound identical.
vibrato.xm                     | PASS           | MilkyTracker           | Should sound identical.
~~~

Thanks
======

Thanks to:

* Thunder <kurttt@sfu.ca>, for writing the `modfil10.txt` file;

* Matti "ccr" Hamalainen <ccr@tnsp.org>, for writing the `xm-form.txt`
  file;

* Mr.H of Triton and Guru and Alfred of Sahara Surfers, for writing
  the specification of XM 1.04 files;

* All the MilkyTracker contributors, for the [thorough
  documentation](http://www.milkytracker.org/docs/MilkyTracker.html#effects)
  of effects.

* All the people that helped on `#milkytracker` IRC.
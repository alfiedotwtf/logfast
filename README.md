# NAME

logfast - Insanely fast logging with a simple interface

## Example

    use logfast::LogFast;

    fn main() {
      let mut lf = LogFast::new("my.log", 100).unwrap();

      ...

      lf.log("Here's a test log line");
      lf.log("And here's another");

      ...
    }


    $ cat my.log
    2020-01-10 23:54:25.177300600: Here's a test log line
    2020-01-10 23:54:25.177380200: And here's another

# DESCRIPTION

When you're doing things at speed, synchronous logging can really get in the
way. Normally, each time you write to your logfile, your main thread will block
until the write happens (this can be SLOW). Not with `LogFast`. Each time you
call `log()`, it:

* Pushes your log message to a buffer in a seperate thread
* Instantly returns to let you keep going 

Meanwhile, in the `LogFast` thread, it will flush the logs to the file as fast
as it can, and then go back to sleep until new log messages arrive. This
effectively parallelises the log writing with your main thread. Neat!

# SUPPORT

Please report any bugs or feature requests at:

* [https://gitlab.com/alfiedotwtf/logfast/issues](https://gitlab.com/alfiedotwtf/logfast/issues)

Feel free to fork the repository and submit pull requests :)

# SEE ALSO

* [Some nice pictures of logs](https://www.flickr.com/search?text=logs)

# AUTHOR

[Alfie John](https://www.alfie.wtf)

# WARRANTY

IT COMES WITHOUT WARRANTY OF ANY KIND.

# COPYRIGHT AND LICENSE

Perpetual Copyright (C) by Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).

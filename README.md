# Screen Time
My optician recommended 20 seconds looking away from screens every 20 mins

This is a collection of scripts aiming towards a configurable app for screen break reminders. The `prototype` branch features a love2d script which just counts up infinitely. The `app` branch contains an as yet unfinished Rust/GTK app which is planned to support configuration like turning off for a set amount of time and avoid prompting when in fullscreen mode. The `notify` branch contains a very basic shell script and notes for use in cronjobs using the `notify-send` command which while also unconfigurable doesn't steal focus like the prototype does.

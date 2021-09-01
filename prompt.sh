/usr/bin/notify-send "Screen Break" "20 seconds" -t 20000
# (Note that Notify OSD ignores the timeout parameter: https://bugs.launchpad.net/ubuntu/+source/notify-osd/+bug/390508)
# One workaround is setting -u critical so the prompt stays up till dismissed
#
# What you need in crontab to run this
# you might also need to set the DBUS_SESSION_BUS_ADDRESS to whatever you see
# from `env | grep DBUS_SESSION_BUS_ADDRESS` on some desktop environments
# the same way DISPLAY is set here (eg DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus)
# 20 * * * * env DISPLAY=:0 /bin/bash /home/pathtofile/prompt.sh
# 40 * * * * env DISPLAY=:0 /bin/bash /home/pathtofile/prompt.sh
# 0 * * * *  env DISPLAY=:0 /bin/bash /home/pathtofile/prompt.sh


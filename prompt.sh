/usr/bin/notify-send "Screen Break" "20 seconds" -t 20000
# What you need in crontab to run this
# 20 * * * * env DISPLAY=:0 /bin/bash /home/pathtofile/prompt.sh
#40 * * * * env DISPLAY=:0 /bin/bash /home/pathtofile/prompt.sh
#0 * * * *  env DISPLAY=:0 /bin/bash /home/pathtofile/prompt.sh


#!/bin/sh

INTERFACE=$(ifconfig | awk -F ':' '/BROADCAST,RUNNING/{ print $1 }')
ZONE="/sys/devices/virtual/thermal/thermal_zone2/temp"
while true; do
    WIFI="$(iw $INTERFACE station dump | awk '/signal:/{ print $2"dBm" }')"
    CPU="$(mpstat 1 1 | awk '/Average/{ printf("%.1f%"), 100 - $12 }')"
    TIME="$(date +"%H:%M:%S")"
    DATE="$(date +"%d/%m/%Y")"
    MEM="$(free | awk '/Mem/{ printf("%.1f%"), (1.0 - $7/$2) * 100.0 }')"
    VOL="$(amixer get Master | awk '/Mono/{ printf $4 }' | tr -d '[]')"
    TEMP="$(awk '{ printf "%.1fC", $1/1000.0 }' < $ZONE)"
    xsetroot -name "cpu $CPU | mem $MEM | net $WIFI | vol $VOL | temp $TEMP | $TIME | $DATE"
done


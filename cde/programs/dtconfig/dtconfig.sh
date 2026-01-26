#!/bin/sh
#
# dtconfig - CDE Configuration Utility (Modernized)
#
# Usage:
#   dtconfig -e      (Enable autostart)
#   dtconfig -d      (Disable autostart)
#   dtconfig -kill   (Kill dtlogin)
#   dtconfig -reset  (Reload configuration)
#   dtconfig -p      (Print printer info - Not Implemented/Legacy)
#   dtconfig -inetd  (Configure inetd - Not Implemented/Legacy)
#

export PATH=/bin:/usr/bin:/sbin:/usr/sbin

USAGE="Usage: dtconfig [-e] [-d] [-kill] [-reset]"

if [ $# -eq 0 ]; then
    echo "$USAGE"
    exit 1
fi

case "$1" in
    -e)
        echo "Enabling dtlogin systemd service..."
        if systemctl enable --now dtlogin.service; then
            echo "Done."
        else
            echo "Failed."
            exit 1
        fi
        ;;
    -d)
        echo "Disabling dtlogin systemd service..."
        if systemctl disable --now dtlogin.service; then
            echo "Done."
        else
            echo "Failed."
            exit 1
        fi
        ;;
    -kill)
        echo "Killing dtlogin..."
        pkill -TERM dtlogin
        ;;
    -reset)
        echo "Resetting dtlogin (reloading config)..."
        pkill -HUP dtlogin
        ;;
    *)
        echo "Unknown option: $1"
        echo "$USAGE"
        exit 1
        ;;
esac

exit 0

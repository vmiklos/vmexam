#!/usr/bin/env bash
#
# Copyright 2024 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

#
# This script publishes one activity from Strava to an unlisted link, visible to unregistered users
# as well. 'plug' is where the Strava backup is invoked, 'wilson' is where the result is published.
#

if [[ "$HOSTNAME" =~ plug ]]; then
    STRAVA_DEST="localhost"
else
    STRAVA_DEST="plug"
fi
BACKUP=$(ssh $STRAVA_DEST 'find .local/share/strava-backup/activities |grep fit$ |sort |tail -n 1')
scp $STRAVA_DEST:$BACKUP $STRAVA_DEST:$(echo $BACKUP |sed 's|.fit|.meta.json|') .
JSON=$(fit2json -- $(basename $BACKUP))
scp -- $JSON wilson:share/pages/map/
echo "<https://share.vmiklos.hu/pages/map/?a=$(basename -- $JSON .json)>"

# vim:set shiftwidth=4 softtabstop=4 expandtab:

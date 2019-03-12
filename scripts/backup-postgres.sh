#!/usr/bin/env bash

stats_db_dir=/home/rob/src/baseball-stats-db
todays_date=`date +"%Y-%m-%d"`

# postgres
docker exec stats-postgres pg_dump -U baseball -W retrosheet_events > $stats_db_dir/retrosheet/backups/retrosheet-events-postgres.sql
tar -cvf $stats_db_dir/retrosheet/backups/retrosheet-events-$todays_date-postgres.tgz $stats_db_dir/retrosheet/backups/retrosheet-events-postgres.sql
if [ -s $stats_db_dir/retrosheet/backups/retrosheet-events-$todays_date-postgres.tgz ]
then
        rm $stats_db_dir/retrosheet/backups/retrosheet-events-postgres.sql
fi


#!/usr/bin/env bash

stats_db_dir=/home/rob/src/baseball-stats-db
sqlite_db_path=$stats_db_dir/retrosheet/retrosheet_events.db

orig_dir=`pwd`
mysql_dump=`which mysqldump`
pg_dump=`which pg_dump`
sqlite_dump="sqlite3 $sqlite_db_path .dump"
gzip=`which tar`

todays_date=`date +"%Y-%m-%d"`

./backup-postgres.sh
./backup-mysql.sh

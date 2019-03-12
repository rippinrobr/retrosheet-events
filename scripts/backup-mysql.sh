#!/usr/bin/env bash

mysql_dump=`which mysqldump`
stats_db_dir=/home/rob/src/baseball-stats-db
todays_date=`date +"%Y-%m-%d"`

# mysql
$mysql_dump -h mkultra -u baseball --password=itsmerob retrosheet_events >$stats_db_dir/retrosheet/backups/retrosheet-events-mysql.sql
tar -cvf $stats_db_dir/retrosheet/backups/retrosheet-events-$todays_date-mysql.tgz $stats_db_dir/retrosheet/backups/retrosheet-events-mysql.sql
if [ -s $stats_db_dir/retrosheet/backups/retrosheet-events-$todays_date-mysql.tgz ]
then
        rm $stats_db_dir/retrosheet/backups/retrosheet-events-mysql.sql
fi


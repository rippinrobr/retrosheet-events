#!/bin/bash

for season in {9999,1963}; do  
   echo "delete from coms where game_id like '%${season}%';"
   echo "delete from data where game_id like '%${season}%';"
   echo "delete from plays where game_id like '%${season}%';"
   echo "delete from starters where game_id like '%${season}%';"
   echo "delete from subs where game_id like '%${season}%';"
   echo "delete from games where game_id like '%${season}%';"
   echo ""
done
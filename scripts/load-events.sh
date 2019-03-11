#!/bin/bash

error_file="./errors.txt"
export RUST_BACKTRACE=full

cd ..
#season=1935
for season in {1933..1921}; do 
    echo "Loading events for $season regular season.."
    #elapsed_time=`time ./target/release/retrosheet-loader regular $season 2>>errors.txt`
    elapsed_time=`time ./target/debug/retrosheet-loader regular $season 2>>errors.txt`
    echo "$elapsed_time"

    if [ -s $error_file ]
    then 
        echo "there are errors for $season"
        cp ${error_file} "${season}_errors.txt"
        cp /dev/null ${error_file}
    fi
    echo ""
done

cd scripts

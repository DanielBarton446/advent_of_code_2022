#!/usr/bin/env sh

cat template | sed "s/NN/$1/g" > src/bin/day_$1.rs;
mkdir inputs/day_$1;
touch inputs/day_$1/{input.txt,sample.txt};

#!/bin/bash
source $HOME/.cargo/env
filename=$1
output=${filename%.rs}.run
rustc ${filename} -o ${output} && cd $(dirname ${output}) && ./$(basename ${output})

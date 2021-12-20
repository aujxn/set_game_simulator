#!/bin/bash

#SBATCH --job-name set_l
#SBATCH --partition long
#SBATCH --cpus-per-task 20
#SBATCH --ntasks 30
#SBATCH --nodes 30
#SBATCH --output output/out_%j.txt 
#SBATCH --error output/set_%j.err 

srun ./target/release/set_simulator run -h 168 -t 20

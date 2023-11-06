#!/bin/bash

HOSTNAME="seff_jr"
IP="192.168.0.5"

# Command to run on Raspberry PI
# nohup is used to stop hanging on the session and prints goes to log.out
COMMAND="nohup bash BenchManagementRaspbPI/logging_manager.sh $1 > log.out"

# Call Raspberry PI with command
# Uses -i ~/.ssh/id_rsa for public key use, rather than password auth
ssh -i $HOME/.ssh/id_rsa $HOSTNAME@$IP $COMMAND &

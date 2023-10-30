#!/bin/bash

HOSTNAME="seff_jr"
IP="192.168.0.5"

COMMAND="bash BenchManagementRaspbPI/logging_manager.sh"

# Call Raspberry PI with command
# Uses -i ~/.ssh/id_rsa for public key use, rather than password auth
ssh -i $HOME/.ssh/id_rsa $HOSTNAME@$IP $COMMAND $1 &

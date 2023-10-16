#!/bin/bash

HOSTNAME="seff_jr"
IP="192.168.0.5"


# Commands for the raspberry pi
if [ "$1" == 1 ]; then
  COMMAND="python kasa_energy_consumption.py"
elif [ "$1" == 0 ]; then
  COMMAND="python temp_socket_testing_manager.py"

# Call Raspberry PI with command
ssh $HOSTNAME@$IP $COMMAND
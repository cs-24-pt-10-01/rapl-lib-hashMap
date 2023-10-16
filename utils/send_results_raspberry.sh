#!/bin/bash

HOSTNAME="seff_jr"
IP="192.168.0.5"

# Send entire results dir to Raspberry Pi
scp /results/* $HOSTNAME@$IP:/home/$HOSTNAME/results/
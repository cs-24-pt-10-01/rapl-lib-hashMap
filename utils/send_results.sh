#!/bin/bash

HOSTNAME="seff_jr"
IP="192.168.0.5"

# Send entire results dir to Raspberry Pi
scp -r /home/seff/rapl-interface/results $HOSTNAME@$IP:/home/$HOSTNAME/

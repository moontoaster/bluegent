#!/usr/bin/env bash

systemctl restart bluetooth
bluetoothctl discoverable-timeout 180

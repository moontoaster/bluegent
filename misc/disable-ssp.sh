#!/usr/bin/env bash

bluetoothctl power off
bluetoothctl mgmt.ssp off
bluetoothctl mgmt.sc off
bluetoothctl power on
bluetoothctl discoverable on
bluetoothctl discoverable-timeout 0
bluetoothctl pairable on

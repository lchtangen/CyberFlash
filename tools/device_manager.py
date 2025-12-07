#!/usr/bin/env python3
import subprocess
import json
import sys
import time

def run_command(command):
    try:
        result = subprocess.run(command, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return None

def get_connected_devices():
    adb_output = run_command("adb devices -l")
    fastboot_output = run_command("fastboot devices")
    
    devices = []
    
    # Parse ADB
    if adb_output:
        lines = adb_output.split('\n')[1:]
        for line in lines:
            if line.strip():
                parts = line.split()
                serial = parts[0]
                state = parts[1]
                model = "Unknown"
                if "model:" in line:
                    model = line.split("model:")[1].split()[0]
                
                devices.append({
                    "serial": serial,
                    "type": "adb",
                    "state": state,
                    "model": model
                })

    # Parse Fastboot
    if fastboot_output:
        lines = fastboot_output.split('\n')
        for line in lines:
            if line.strip():
                parts = line.split()
                serial = parts[0]
                devices.append({
                    "serial": serial,
                    "type": "fastboot",
                    "state": "bootloader",
                    "model": "Fastboot Device"
                })
                
    return devices

def get_battery_level(serial):
    level = run_command(f"adb -s {serial} shell dumpsys battery | grep level")
    if level:
        return int(level.split(":")[1].strip())
    return None

def main():
    action = sys.argv[1] if len(sys.argv) > 1 else "scan"
    
    if action == "scan":
        devices = get_connected_devices()
        # Enrich with battery data if ADB
        for dev in devices:
            if dev['type'] == 'adb' and dev['state'] == 'device':
                dev['battery'] = get_battery_level(dev['serial'])
        
        print(json.dumps(devices, indent=2))
        
    elif action == "monitor":
        print("Starting device monitor...")
        last_state = []
        while True:
            current_state = get_connected_devices()
            if current_state != last_state:
                print(json.dumps(current_state))
                last_state = current_state
            time.sleep(2)

if __name__ == "__main__":
    main()

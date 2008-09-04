#!/usr/bin/env python

import os, sys, glob

def handle_linux2_ppc():
	data = {}
	battery_desc = "Battery"

	pmu_proc = open("/proc/pmu/info")
	lines = pmu_proc.readlines()
	for line in lines:
		token = line.split(':')
		data[token[0].strip()] = token[1].strip()

	for i in glob.glob("/proc/pmu/battery_*"):
		pmu_battery = open(i)
		lines = pmu_battery.readlines()
		for line in lines:
			try:
				token = line.split(':')
				data[token[0].strip()] = token[1].strip()
			except:
				pass

		percent = int(100 * int(data['charge']) / int(data['max_charge']))
		seconds = int(data['time rem.'])
		hours = seconds / 3600
		seconds -= 3600 * hours
		minutes = seconds / 60
		seconds -= 60 * minutes

		if int(data['time rem.']) == 0:
			state = "charged"
		elif data['Battery count'] != "0" and data['AC Power'] == "0":
			state = "discharging"
			poststr = "remaining"
		else:
			state = "charging"
			poststr = "until charged"

		battery_num = int(i.split('_')[-1])+1
		sys.stdout.write("%12s %d: %s, %d%%" % (battery_desc, battery_num, state, percent))

		if state == "charged":
			print
		else:
			print ", %02d:%02d:%02d %s" % (hours, minutes, seconds, poststr)

if __name__ == "__main__":
	if sys.platform == "linux2":
		if os.uname()[-1] == "ppc":
			handle_linux2_ppc()
		else:
			os.system("acpi")

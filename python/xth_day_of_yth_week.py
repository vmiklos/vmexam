#!/usr/bin/env python3

import datetime
import sys

"""
Prints the Xth day on the Yth week of the current month.
For example, to print the 2nd Sat of the current month:
./xth_day_of_yth_week.py 2 5
"""

week_num = int(sys.argv[1])
day_num = int(sys.argv[2])

day = datetime.datetime.now()
# 1st day in the current month
day = day.replace(day=1)
start_of_month = day.weekday()
# get to the requested week
for week in range(week_num - 1):
    day += datetime.timedelta(days=7)
# get to the requested day: 0..6
day += datetime.timedelta(days=day_num - start_of_month)
print(day.strftime("%Y-%m-%d"))

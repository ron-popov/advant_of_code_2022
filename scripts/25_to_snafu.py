#!/usr/bin/python3

import argparse
import math

snafu_to_decimal = {
	'2': 2,
	'1': 1,
	'0': 0,
	'-': -1,
	'=': -2
}

base5_to_snafu = {
	2: '2',
	1: '1',
	0: '0',
	-1: '-',
	-2: '='
}

decimal_to_base5 = {
	'4': 4,
	'3': 3,
	'2': 2,
	'1': 1,
	'0': 0,
}

if __name__ == "__main__":
	parser = argparse.ArgumentParser()
	parser.add_argument("decimal")
	args = parser.parse_args()
	
	decimal = int(args.decimal)

	# Calc base5 value of decimal
	base5 = ""
	while decimal != 0:
		remainder = decimal % 5
		decimal = math.floor(decimal / 5)
		base5 = str(remainder) + base5

	print("[-] Base5 value is {}".format(base5))

	snafu_value = ""
	leftover = 0
	while len(base5) != 0:
		base5_digit = int(base5[-1])
		base5 = base5[:-1]

		base5_digit += leftover
		leftover = 0

		if base5_digit not in base5_to_snafu.keys():
			base5_digit -= 5
			leftover += 1
			print("[d] Subtracting 5, increasing leftover by 1")

		snafu_digit = base5_to_snafu[base5_digit]
		snafu_value = snafu_digit + snafu_value

	if leftover:
		snafu_value = str(leftover) + snafu_value
		leftover = 0

	print("[V] Snafu Value is {}".format(snafu_value))



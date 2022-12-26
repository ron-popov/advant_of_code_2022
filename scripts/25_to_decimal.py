#!/usr/bin/python3

import argparse

snafu_digit_to_decimal = {
	'2': 2,
	'1': 1,
	'0': 0,
	'-': -1,
	'=': -2
}

if __name__ == "__main__":
	parser = argparse.ArgumentParser()
	parser.add_argument("snafu")
	args = parser.parse_args()
	
	snafu = args.snafu
	decimal_value = 0
	digit_value = 1

	while len(snafu) != 0:
		snafu_digit = snafu[-1]
		snafu = snafu[:-1]

		decimal_value += snafu_digit_to_decimal[snafu_digit] * digit_value
		digit_value *= 5

	print(decimal_value)

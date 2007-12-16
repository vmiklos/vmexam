"""
Write a program which inputs a single even number (between 4 and 10,000
inclusive) and outputs a single number which is the number of different ways
the input can be expressed as the sum of two primes.

(http://en.wikipedia.org/wiki/Goldbach's_conjecture)
"""

def isprime(n):
	'''check if integer n is a prime'''
	# make sure n is a positive integer
	n = abs(int(n))
	# 0 and 1 are not primes
	if n < 2:
		return False
	# 2 is the only even prime number
	if n == 2:
		return True
	# all other even numbers are not primes
	if not n & 1:
		return False
	# range starts with 3 and only needs to go up the squareroot of n
	# for all odd numbers
	for x in range(3, int(n**0.5)+1, 2):
		if n % x == 0:
			return False
	return True

input = 22
output = 0

for i in range(1, input+1):
	for j in range(1, i+1):
		if isprime(i) and isprime(j) and i+j == input:
			output += 1
print output

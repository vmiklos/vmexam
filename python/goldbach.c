#include <stdlib.h>
#include <math.h>
#include <stdio.h>

int isprime(int n)
{
	int i;

	// make sure n is a positive integer
	n = abs(n);
	// 0 and 1 are not primes
	if(n < 2)
		return(0);
	// 2 is the only even prime number
	if(n==2)
		return(1);
	// all other even numbers are not primes
	if(n%2==0)
		return(0);
	/* range starts with 3 and only needs to go up the squareroot of n
	 * for all odd numbers */
	for(i=3;i<=pow(n, 0.5)+1; i+=2)
		if(n % i == 0)
			return(0);
	return(1);
}

int main()
{
	int input = 22;
	int output = 0;
	int i, j;

	for(i=1; i<=input;i++)
		for(j=1; j<=i;j++)
			if(isprime(i) && isprime(j) && i+j==input)
				output++;
	printf("%d\n", output);
}

#include <stdio.h>
#include <math.h>
#include <string.h>

int l, n;
float mx[20][20], a[20], b[20], c[20], f[20];

void forwardSubstitution() {
	int i, j, k, max;
	float t;
	for (i = 0; i < n; ++i) {
		max = i;
		for (j = i + 1; j < n; ++j)
			if (mx[j][i] > mx[max][i])
				max = j;
		
		for (j = 0; j < n + 1; ++j) {
			t = mx[max][j];
			mx[max][j] = mx[i][j];
			mx[i][j] = t;
		}
		
		for (j = n; j >= i; --j)
			for (k = i + 1; k < n; ++k)
				mx[k][j] -= mx[k][i]/mx[i][i] * mx[i][j];

	}
}

void reverseElimination() {
	int i, j;
	for (i = n - 1; i >= 0; --i) {
		mx[i][n] = mx[i][n] / mx[i][i];
		mx[i][i] = 1;
		for (j = i - 1; j >= 0; --j) {
			mx[j][n] -= mx[j][i] * mx[i][n];
			mx[j][i] = 0;
		}
	}
}

void gauss() {
	int i, j;

	forwardSubstitution();
	reverseElimination();

	if(isinf(mx[0][n]))
	{
		printf("hiba");
		return;
	}
	
	for (i = 0; i < n; ++i) {
		// hack! (to avoid -0.00000 and use 0.00000)
		if(mx[i][n]<0&&mx[i][n]>-0.00001)
			mx[i][n]=0;
		if(i+1<n)
		printf("%.5f ", mx[i][n]);
		else
		printf("%.5f", mx[i][n]);
	}
}

int main(int argc, char *argv[]) {
	int i, j;

	FILE *fin = fopen("be.txt", "r");
	fscanf(fin, "%d", &l);
	for(i=0;i<l;i++)
	{
		memset(mx, 0, sizeof(mx));
		memset(a, 0, sizeof(a));
		memset(b, 0, sizeof(b));
		memset(c, 0, sizeof(c));
		memset(f, 0, sizeof(f));
		fscanf(fin, "%d", &n);
		for(j=0;j<n-1;j++)
			fscanf(fin, "%f", &a[j]);
		for(j=0;j<n;j++)
			fscanf(fin, "%f", &b[j]);
		for(j=0;j<n-1;j++)
			fscanf(fin, "%f", &c[j]);
		for(j=0;j<n;j++)
			fscanf(fin, "%f", &f[j]);
		for (j=1;j<n;j++)
			mx[j][j-1] = a[j-1];
		for (j=0;j<n;j++)
			mx[j][j] = b[j];
		for (j=0;j<n-1;j++)
			mx[j][j+1] = c[j];
		for (j=0;j<n;j++)
			mx[j][n] = f[j];
		gauss();
		if(i+1==l)
			break;
		printf("\n");
	}
	fclose(fin);
	return 0;
}

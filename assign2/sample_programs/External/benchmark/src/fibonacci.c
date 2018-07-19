#include <stdio.h>

// A very simple fibonacci sequence that we will be using its implementation in our rust file

int fibonacci(int n){
	int f1 = 0;
	int f2 = 1;
	int f3 = f1 + f2;

	if (n == 0){
		return 0;
	}

	for (int i = 1; i < n; i++){
		f3 = f1 + f2;
		f1 = f2;
		f2 = f3;
	}

	return f3;
}


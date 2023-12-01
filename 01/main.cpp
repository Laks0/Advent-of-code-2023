#include <cctype>
#include <cmath>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int res1(const vector<string>& palabras) {
	int res = 0;

	for (string p : palabras) {
		int pri = -1;
		int ult = -1;
		for (char c : p) {
			if (!isdigit(c)) continue;
			ult = int(c) - 48;
			if (pri == -1) pri = ult;
		}
		res += pri * 10 + ult;
	}

	return res;
}

string numeros[9] = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

int res2(const vector<string>& palabras) {
	int res = 0;

	for (string p : palabras) {
		int pri = -1;
		int ult = -1;
		for (int i = 0; i < p.size(); i++) {
			int dig = -1;
			char c = p[i];
			if (isdigit(c)) {
				dig = int(c) - 48;
			}

			// Buscar números
			int encontrado = 0;
			for (int n = 0; n < 9; n++) {
				string dString = numeros[n];
				bool esta = true;
				for (int l = 0; l < dString.size(); l++) 
					esta = esta && dString[l] == p[i + l];
				
				if (esta) {
					encontrado = n+1;
					break;
				}
			}
			if (encontrado != 0) dig = encontrado;
			
			if (dig == -1) continue;

			ult = dig;
			if (pri == -1) pri = ult;
		}
		res += pri * 10 + ult;
	}

	return res;
}

int main() {
	vector<string> palabras;
	string p;
	cin >> p;
	while (p != "fin") {
		palabras.push_back(p);
		cin >> p;
	}

	cout << res2(palabras) << endl;
}

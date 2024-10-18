#include <bits/stdc++.h>
using namespace std;

// Complete the catAndMouse function below.
string catAndMouse(int x, int y, int z) {
int distA = abs(x - z);  // Distance from Cat A to the mouse
int distB = abs(y - z);  // Distance from Cat B to the mouse

if (distA < distB) {
return "Cat A";
} else if (distB < distA) {
return "Cat B";
} else {
return "Mouse C";  // They arrive at the same time
}
}

int main() {
ofstream fout(getenv("OUTPUT_PATH"));

int q;
cin >> q;  // Number of queries
cin.ignore(numeric_limits<streamsize>::max(), '\n');

for (int q_itr = 0; q_itr < q; q_itr++) {
int x, y, z;
cin >> x >> y >> z;

string result = catAndMouse(x, y, z);

fout << result << "\n";
}

fout.close();

return 0;
}

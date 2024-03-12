#include <stdio.h>
#define T(a) int main() { printf(a, #a); }
T("#include <stdio.h>\n#define T(a) int main() { printf(a, #a); }\nT(%s)\n")

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

bool find_pattern(const char *pattern, const char *string) {
  size_t pattern_len = strlen(pattern);
  size_t string_len = strlen(string);

  if (pattern_len > string_len) {
    return false;
  }

  for (size_t offset = 0; offset + pattern_len <= string_len; offset++) {
    if (memcmp(pattern, &string[offset], pattern_len) == 0) {
      return true;
    }
  }

  return false;
}

int main(int argc, char *argv[]) {
  assert(find_pattern("aba", "fkjashdkfjhaababaskjdhfkjash"));
  assert(!find_pattern("aba", "fkjashdkfjhasbabskjdhfkjash"));

  return 0;
}

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

bool is_prefix(const char *prefix, const char *string) {
  size_t prefix_len = strlen(prefix);

  if (strlen(prefix) > strlen(string)) {
    return false;
  }

  return strncmp(prefix, string, strlen(prefix)) == 0;
}

bool is_suffix(const char *suffix, const char *string) {
  size_t suffix_len = strlen(suffix);
  size_t string_len = strlen(string);

  if (suffix_len > string_len) {
    return false;
  }

  size_t offset = string_len - suffix_len;

  return strcmp(suffix, &string[offset]) == 0;
}

size_t find_k(const char *x, const char *y) {
  size_t x_len = strlen(x);
  size_t y_len = strlen(y);

  size_t result = 0;

  size_t min_len = x_len <= y_len ? x_len : y_len;

  for (size_t k = 0; k <= min_len; k++) {
    if (strncmp(x, &y[y_len - k], k) == 0) {
      result = k;
    }
  }

  return result;
}

int main(int argc, char *argv[]) {
  assert(find_k("aba", "aba") == 3);
  assert(find_k("aba", "ababfasdfhasjdfhaskdhfajkshd") == 0);
  assert(find_k("aba", "jhfaskdfjdhaksdjhfkajshdfkjash") == 0);
  assert(find_k("aba", "ab") == 2);

  assert(find_k("aba", "aba") == 3);
  assert(find_k("aba", "asdasaba") == 3);
  assert(find_k("aba", "asdfjhaskdjhfaskdjhfaksjhdhh") == 0);
  assert(find_k("aba", "ba") == 1);

  return 0;
}

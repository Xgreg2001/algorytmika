#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

// string.startswith(prefix)
bool is_prefix(const char *prefix, const char *string) {
  size_t prefix_len = strlen(prefix);

  if (strlen(prefix) > strlen(string)) {
    return false;
  }

  return strncmp(prefix, string, strlen(prefix)) == 0;
}

// string.endswith(suffix)
bool is_suffix(const char *suffix, const char *string) {
  size_t suffix_len = strlen(suffix);
  size_t string_len = strlen(string);

  if (suffix_len > string_len) {
    return false;
  }

  size_t offset = string_len - suffix_len;

  return strcmp(suffix, &string[offset]) == 0;
}

int main(int argc, char *argv[]) {
  assert(is_prefix("aba", "aba"));
  assert(is_prefix("aba", "ababfasdfhasjdfhaskdhfajkshd"));
  assert(!is_prefix("aba", "jhfaskdfjdhaksdjhfkajshdfkjash"));
  assert(!is_prefix("aba", "ab"));

  assert(is_suffix("aba", "aba"));
  assert(is_suffix("aba", "asdasaba"));
  assert(!is_suffix("aba", "asdfjhaskdjhfaskdjhfaksjhdhh"));
  assert(!is_suffix("aba", "ba"));

  return 0;
}

#include <memory>
#include <split.h>
#include <stdlib.h>
#include <string.h>
#include <string>
#include <vector>

__attribute__((weak, export_name("canonical_abi_realloc"))) void *
canonical_abi_realloc(void *ptr, size_t orig_size, size_t org_align,
                      size_t new_size) {
  void *ret = realloc(ptr, new_size);
  if (!ret)
    abort();
  return ret;
}

__attribute__((weak, export_name("canonical_abi_free"))) void
canonical_abi_free(void *ptr, size_t size, size_t align) {
  free(ptr);
}
#include <string.h>

void split_string_set(split_string_t *ret, const char *s) {
  ret->ptr = (char *)s;
  ret->len = strlen(s);
}

void split_string_dup(split_string_t *ret, const char *s) {
  ret->len = strlen(s);
  ret->ptr =
      reinterpret_cast<char *>(canonical_abi_realloc(NULL, 0, 1, ret->len));
  memcpy(ret->ptr, s, ret->len);
}

void split_string_free(split_string_t *ret) {
  canonical_abi_free(ret->ptr, ret->len, 1);
  ret->ptr = NULL;
  ret->len = 0;
}
void split_subphrase_free(split_subphrase_t *ptr) {
  split_string_free(&ptr->str);
}
void split_list_subphrase_free(split_list_subphrase_t *ptr) {
  for (size_t i = 0; i < ptr->len; i++) {
    split_subphrase_free(&ptr->ptr[i]);
  }
  canonical_abi_free(ptr->ptr, ptr->len * 12, 4);
}

__attribute__((aligned(4))) static uint8_t RET_AREA[8];
__attribute__((export_name("split-str"))) int32_t
__wasm_export_split_split_str(int32_t arg, int32_t arg0, int32_t arg1,
                              int32_t arg2) {
  split_string_t arg3 = (split_string_t){(char *)(arg), (size_t)(arg0)};
  split_string_t arg4 = (split_string_t){(char *)(arg1), (size_t)(arg2)};
  split_list_subphrase_t ret;
  split_split_str(&arg3, &arg4, &ret);
  int32_t ptr = (int32_t)&RET_AREA;
  *((int32_t *)(ptr + 4)) = (int32_t)(ret).len;
  *((int32_t *)(ptr + 0)) = (int32_t)(ret).ptr;
  return ptr;
}

void split_split_str(split_string_t *phrase, split_string_t *delim,
                     split_list_subphrase_t *ret0) {
  // Clear the result.
  memset(ret0, 0, sizeof(split_list_subphrase_t));

  // Parse the tokens.
  std::string phr(phrase->ptr, phrase->len);
  std::string dlm(delim->ptr, delim->len);
  std::string tok;
  std::vector<std::pair<std::string, size_t>> subs;
  size_t start = 0, end = 0;
  if (delim->len) {
    while ((end = phr.find(dlm, start)) != std::string::npos) {
      tok = phr.substr(start, end - start);
      subs.push_back(std::pair<std::string, size_t>(tok, start));
      start = end + dlm.length();
    }
  }
  subs.push_back(std::pair<std::string, size_t>(phr.substr(start), start));

  // Populate the result.
  bool err = false;
  auto res =
      (split_subphrase_t *)malloc(phr.size() * sizeof(split_subphrase_t));
  for (int i = 0; !err && i < subs.size(); ++i) {
    auto &sub = subs[i].first;
    res[i].idx = static_cast<int32_t>(subs[i].second);
    res[i].str.len = sub.length();
    res[i].str.ptr = strdup(sub.c_str());
    if (!res[i].str.ptr)
      err = true;
  }

  // If success, assign the result.  Else, clean up and return an empty list.
  if (!err) {
    // Success; assign the result.
    ret0->ptr = res;
    ret0->len = subs.size();
  } else {
    if (res) {
      for (int i = 0; i < subs.size(); ++i)
        if (res[i].str.ptr)
          free(res[i].str.ptr);
      free(res);
    }
  }

  // Per the Canonical ABI contract, free the input pointers.
  free(phrase->ptr);
  free(delim->ptr);
}

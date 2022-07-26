#include <memory>
#include <split.h>
#include <stdlib.h>
#include <string.h>
#include <string>
#include <vector>

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
      (split_subphrase_t *)malloc(subs.size() * sizeof(split_subphrase_t));
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

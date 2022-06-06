#ifndef __BINDINGS_SPLIT_H
#define __BINDINGS_SPLIT_H
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  
  typedef struct {
    char *ptr;
    size_t len;
  } split_string_t;
  
  void split_string_set(split_string_t *ret, const char *s);
  void split_string_dup(split_string_t *ret, const char *s);
  void split_string_free(split_string_t *ret);
  typedef struct {
    split_string_t str;
    int32_t idx;
  } split_subphrase_t;
  void split_subphrase_free(split_subphrase_t *ptr);
  typedef struct {
    split_subphrase_t *ptr;
    size_t len;
  } split_list_subphrase_t;
  void split_list_subphrase_free(split_list_subphrase_t *ptr);
  void split_split_str(split_string_t *phrase, split_string_t *delim, split_list_subphrase_t *ret0);
  #ifdef __cplusplus
}
#endif
#endif

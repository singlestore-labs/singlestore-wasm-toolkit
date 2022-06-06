#include <stdlib.h>
#include <power.h>

__attribute__((weak, export_name("canonical_abi_realloc")))
void *canonical_abi_realloc(
void *ptr,
size_t orig_size,
size_t org_align,
size_t new_size
) {
  void *ret = realloc(ptr, new_size);
  if (!ret)
  abort();
  return ret;
}

__attribute__((weak, export_name("canonical_abi_free")))
void canonical_abi_free(
void *ptr,
size_t size,
size_t align
) {
  free(ptr);
}
__attribute__((export_name("power-of")))
int32_t __wasm_export_power_power_of(int32_t arg, int32_t arg0) {
  int32_t ret = power_power_of(arg, arg0);
  return ret;
}

int32_t power_power_of(int32_t base, int32_t exp)
{
    int32_t res = 1;
    for (int32_t i = 0; i < exp; ++i)
    {
        res *= base;
    }
    return res;
}


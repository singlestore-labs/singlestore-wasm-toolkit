#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <math.h>
#include "extension.h"

#define ABORT_IF(x_) { if (x_) abort(); }

void
extension_vector_pow_f64(
    extension_list_u8_t *packed,
    double n,
    extension_list_u8_t *ret)
{
    size_t i;
    uint8_t *in, *out;
    double v;

    // Input buffer should be divisible by sizeof(double).
    ABORT_IF(packed->len % sizeof(double) != 0);

    // Allocate space in the result.  Should be the same as the input.
    ret->len = packed->len;
    if (packed->len == 0)
    {
        ret->ptr = NULL;
        return;
    }

    ret->ptr = (uint8_t*) malloc(packed->len);
    ABORT_IF(!ret->ptr);

    // Walk the input and compute the corresponding value in the output.
    in = packed->ptr;
    out = ret->ptr;
    for (i = 0; i < packed->len; i += sizeof(double))
    {
        v = *(double*) (in + i);
        *(double*) (out + i) = pow(v, n);
    }

cleanup:
    // Finally, free the input pointer, per Canonical ABI rules.
    free(packed->ptr);
}

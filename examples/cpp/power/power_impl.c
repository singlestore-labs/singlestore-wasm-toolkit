#include <stdlib.h>
#include <power.h>

int32_t power_power_of(int32_t base, int32_t exp)
{
    int32_t res = 1;
    for (int32_t i = 0; i < exp; ++i)
    {
        res *= base;
    }
    return res;
}

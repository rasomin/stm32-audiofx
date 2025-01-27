#include "hw.h"


void hwInit(void)
{
    bspInit();

    ledInit();
    buttonInit();
    uartInit();
}
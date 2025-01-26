#include "app.h"

int appMain(void)
{
    hwInit();
    apInit();

    apMain();

    return 0;
}
#include "ap.h"


void apInit(void)
{
    uartOpen(_DEF_UART2, 115200);
}

void apMain(void)
{
    uint32_t pre_time;

    pre_time = millis();

    while (1)
    {
        if (millis() - pre_time >= 1000)
        {
            pre_time = millis();
            ledToggle(_DEF_LED1);

            if (uartAvailable(_DEF_UART2) > 0)
            {
                uartPrintf(_DEF_UART2, "UART2 : %d\n", millis());
            }
        }

    }
}
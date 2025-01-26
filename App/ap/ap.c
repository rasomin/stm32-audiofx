#include "ap.h"


void apInit(void)
{

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
        }

        if (buttonGetPressed(_DEF_BUTTON) == true)
        {
            ledOn(_DEF_LED1);
        }
        else
        {
            ledOff(_DEF_LED1);
        }
    }
}
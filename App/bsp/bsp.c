#include "bsp.h"


void bspInit(void)
{
    __HAL_RCC_GPIOC_CLK_ENABLE();
}



void delay(uint32_t ms)
{
    HAL_Delay(ms);
}

uint32_t millis(void)
{
    return HAL_GetTick();
}
#include "button.h"

typedef struct
{
    GPIO_TypeDef *port;
    uint16_t pin;
    GPIO_PinState pressed;
} button_tbl_t;


button_tbl_t button_tbl[BUTTON_MAX_CH] =
{
    {GPIOC, GPIO_PIN_13, GPIO_PIN_SET},
};

bool buttonInit(void)
{
    bool ret = true;

    GPIO_InitTypeDef GPIO_InitStruct = {0};

    GPIO_InitStruct.Mode = GPIO_MODE_INPUT;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;

    for (int i = 0; i < BUTTON_MAX_CH; i++)
    {
        GPIO_InitStruct.Pin = button_tbl[i].pin;
        HAL_GPIO_Init(button_tbl[i].port, &GPIO_InitStruct);
    }

    return ret;
}

bool buttonGetPressed(uint8_t ch)
{
    if (ch >= BUTTON_MAX_CH)
    {
        return false;
    }

    if (HAL_GPIO_ReadPin(button_tbl[ch].port, button_tbl[ch].pin) == button_tbl[ch].pressed)
    {
        return true;
    }
    else
    {
        return false;
    }
}
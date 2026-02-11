#include <stdint.h>
#define GPIO_REG (*(volatile uint32_t*)0x40000000)

int main(void)
{
    GPIO_REG = 1;
    while (1)
        GPIO_REG ^= 1;
}

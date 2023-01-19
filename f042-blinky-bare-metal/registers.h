// Copyright (c) 2023 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// RM0091 8.4 GPIO registers
typedef struct
{
    volatile uint32_t MODER, OTYPER, OSPEEDR, PUPDR, IDR, ODR, BSRR, LCKR, AFR[2], BRR;
} GPIO_TypeDef;

// RM0091 6.4 RCC registers
typedef struct
{
    volatile uint32_t CR, CFGR, CIR, APB2RSTR, APB1RSTR, AHBENR, APB2ENR, APB1ENR, BDCR,
        CSR, AHBRSTR, CFGR2, CFGR3, CR2;
} RCC_TypeDef;

// RM0091 2.2.2 Memory map and register boundary addresses
#define GPIOA ((GPIO_TypeDef *)0x48000000UL)
#define GPIOB ((GPIO_TypeDef *)0x48000400UL)
#define RCC ((RCC_TypeDef *)0x40021000UL)

// RM0091 6.4.6 AHB peripheral clock enable register (RCC_AHBENR)
#define RCC_AHBENR_GPIOAEN (0x1UL << 17U)
#define RCC_AHBENR_GPIOBEN (0x1UL << 18U)
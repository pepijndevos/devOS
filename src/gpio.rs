use uart;
use core::fmt::Write;
use core::ptr::write_volatile;

#[repr(u8)]
#[allow(non_camel_case_types)]
enum c_void {
    #[doc(hidden)]
    __hidden,
}

unsafe fn iowrite32(value: u32, addr: *mut c_void) {
    write_volatile(addr as *mut u32, value);
}

const DA8XX_SYSCFG0_BASE: *mut c_void = (0x01c00000 + 0x14000) as (*mut c_void);
const DA8XX_GPIO_BASE: *mut c_void = 0x01e26000 as (*mut c_void);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct1 {
    pub pin: i32,
    pub mux_reg: u16,
    pub mask: u32,
    pub mode: u32,
}

#[derive(Clone, Copy)]
#[repr(i32)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub enum Enum2 {
    GP0_0,
    GP0_1,
    GP0_2,
    GP0_3,
    GP0_4,
    GP0_5,
    GP0_6,
    GP0_7,
    GP0_8,
    GP0_9,
    GP0_10,
    GP0_11,
    GP0_12,
    GP0_13,
    GP0_14,
    GP0_15,
    GP1_0,
    GP1_1,
    GP1_2,
    GP1_3,
    GP1_4,
    GP1_5,
    GP1_6,
    GP1_7,
    GP1_8,
    GP1_9,
    GP1_10,
    GP1_11,
    GP1_12,
    GP1_13,
    GP1_14,
    GP1_15,
    GP2_0,
    GP2_1,
    GP2_2,
    GP2_3,
    GP2_4,
    GP2_5,
    GP2_6,
    GP2_7,
    GP2_8,
    GP2_9,
    GP2_10,
    GP2_11,
    GP2_12,
    GP2_13,
    GP2_14,
    GP2_15,
    GP3_0,
    GP3_1,
    GP3_2,
    GP3_3,
    GP3_4,
    GP3_5,
    GP3_6,
    GP3_7,
    GP3_8,
    GP3_9,
    GP3_10,
    GP3_11,
    GP3_12,
    GP3_13,
    GP3_14,
    GP3_15,
    GP4_0,
    GP4_1,
    GP4_2,
    GP4_3,
    GP4_4,
    GP4_5,
    GP4_6,
    GP4_7,
    GP4_8,
    GP4_9,
    GP4_10,
    GP4_11,
    GP4_12,
    GP4_13,
    GP4_14,
    GP4_15,
    GP5_0,
    GP5_1,
    GP5_2,
    GP5_3,
    GP5_4,
    GP5_5,
    GP5_6,
    GP5_7,
    GP5_8,
    GP5_9,
    GP5_10,
    GP5_11,
    GP5_12,
    GP5_13,
    GP5_14,
    GP5_15,
    GP6_0,
    GP6_1,
    GP6_2,
    GP6_3,
    GP6_4,
    GP6_5,
    GP6_6,
    GP6_7,
    GP6_8,
    GP6_9,
    GP6_10,
    GP6_11,
    GP6_12,
    GP6_13,
    GP6_14,
    GP6_15,
    GP7_0,
    GP7_1,
    GP7_2,
    GP7_3,
    GP7_4,
    GP7_5,
    GP7_6,
    GP7_7,
    GP7_8,
    GP7_9,
    GP7_10,
    GP7_11,
    GP7_12,
    GP7_13,
    GP7_14,
    GP7_15,
    GP8_0,
    GP8_1,
    GP8_2,
    GP8_3,
    GP8_4,
    GP8_5,
    GP8_6,
    GP8_7,
    GP8_8,
    GP8_9,
    GP8_10,
    GP8_11,
    GP8_12,
    GP8_13,
    GP8_14,
    GP8_15,
    NO_OF_GPIOS,
    UART0_TXD,
    UART0_RXD,
    UART1_TXD,
    UART1_RXD,
    SPI0_MOSI,
    SPI0_MISO,
    SPI0_SCL,
    SPI0_CS,
    SPI1_MOSI,
    SPI1_MISO,
    SPI1_SCL,
    SPI1_CS,
    EPWM1A,
    EPWM1B,
    APWM0,
    APWM1,
    EPWM0B,
    AXR3,
    AXR4,
}

pub static mut mux_reg_map: [Struct1; 129] = [Struct1 {
                                                pin: Enum2::GP0_1 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_2 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_3 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_4 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_5 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_6 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_7 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_11 as (i32),
                                                mux_reg: 0i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_12 as (i32),
                                                mux_reg: 0i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_13 as (i32),
                                                mux_reg: 0i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_14 as (i32),
                                                mux_reg: 0i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP0_15 as (i32),
                                                mux_reg: 0i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_0 as (i32),
                                                mux_reg: 4i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_8 as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x4i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_9 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x4000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_10 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x400000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_11 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x40000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_12 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x4000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_13 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x400i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_14 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x40i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP1_15 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_0 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_1 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_2 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_3 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_4 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_5 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_6 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_7 as (i32),
                                                mux_reg: 6i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_8 as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_9 as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_10 as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_11 as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_12 as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP2_13 as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_0 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_1 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_2 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_3 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_4 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_5 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_6 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_7 as (i32),
                                                mux_reg: 8i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_8 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_9 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_10 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_11 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_12 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_13 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_14 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP3_15 as (i32),
                                                mux_reg: 7i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP4_1 as (i32),
                                                mux_reg: 10i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP4_8 as (i32),
                                                mux_reg: 9i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP4_9 as (i32),
                                                mux_reg: 9i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP4_10 as (i32),
                                                mux_reg: 9i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP4_12 as (i32),
                                                mux_reg: 9i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP4_14 as (i32),
                                                mux_reg: 9i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_0 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_1 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_2 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_3 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_4 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_5 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_6 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_7 as (i32),
                                                mux_reg: 12i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_8 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_9 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_10 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_11 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_12 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_13 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_14 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP5_15 as (i32),
                                                mux_reg: 11i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_0 as (i32),
                                                mux_reg: 19i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_1 as (i32),
                                                mux_reg: 19i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_2 as (i32),
                                                mux_reg: 19i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_3 as (i32),
                                                mux_reg: 19i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_4 as (i32),
                                                mux_reg: 19i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_5 as (i32),
                                                mux_reg: 16i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_6 as (i32),
                                                mux_reg: 14i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_7 as (i32),
                                                mux_reg: 14i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_8 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_9 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_10 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_11 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_12 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_13 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_14 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP6_15 as (i32),
                                                mux_reg: 13i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_4 as (i32),
                                                mux_reg: 17i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_8 as (i32),
                                                mux_reg: 17i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_9 as (i32),
                                                mux_reg: 17i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_10 as (i32),
                                                mux_reg: 16i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_11 as (i32),
                                                mux_reg: 16i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_12 as (i32),
                                                mux_reg: 16i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_13 as (i32),
                                                mux_reg: 16i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_14 as (i32),
                                                mux_reg: 16i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP7_15 as (i32),
                                                mux_reg: 16i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_2 as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x4000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_3 as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x400000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_5 as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x4000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_6 as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x400i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_8 as (i32),
                                                mux_reg: 19i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x80i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_9 as (i32),
                                                mux_reg: 19i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x8i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_10 as (i32),
                                                mux_reg: 18i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x80000000u32,
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_11 as (i32),
                                                mux_reg: 18i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x8000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_12 as (i32),
                                                mux_reg: 18i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x800000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_13 as (i32),
                                                mux_reg: 18i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x80000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_14 as (i32),
                                                mux_reg: 18i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::GP8_15 as (i32),
                                                mux_reg: 18i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x800i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::UART0_TXD as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x200000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::UART0_RXD as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x20000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::UART1_TXD as (i32),
                                                mux_reg: 4i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x20000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::UART1_RXD as (i32),
                                                mux_reg: 4i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x2000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI0_MOSI as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x1000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI0_MISO as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x100i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI0_SCL as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x1i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI0_CS as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xf0ffffffu32,
                                                mode: 0x1000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI1_MOSI as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xff0fffffu32,
                                                mode: 0x100000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI1_MISO as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x10000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI1_SCL as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xfffff0ffu32,
                                                mode: 0x100i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::SPI1_CS as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x8000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::EPWM1A as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xfffffff0u32,
                                                mode: 0x2i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::EPWM1B as (i32),
                                                mux_reg: 5i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x20i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::APWM0 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x20000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::APWM1 as (i32),
                                                mux_reg: 1i32 as (u16),
                                                mask: 0xfffffffi32 as (u32),
                                                mode: 0x40000000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::EPWM0B as (i32),
                                                mux_reg: 3i32 as (u16),
                                                mask: 0xffffff0fu32,
                                                mode: 0x20i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::AXR3 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xfff0ffffu32,
                                                mode: 0x10000i32 as (u32),
                                            },
                                            Struct1 {
                                                pin: Enum2::AXR4 as (i32),
                                                mux_reg: 2i32 as (u16),
                                                mask: 0xffff0fffu32,
                                                mode: 0x1000i32 as (u32),
                                            }];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct gpio_controller {
    pub dir: u32,
    pub out_data: u32,
    pub set_data: u32,
    pub clr_data: u32,
    pub in_data: u32,
    pub set_rising: u32,
    pub clr_rising: u32,
    pub set_falling: u32,
    pub clr_falling: u32,
    pub intstat: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Struct3 {
    pub pin: i32,
    pub p_gpio: *mut gpio_controller,
    pub mask: u32,
}

pub static mut ui_led_pin
    : [Struct3; 4]
    = [   Struct3 {
              pin: Enum2::GP6_12 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP6_14 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP6_13 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP6_7 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          }
      ];

pub static mut ui_but_pin
    : [Struct3; 6]
    = [   Struct3 {
              pin: Enum2::GP7_15 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP1_13 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP7_14 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP7_12 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP6_6 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          },
          Struct3 {
              pin: Enum2::GP6_10 as (i32),
              p_gpio: 0i32 as (*mut gpio_controller),
              mask: 0i32 as (u32)
          }
      ];

pub unsafe fn set_gpio(pin: i32) {
    write!(uart::UART0,
           "Setting GPIO {}\r\n",
           pin).unwrap();
    let mut tmp: i32 = 0i32;
    let reg: *mut c_void;
    if pin >= 0i32 {
        'loop1: loop {
            if mux_reg_map[tmp as (usize)].pin != -1i32 && (mux_reg_map[tmp as (usize)].pin != pin) {
                tmp = tmp + 1;
                continue 'loop1;
            } else {
                break 'loop1;
            }
        }
        if mux_reg_map[tmp as (usize)].pin == pin {
            reg = DA8XX_SYSCFG0_BASE.offset(0x120i32 as (isize))
                .offset((mux_reg_map[tmp as (usize)].mux_reg as (i32) << 2i32) as (isize));
            {
                let _rhs = mux_reg_map[tmp as (usize)].mask;
                let _lhs = &mut *(reg as (*mut u32));
                *_lhs = *_lhs & _rhs;
            }
            {
                let _rhs = mux_reg_map[tmp as (usize)].mode;
                let _lhs = &mut *(reg as (*mut u32));
                *_lhs = *_lhs | _rhs;
            }
        } else {
            write!(uart::UART0,
                   "*   GP{}_{}  ********* ERROR not found *********\n\r",
                   pin >> 4i32,
                   pin & 0xfi32).unwrap();
        }
    }
}

pub unsafe fn init_gpio() {
    let mut pin: i32;
    iowrite32(0x83e70b13u32, DA8XX_SYSCFG0_BASE.offset(0x38i32 as (isize)));
    iowrite32(0x95a4f1e0u32, DA8XX_SYSCFG0_BASE.offset(0x3ci32 as (isize)));
    pin = 0i32;
    'loop3: loop {
        if pin < 4i32 {
            if ui_led_pin[pin as (usize)].pin >= 0i32 {
                ui_led_pin[pin as (usize)].p_gpio =
                    DA8XX_GPIO_BASE.offset(((ui_led_pin[pin as (usize)].pin >> 5i32) * 0x28i32) as (isize))
                        .offset(0x10i32 as (isize)) as (*mut gpio_controller);
                ui_led_pin[pin as (usize)].mask =
                    (1i32 << (ui_led_pin[pin as (usize)].pin & 0x1fi32)) as (u32);
                set_gpio(ui_led_pin[pin as (usize)].pin);
            }
            pin = pin + 1;
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    pin = 0i32;
    'loop7: loop {
        if pin < 6i32 {
            if ui_but_pin[pin as (usize)].pin >= 0i32 {
                ui_but_pin[pin as (usize)].p_gpio =
                    DA8XX_GPIO_BASE.offset(((ui_but_pin[pin as (usize)].pin >> 5i32) * 0x28i32) as (isize))
                        .offset(0x10i32 as (isize)) as (*mut gpio_controller);
                ui_but_pin[pin as (usize)].mask =
                    (1i32 << (ui_but_pin[pin as (usize)].pin & 0x1fi32)) as (u32);
                set_gpio(ui_but_pin[pin as (usize)].pin);
            }
            pin = pin + 1;
            continue 'loop7;
        } else {
            break 'loop7;
        }
    }
    iowrite32(0x0i32 as (u32),
              DA8XX_SYSCFG0_BASE.offset(0x38i32 as (isize)));
    iowrite32(0x0i32 as (u32),
              DA8XX_SYSCFG0_BASE.offset(0x3ci32 as (isize)));
}

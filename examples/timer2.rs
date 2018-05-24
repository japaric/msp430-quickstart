// vim: set ts=4 sts=4 sw=4 expandtab:
#![no_std]
#![feature(abi_msp430_interrupt)]
/*
    Set timer to count up to 5000 
    When counter reach 1000 CCR1 generate interrupt and handler switch port p1.0 (LED1).
    When counter reach 4000 CCR2 generate interrupt and handler switch port p1.6 (LED2).
*/

extern crate msp430;

// Some explanations about msp430g2553's API: https://docs.rs/svd2rust/0.11.2/svd2rust/
#[macro_use(interrupt)]
extern crate msp430g2553;

use msp430::interrupt;
use msp430g2553::{PORT_1_2, TIMER0_A3, SYSTEM_CLOCK};

fn main() {
    interrupt::free(|cs| {
        // Disable watchdog
        let wdt = msp430g2553::WATCHDOG_TIMER.borrow(&cs);
        wdt.wdtctl.write(|w| {
            unsafe { w.bits(0x5A00) } // password
            .wdthold().set_bit()
        });


        // p1.0 LED1
        // p1.6 LED2
        let port_1_2 = PORT_1_2.borrow(cs);
        // dir - direction out?
        port_1_2.p1dir.modify(|_, w| w.p0().set_bit()
                                      .p6().set_bit());
        // p0 = 1, p6 = 0
        port_1_2.p1out.modify(|_, w| w.p0().set_bit()
                                      .p6().clear_bit());

        /* ********* CLOCK **********

        */
        let clock = SYSTEM_CLOCK.borrow(cs);
        // "0x01 - Basic Clock System Control 3"
        clock.bcsctl3.modify(|_, w|
            // _ - byte read from register (struct bcsctl3::R)
            // w - byte write to register (struct bcsctl3::W)
            w
                // lfxt -  Low/high Frequency Oscillator
                // Bits 4:5 - Mode 0 for LFXT1 (XTS = 0) -> _LFXT1SW
                .lfxt1s()
                // Mode 2 for LFXT1 : VLO
                .lfxt1s_2()
        );
        // "0x05 - Basic Clock System Control 1"
        clock.bcsctl1.modify(|_, w|
            // bcsctl1::W
            w
                // Bits 4:5 - ACLK Divider 0 -> _DIVAW
                .diva()
                // ACLK Divider 1: /2 -> bcsctl1::W
                .diva_1()
        );

        /* ********* TIMER ***********
            Guide p18 peripherals -> File map 

            Timer_An: n = # of CCR's
                 TAx: Instance of Timer_A
            Therefore:
            TA0 is the first instance of Timer_A5

        As we will cover in great detail during this chapter, these timers contain one or more Capture and
        Compare Registers (CCR); these are useful for creating sophisticated timings, interrupts and
        waveforms. The more CCR registers a timer contains, the more independent waveforms that can
        be generated. To this end, the documentation often includes the number of CCR registers when
        listing the name of the timer. For example, if TIMER_A on a given device has 5 CCR registers,
        they often name it:
            `Timer_A5`

        But wait, that’s not all. What happens when a device, such as the ‘F5529 has more than one
        instance of TIMER_A? Each of these instances needs to be enumerated as well. This is done by
        appending the instance number after the word “Timer”, as in Timer0.
        To summarize, here’s the long (and short) names for each of the ‘F5529 TIMER_A modules:
        ```
            Instance Long Name Short Name
            0        Timer0_A5 TA0
            1        Timer1_A3 TA1
            2        Timer2_A3 TA2

        The CCR0 (Capture and Control Register 0) is special. That is, it is special in comparison to the
        other CCR registers. It is only CCR0 that can be used to define the upper limit of the counter in Up
        (or UpDown) count mode.
        The other special feature of CCR0 is that it provides a dedicated interrupt (CC0IFG). In other
        words, there is an Interrupt Vector location dedicated to CC0IFG. All the other Timer_A interrupts
        share a common vector location (i.e. they make up a grouped interrupt).


        ```
        */
        let timer = TIMER0_A3.borrow(cs);
        timer
            // timer0_a3::ta0ccr0 - Timer0_A3 Capture/Compare 0
            .ta0ccr0.write(|w|
                // w: timer0_a3::ta0ccr0::W
                unsafe {
                    // Writes raw bits to the register
                    // Count up to CCR0=5000
                    w.bits(5000)
                }
            );
        timer
            // "0x32 - Timer0_A3 Control"
            // timer0_a3::ta0ctl
            .ta0ctl.modify(|_, w|
                // w: timer0_a3::ta0ctl::W
                w
                    // Bits 8:9 - Timer A clock source select 1 -> timer0_a3::ta0ctl::_TASSELW
                    .tassel()
                    // Timer A clock source select: 1 - ACLK -> timer0_a3::ta0ctl::W
                    .tassel_1()
                    // Bits 4:5 - Timer A mode control 1 -> timer0_a3::ta0ctl::_MCW
                    .mc()
                    // Timer A mode control: 1 - Up to CCR0 -> timer0_a3::ta0ctl::W
                    // countiung mode: Up to value of ccr0, then catch interrupt and begin counting from 0
                    .mc_1()
            );
        timer
            // Timer0_A3 Capture/Compare Control 1
            // timer0_a3::ta0cctl1
            .ta0cctl1.modify(|_, w|
                // timer0_a3::ta0cctl1::W
                w
                    // Bit 4 - Capture/compare interrupt enable
                    .ccie()
                    .set_bit()
            );
        timer
            // Timer0_A3 Capture/Compare 1
            // timer0_a3::ta0ccr1
            .ta0ccr1.write(|w|
                // w: timer0_a3::ta0ccr1::W
                // generate interupt when TimerA count to 1000
                unsafe { w.bits(1000) }
            );
         
        // Same as privious but for Timer0_A3 Capture/Compare Control 2
        timer
            .ta0cctl2.modify(|_, w|
                w
                    .ccie()
                    .set_bit()
            );
        timer
            .ta0ccr2.write(|w|
                unsafe { w.bits(4000) }
            );
    });

    unsafe {
        interrupt::enable();
    }

    loop {}
}


/*
    Macro interrupt! replaced by something like this:
        ( $NAME: ident , $path: path ) => {
            # [ allow ( non_snake_case ) ]
            # [ no_mangle ]
            pub extern "msp430-interrupt" fn $NAME ( ) {
                let _ = $ crate::interrupt::Interrupt::$NAME ;
                let f: fn() = $path ;
                f() ;
            }
        }

    msp430g2553::interrupt::Interrupt has 4 names:
        TIMER0_A0, TIMER0_A1
        TIMER1_A0, TIMER1_A1


    Vector TIMER0_A0 dedicated to CCR0
    Vector TIMER0_A1 dedicated to CCR1..CCRn

*/
interrupt!(TIMER0_A1, timer_handler);
fn timer_handler() {
    interrupt::free(|cs| {
        let timer = TIMER0_A3.borrow(cs);
        let port_1_2 = PORT_1_2.borrow(cs);

        let counter = timer.ta0r.read().bits();

        match counter {
            c if c >= 1000 && c < 2000 => {
                timer.ta0cctl1.modify(|_, w| w.ccifg().clear_bit());
                // invert bits
                port_1_2.p1out.modify(|r, w| w.p0().bit(!r.p0().bit()));
            },
            c if c >= 4000 => {
                timer.ta0cctl2.modify(|_, w| w.ccifg().clear_bit());
                // invert bits
                port_1_2.p1out.modify(|r, w| w.p6().bit(!r.p6().bit()));
            },
            _ => {}
        }

        // invert bits
        //port_1_2.p1out.modify(|r, w| w.p0().bit(!r.p0().bit())
         //                             .p6().bit(!r.p6().bit()));
    });
}

#![no_std]
#![no_main]


use cortex_m_rt::entry;
use cortex_m;
use panic_rtt_target as _;
use stm32f1xx_hal::{
    prelude::*,
    pac::{self},
    timer::{Channel, Tim2NoRemap},
    adc
};
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // let pac_peripherals = pac::Peripherals::take().unwrap();
    // let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    //
    // let mut flash = pac_peripherals.FLASH.constrain();
    // let rcc = pac_peripherals.RCC.constrain();
    //
    // // let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(72.MHz()).freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);
    //
    // let mut afio = pac_peripherals.AFIO.constrain();
    // let mut gpioa = pac_peripherals.GPIOA.split();
    // let servo = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    //
    // let mut servo_pwm = pac_peripherals.TIM2.pwm_hz::<Tim2NoRemap, _, _>(servo, &mut afio.mapr, 1.kHz(), &clocks);
    // 
    // let max_duty = servo_pwm.get_max_duty();
    // 
    // // servo_pwm.enable(Channel::C1);
    // servo_pwm.enable(Channel::C2);
    // // servo_pwm.enable(Channel::C3);
    // // servo_pwm.enable(Channel::C4);
    // 
    // 
    // loop {
    //     servo_pwm.set_duty(Channel::C2, max_duty / 2);    
    //     // servo_pwm.set_duty(Channel::C1, max_duty / 2);    
    //     // servo_pwm.set_duty(Channel::C3, max_duty / 2);    
    //     // servo_pwm.set_duty(Channel::C4, max_duty / 2);    
    // }
    let pac_peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

    let mut flash = pac_peripherals.FLASH.constrain();
    let rcc = pac_peripherals.RCC.constrain();

    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(72.MHz()).freeze(&mut flash.acr);
    let clocks = rcc.cfgr.adcclk(8.MHz()).use_hse(8.MHz()).sysclk(72.MHz()).freeze(&mut flash.acr);
    // let adc_clocks = 
    // let adc_clocks = rcc.cfgr.adcclk(8.MHz()).freeze(&mut flash.acr);

    let mut afio = pac_peripherals.AFIO.constrain();

    let mut gpioa = pac_peripherals.GPIOA.split();
    // let mut gpiob = p.GPIOB.split();

    // TIM2
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    // let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // If you don't want to use all channels, just leave some out
    // let c4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);
    let pins = (c1, c2);

    // TIM3
    // let c1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    // let c2 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    // let c3 = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    // let c4 = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);

    // TIM4 (Only available with the "medium" density feature)
    // let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    // let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    // let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    //let mut pwm =
    //    Timer::new(p.TIM2, &clocks).pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz());
    // or
    let mut pwm = pac_peripherals
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 50.Hz(), &clocks);

    // Enable clock on each of the channels
    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    // pwm.enable(Channel::C3);

    //// Operations affecting all defined channels on the Timer

    // Adjust period to 0.5 seconds
    // pwm.set_period(ms(500).into_rate());

    // asm::bkpt();

    // Return to the original frequency
    // pwm.set_period(1.kHz());

    // asm::bkpt();

    let max_duty = pwm.get_max_duty();

    //// Operations affecting single channels can be accessed through
    //// the Pwm object or via dereferencing to the pin.

    // Use the Pwm object to set C3 to full strength
    // pwm.set_duty(Channel::C3, max);

    // asm::bkpt();

    // Use the Pwm object to set C3 to be dim
    // pwm.set_duty(Channel::C3, max / 4);

    // asm::bkpt();

    // Use the Pwm object to set C3 to be zero
    // pwm.set_duty(Channel::C3, 0);

    // asm::bkpt();

    // Extract the PwmChannel for C3
    // let mut pwm_led_channel = pwm.split().0;
    // let mut pwm_servo_channel = pwm.split().1;
    let pwm_channels = pwm.split();
    let mut pwm_led_channel = pwm_channels.0;
    let mut pwm_servo_channel = pwm_channels.1;

    // Use the PwmChannel object to set C3 to be full strength
    // pwm_channel.set_duty(max);

    // asm::bkpt();

    // Use the PwmChannel object to set C3 to be dim
    // pwm_channel.set_duty(max / 4);

    // asm::bkpt();

    // Use the PwmChannel object to set C3 to be zero
    // pwm_channel.set_duty(0);

    // asm::bkpt();
    let mut delay = cortex_peripherals.SYST.delay(&clocks);

    let mut adc1 = adc::Adc::adc1(pac_peripherals.ADC1, clocks);
    let mut gpiob = pac_peripherals.GPIOB.split();
    let mut analog_pin = gpiob.pb0.into_analog(&mut gpiob.crl);
    
    loop {
        // pwm_channel.set_duty(max);
        // rprintln!("{}", pwm_channel.get_duty());
        // data = adc1.read(&mut analog_pin).unwrap();
        // rprintln!("analog data: {}", data);
        // delay.delay_ms(1000_u16);
        //
        // pwm_channel.set_duty(max / 4);
        // rprintln!("{}", pwm_channel.get_duty());
        // data = adc1.read(&mut analog_pin).unwrap();
        // rprintln!("analog data: {}", data);
        // delay.delay_ms(1000_u16);
        //
        // pwm_channel.set_duty(0);
        // rprintln!("{}", pwm_channel.get_duty());
        // data = adc1.read(&mut analog_pin).unwrap();
        // rprintln!("analog data: {}", data);
        // delay.delay_ms(1000_u16);
        // pwm_channel.set_duty(max as f32 * convert_to_float(adc1.read(&mut analog_pin).unwrap()));
        let data: u16 = adc1.read(&mut analog_pin).unwrap();
        let pwm_value = map(data as i32, 0, 4096, 0, max_duty as i32);
        pwm_servo_channel.set_duty(pwm_value as u16);
        pwm_led_channel.set_duty(pwm_value as u16);
        rprintln!("PWM value: {}, analog value: {}", pwm_value, data);
        delay.delay_ms(10_u16);

        
    }
}


fn map(input: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    // https://www.arduino.cc/reference/en/language/functions/math/map/
    
    return (input - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

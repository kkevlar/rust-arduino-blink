// main.rs

#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;
use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Morse {
    Dot,
    Dash,
    Space,
    Error,
    LongSpace,
}

fn char_to_morse(c: char) -> Morse {
    use Morse::*;
    match c {
        '0' => Dot,
        '1' => Dash,
        _ => Error,
    }
}

fn string_to_code(code: &String) -> Vec<Morse> {

   code.chars().map(char_to_morse).collect()
}

fn ez(o : (&String,&char)) -> (Vec<Morse>, char)
{
    match o {
        (str,c) => (string_to_code(str), c.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_code() {
        use Morse::*;
        assert_eq!(string_to_code(&"0".to_string()), vec![Dot]);
        assert_eq!(string_to_code(&"01".to_string()), vec![Dot, Dash]);
    }
}

fn ree(mm : &HashMap<Vec<Morse>, char>) -> Option<&char>
{
    let c  = mm.get(&vec![Morse::Dot, Morse::Dash, Morse::Dot]);
    println!("{}", c?);
    c
}

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}

#[arduino_uno::entry]
fn main() -> ! {

    let morse_key: HashMap<String, char> = hashmap![
    "01".to_string() => 'a',
    "1000".to_string() => 'b',
    "1010".to_string() => 'c',
    "100".to_string() => 'd',
    "0".to_string() => 'e',
    "0010".to_string() => 'f',
    "110".to_string() => 'g',
    "0000".to_string() => 'h',
    "00".to_string() => 'i',
    "0111".to_string() => 'j',
    "101".to_string() => 'k',
    "0100".to_string() => 'l',
    "11".to_string() => 'm',
    "10".to_string() => 'n',
    "111".to_string() => 'o',
    "0110".to_string() => 'p',
    "1101".to_string() => 'q',
    "010".to_string() => 'r',
    "000".to_string() => 's',
    "1".to_string() => 't',
    "001".to_string() => 'u',
    "0001".to_string() => 'v',
    "011".to_string() => 'w',
    "1001".to_string() => 'x',
    "1011".to_string() => 'y',
    "1100".to_string() => 'z'
    ];

    let mm : HashMap<Vec<Morse>, char> = morse_key.iter().map(ez).collect();

    ree(&mm);
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut led = pins.d13.into_output(&mut pins.ddr);
    
    loop {
        stutter_blink(&mut led, 25);
    }
}

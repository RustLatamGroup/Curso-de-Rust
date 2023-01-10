fn main() {
    let mut value = 0b1111_0101u8;
    println!("values is {}", value);
    println!("values is {:08b}", value);

    value = !value;
    println!("values is {:08b}", value);

    value = value & 0b1111_0111;
    println!("values is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("values is {:08b}", value);

    value = value ^ 0b0101_0101;
    println!("values is {:08b}", value);

    value = value << 4;
    println!("value is {0:08b}", value);

    value = value >> 2;
    println!("value is {0:08b}", value);
}

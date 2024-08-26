fn main() {
   let version: u8 = 4;
   let header_length: u8 = 5;
   let packed_byte = (version << 4) | (header_length & 0x0F);
   println!("{:08b}", packed_byte);
   println!("{} bytes", std::mem::size_of_val(&packed_byte))
}

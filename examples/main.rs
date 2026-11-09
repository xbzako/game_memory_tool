use game_memory_tool::core::*;

fn main() -> Result<()> {
    let process = Core::new("main.exe").unwrap();
    println!("{}", process.pid().unwrap());    

    let pattern = String::from("86");    
    let address = process.scan_pattern(&bytes_string_to_vec_u8(&pattern), 0x000000000, 0xFFFFFFFF);
    for addr in address {
        println!("{:08X}", addr);
    }
    Ok(())
}

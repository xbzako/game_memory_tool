use game_memory_tool::core::*;

fn main() -> Result<()> {
    let process = Core::new("main.exe").unwrap();
    println!("PID: {}", process.pid().unwrap());

    let pattern = "86 ??";
    let address = process.scan_pattern(
        &bytes_string_to_vec_u8(&pattern),
        Address(Some(0x00000000)),
        0xFFFFFFFFusize.into(),
    );
    for addr in address {
        println!("Found address: {} ", addr);
    }

    println!("{}", Address(None));
    Ok(())
}

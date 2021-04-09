pub mod ipv4;
pub mod assembler;

#[cfg(test)]
mod tests {
    #[test]
    fn asm() {
        println!("{:#?}", crate::assembler::simple_assembler(vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ]));
        println!("{:#?}", crate::assembler::simple_assembler(vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"]));
    }
}
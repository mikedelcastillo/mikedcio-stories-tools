use rand::Rng;

pub const ID_CHAR_POOL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPRQSTUVWXYZ0123456789";

pub fn generate_id(len: u32) -> String {
    let mut out = String::new();
    for _ in 0..len {
        let index = rand::thread_rng().gen_range(0..ID_CHAR_POOL.len());
        let ch = &ID_CHAR_POOL[index..=index];
        out.push_str(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        for len in [0, 2, 16, 512, 2048] {
            let id = generate_id(len);
            assert_eq!(len, id.len() as u32)
        }
    }
}

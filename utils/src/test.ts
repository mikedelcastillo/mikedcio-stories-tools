export const ID_CHAR_POOL = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPRQSTUVWXYZ0123456789";

export function generateId(len: number): string {
    let out = ""
    for(let i = 0; i < len; i++){
        let index = Math.floor(Math.random() * ID_CHAR_POOL.length)
        let ch = ID_CHAR_POOL[index]
        out += ch
    }
    return out
}



use super::load;

fn decrypt(cipher: &[i64], key: &[i64]) -> Vec<u8> {
    let mut result = Vec::with_capacity(cipher.len());
    let kl = key.len();
    for (i, c) in cipher.iter().enumerate() {
        let k = key[i % kl];
        result.push((c ^ k) as u8);
    }

    result
}

const CHAR_A: i64 = 'a' as i64;
const CHAR_Z: i64 = 'z' as i64;

struct KeyIterator {
    key: [i64; 3],
}

impl KeyIterator {
    fn new() -> Self {
        Self { key: [CHAR_A, CHAR_A, CHAR_A - 1] }
    }
}

impl Iterator for KeyIterator {
    type Item = [i64; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.key.len() - 1;
        self.key[last] += 1;

        let mut carry = 0;
        for i in (0..self.key.len()).rev() {
            self.key[i] += carry;
            if self.key[i] > CHAR_Z {
                carry = 1;
                self.key[i] = CHAR_A;
            } else {
                carry = 0;
            }
        }

        if carry > 0 {
            None
        } else {
            Some(self.key)
        }
    }
}

enum State {
    Init,
    FoundT,
    FoundTH,
    FoundTHE,
}

fn check(plain: &[u8]) -> bool {
    let mut state = State::Init;
    let mut result = false;

    for &c in plain {
        if !(32..=126).contains(&c) {
            result = false;
            break;
        }
    
        match state {
            State::Init => {
                if c == b't' {
                    state = State::FoundT;
                }
            }
            State::FoundT => {
                state = match c {
                    b'h' => State::FoundTH,
                    b't' => State::FoundT,
                    _ => State::Init,
                }
            }
            State::FoundTH => {
                state = match c {
                    b'e' => State::FoundTHE,
                    b't' => State::FoundT,
                    _ => State::Init,
                };
            }
            State::FoundTHE => {
                if c == b' ' {
                    result = true;
                    break;
                } else if c == b't' {
                    state = State::FoundT;
                } else {
                    state = State::Init;
                }
            }
        }
    }

    result
}

pub fn solve() -> i64 {
    let data = load();
    let keys = KeyIterator::new();

    for key in keys {
        let decrypted = decrypt(&data, &key);
        if !check(&decrypted) {
            continue;
        }

        let sum = decrypted.iter().map(|&c| c as i64).sum();
        return sum;
    }

    0
}

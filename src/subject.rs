#[derive(Debug, Clone)]
pub struct Subject {
    pub text: String,
    pub base_text: String,
}

impl Subject {
    pub fn mangle(self: &Subject) -> Subject {
        Subject::rcovr(self.clone())
            .and_then(Subject::rcovr)
            .and_then(Subject::rcovr)
            .and_then(Subject::rcovr)
            .and_then(Subject::rcovr)
            .and_then(Subject::rcovr)
            .and_then(Subject::rcovr)
            .and_then(Subject::taint)
            .unwrap_or_else(|| self.clone())
    }

    fn taint(taint_me: Subject) -> Option<Subject> {
        let up: CharPos = pick_rand(&taint_me.text.len(), Subject::CHARSET);

        let a = &taint_me.text[0..up.idx];
        let b = &taint_me.text[up.idx + 1..taint_me.text.len()];

        Some(Subject {
            text: [a, &String::from(up.c), b].concat(),
            ..taint_me
        })
    }
    fn rcovr(recover_me: Subject) -> Option<Subject> {
        let recover: CharPos = pick_rcvr(&recover_me.text.len(), recover_me.base_text.as_bytes());
        let x = &recover_me.text[0..recover.idx];
        let y = &recover_me.text[recover.idx + 1..recover_me.text.len()];

        Some(Subject {
            text: [x, &String::from(recover.c), y].concat(),
            ..recover_me
        })
    }
}

impl Consts for Subject {
    const CHARSET: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~_ ";
}

#[derive(Debug)]
struct CharPos {
    c: char,
    idx: usize,
}

trait Consts {
    const CHARSET: &'static [u8];
}

fn pick_rcvr(available_pos: &usize, base: &[u8]) -> CharPos {
    use rand::Rng;
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let pool_idx: usize = std::cmp::min(rng.gen_range(0..base.len()), *available_pos - 1);

    CharPos {
        c: char::from(base[pool_idx]),
        idx: pool_idx,
    }
}

fn pick_rand(available_pos: &usize, charset: &[u8]) -> CharPos {
    use rand::Rng;
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let pool_idx: usize = rng.gen_range(0..charset.len());
    let dest_idx: usize = rng.gen_range(0..*available_pos);

    CharPos {
        c: char::from(charset[pool_idx]),
        idx: dest_idx,
    }
}

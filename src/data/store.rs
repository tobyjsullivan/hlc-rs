use crate::data::AccountID;

pub struct Store {
    ids: Box<IdMask>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            ids: Box::new(IdMask::new()),
        }
    }

    pub fn mark_account(&mut self, acct_id: AccountID) {
        self.ids.mark(acct_id);
    }
}


type Block = u64;
struct IdMask {
    entries: Vec<Block>,
}

impl IdMask {
    const BITS_PER_BLOCK: usize = 64;

    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn mark(&mut self, acct_id: AccountID) {
        let (block, bit) = Self::locate(acct_id);
        while self.entries.len() <= block {
            self.entries.push(0);
        }
        let cur_block = self.entries[block];
        let cur_block = cur_block | (1 as Block) << bit;

        self.entries[block] = cur_block;
    }

    fn clear(&mut self, acct_id: AccountID) {
        let (block, bit) = Self::locate(acct_id);
        if self.entries.len() <= block {
            return;
        }
        let cur_block = self.entries[block];
        let cur_block = cur_block & !((1 as Block) << bit);

        self.entries[block] = cur_block;
    }

    fn check(&self, acct_id: AccountID) -> bool {
        let (block, bit) = Self::locate(acct_id);
        if self.entries.len() <= block {
            return false;
        }
        let cur_block = self.entries[block];
        return cur_block & (1 << bit) as Block > 0;
    }

    fn locate(acct_id: AccountID) -> (usize, usize) {
        (acct_id as usize / Self::BITS_PER_BLOCK, acct_id as usize % Self::BITS_PER_BLOCK)
    }
}

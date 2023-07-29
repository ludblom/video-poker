pub struct Bank {
    pub status: i64,
    pub betting: u64,
    pub no_of_hands: u64,
}

impl Bank {
    pub fn new() -> Bank {
        Bank { status: 1000, betting: 100, no_of_hands: 0 }
    }

    pub fn add_funds(&mut self) {
        self.status += self.betting as i64;
    }

    pub fn subtract_funds(&mut self) {
        self.status -= self.betting as i64;
    }

    pub fn add_hand_played(&mut self) {
        self.no_of_hands += 1;
    }
}

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let len = self.scores.len();
        if len == 0 {
            None
        } else {
            Some(self.scores[len - 1])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let len = self.scores.len();
        if len == 0 {
            None
        } else {
            let s = self.scores;
            let x = s.iter().fold(0, |acc, x| if acc > *x { acc } else { *x });
            Some(x)
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut s = self.scores.to_vec();
        s.sort();
        let res = match s.len() {
            0 => Vec::new(),
            1 => s,
            2 => {
                let mut x = Vec::new();
                x.push(s[s.len() - 1]);
                x.push(s[s.len() - 2]);
                x
            }
            _ => {
                let mut x = Vec::new();
                x.push(s[s.len() - 1]);
                x.push(s[s.len() - 2]);
                x.push(s[s.len() - 3]);
                x
            }
        };
        res
    }
}

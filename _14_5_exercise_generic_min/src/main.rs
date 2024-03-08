trait LessThan {
    /// Return true if self is less than other.
    fn less_than(&self, other: &Self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Citation {
    author: &'static str,
    year: u32,
}

impl LessThan for Citation {
    fn less_than(&self, other: &Self) -> bool {
        match self.author.cmp(other.author) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => self.year < other.year,
        }
    }
}

fn min<T: LessThan>(a: T, b: T) -> T {
    if a.less_than(&b) {
        a
    } else {
        b
    }
}

fn main() {
    let cit1 = Citation {
        author: "Shapiro",
        year: 2011,
    };
    let cit2 = Citation {
        author: "Baumann",
        year: 2010,
    };
    let cit3 = Citation {
        author: "Baumann",
        year: 2019,
    };
    debug_assert_eq!(min(cit1, cit2), cit2);
    debug_assert_eq!(min(cit2, cit3), cit2);
    debug_assert_eq!(min(cit1, cit3), cit3);
}

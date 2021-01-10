use std::vec::Vec;
use std::cmp::PartialEq;
use std::default::Default;
use crate::error::ErrorKind;

#[derive(Debug)]
struct NumHrd {
    /// 华容道的边长
    size: u8,
    /// 0.0 -> 0.1 -> 0.2 -> 1.0 -> 1.1 -> 1.2 -> 2.0 -> ...
    /// 排序顺序
    nums: Vec<Num>,
}

impl NumHrd {
    pub fn new(s: &u8) -> Self {
        let mut nums:Vec<Num> = Vec::new();
        let nums_len = s * s;
        for i in 0..nums_len {
            let num = Num::new(&i, s);
            nums.push(num);
        }
        Self {
            size: *s,
            nums,
        }
    }

    pub fn exchange(&mut self, one_index: &usize, other_index: &usize) -> Result<(), ErrorKind> {

        let one = self.get_by_index(one_index);
        let other = self.get_by_index(other_index);
        if !self.is_neighbouring(one, other) {
            return Err(ErrorKind::CannotExchangeNotNeighbouring);
        }
        if !one.is_empty() && !other.is_empty() {
            return Err(ErrorKind::CannotExchangeNoneZero);
        }
        let mut temp: Num = Num::default();
        let mut i = 0;
        let mut one_i = 0usize;
        let mut other_i = 0usize;
        for num in &self.nums {
            if num == one {
                one_i = i;
            }

            if num == other {
                other_i = i;
            }
            i += 1;
        }
        temp = self.nums[one_i];
        self.nums[one_i] = self.nums[other_i];
        self.nums[other_i] = temp;
        Ok(())
    }

    pub fn is_neighbouring(&self,  one: &Num, other: &Num) -> bool {
        (one.pos.0 == other.pos.0 && (one.pos.1 as i32 - other.pos.1 as i32).abs() == 1)
            || (one.pos.1 == other.pos.1 && (one.pos.1 as i32 - other.pos.1 as i32).abs() == 1) 
    }

    pub fn get_by_index(&self, n: &usize) -> &Num {
        &self.nums[*n]
    }

    pub fn is_win(&self) -> bool {
        let mut res = false;
        let len = self.len();
        for i in 1..len {
            res = self.get_by_index(&(i - 1)).n as usize == i;
            if !res {
                break
            }
        }
        res
    }

    pub fn shuffle(&mut self) {

    }

    pub fn len(&self) -> usize {
        self.size as usize * self.size as usize
    }
}

/// 表示一个数字块
/// pos: 坐标位置， 表示第m行，第n列  从0开始计数
/// n: 表示具体数字
#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct Num {
    pos: (u8, u8),
    n: u8,
}

impl Num {

    /// 生成一个新的块
    /// n 表示生成的数字
    /// s 表示整个华容道的大小， 用来确定数字的初始位置
    pub fn new(n: &u8, s: &u8) -> Self {
        let x: u8 = n / s;
        let y: u8 = n % s;
        Self {
            pos: (x, y),
            n: *n,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}


#[cfg(test)]
mod tests {
    mod num_tests {

        use super::super::*;

        #[test]
        fn new_works() {
            let num = Num::new(&1, &3);
            assert_eq!(num.pos, (0, 1));
            assert_eq!(num.n, 1);
            let num = Num::new(&2, &3);
            assert_eq!(num.pos, (0, 2));
            assert_eq!(num.n, 2);
            let num = Num::new(&3, &3);
            assert_eq!(num.pos, (1, 0));
            assert_eq!(num.n, 3);
            let num = Num::new(&6, &3);
            assert_eq!(num.pos, (2, 0));
            assert_eq!(num.n, 6);
            let num = Num::new(&4, &3);
            assert_eq!(num.pos, (1, 1));
            assert_eq!(num.n, 4);
            let num = Num::new(&5, &3);
            assert_eq!(num.pos, (1, 2));
            assert_eq!(num.n, 5);
        }
    }

    mod num_hrd_tests {
        use super::super::*;

        #[test]
        fn new_works() {
            let size = 3;
            let num_hrd = NumHrd::new(&size);
            assert_eq!(num_hrd.size, size);
            assert_eq!(num_hrd.nums.len(), (size * size) as usize);
            let mut i = 0;
            for num in num_hrd.nums {
                assert_eq!(num.n, i);
                assert_eq!(num.pos, (i / size as u8, i % size as u8));
                i += 1;
            }
        }

        #[test]
        fn exchange_works() {
            let mut num_hrd = NumHrd::new(&3);
            let exchange_res = num_hrd.exchange(&0, &1);
            println!("{:?}", num_hrd);
            assert_eq!(num_hrd.get_by_index(&1).n, 0);

        }

        #[test]
        fn exchange_panic() {
            let mut num_hrd = NumHrd::new(&3);
            let exchange_res = num_hrd.exchange(&2, &1);
            assert_eq!(exchange_res, Err(ErrorKind::CannotExchangeNoneZero));
            let exchange_res = num_hrd.exchange(&0, &4);
            assert_eq!(exchange_res, Err(ErrorKind::CannotExchangeNotNeighbouring));
        }

        #[test]
        fn is_win_works() {
            let mut num_hrd = NumHrd::new(&3);
            assert_eq!(num_hrd.is_win(), false);
            num_hrd.nums[0] = Num {
                pos: (0, 0),
                n: 1,
            };
            num_hrd.nums[1] = Num {
                pos: (0, 1),
                n: 2,
            };
            num_hrd.nums[2] = Num {
                pos: (0, 2),
                n: 3,
            };
            num_hrd.nums[3] = Num {
                pos: (1, 0),
                n: 4,
            };
            num_hrd.nums[4] = Num {
                pos: (1, 1),
                n: 5,
            };
            num_hrd.nums[5] = Num {
                pos: (1, 2),
                n: 6,
            };
            num_hrd.nums[6] = Num {
                pos: (2, 0),
                n: 7,
            };
            num_hrd.nums[7] = Num {
                pos: (2, 1),
                n: 8,
            };
            assert_eq!(num_hrd.is_win(), true);
        }
    }
}
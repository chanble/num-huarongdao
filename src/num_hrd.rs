use std::vec::Vec;
use std::cmp::PartialEq;
use std::default::Default;

use crate::error::ErrorKind;

pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

///
/// fifteen puzzle game lib
/// 数字华容道
/// 
/// Usage : 
/// ```rust
/// use num_huarongdao::num_hrd::NumHrd;
/// // 生成一个3 x 3的游戏
/// let mut num_hrd = NumHrd::new(&3);
/// 
/// num_hrd.move_num(1usize);
/// ```
/// 
///   
#[derive(Debug, Clone, PartialEq)]
pub struct NumHrd {
    /// 华容道的边长
    size: u8,
    /// 0.0 -> 0.1 -> 0.2 -> 1.0 -> 1.1 -> 1.2 -> 2.0 -> ...
    /// 排序顺序
    nums: Vec<Num>,
}

impl NumHrd {

    /// create a num huarongdao
    /// s: side num
    /// 
    pub fn new(s: &u8) -> Self {
        let mut nums:Vec<Num> = Vec::new();
        let nums_len: usize = (s * s).into();
        for i in 1..nums_len {
            let num = Num::new(&i);
            nums.push(num);
        }
        nums.push(Num::new(&0));
        Self {
            size: *s,
            nums,
        }
    }

    ///
    /// 返回2维坐标数组
    /// 
    pub fn as_2d_vec(&self) -> Vec<Vec<usize>> {
        let mut rows: Vec<Vec<usize>> = Vec::new();
        for i in 0..self.size {
            let mut row: Vec<usize> = Vec::new();
            for j in 0..self.size {
                let index: usize = ((i * self.size ) + j).into();
                let num_opt = self.num_by_index(&index);
                match num_opt {
                    Some(num) => {
                        row.push(num.get_n());
                    },
                    None => {
                        // do nothing...
                        panic!("as_2d_vec error on num_by_index index:{:?}", index);
                    }
                }
            }
            rows.push(row);
        }
        rows
    }

    ///
    /// 交换两个块的位置
    /// 
    pub fn exchange(&mut self, one_index: &usize, other_index: &usize) -> Result<(), ErrorKind> {

        let one_opt = self.num_by_index(one_index);
        let other_opt = self.num_by_index(other_index);
        if one_opt.is_none() || other_opt.is_none() {
            return Err(ErrorKind::CannotExchangeNotNeighbouring);
        }
        let one = one_opt.unwrap();
        let other = other_opt.unwrap();
        if !self.is_neighbouring(one_index, other_index) {
            return Err(ErrorKind::CannotExchangeNotNeighbouring);
        }
        if !one.is_empty() && !other.is_empty() {
            return Err(ErrorKind::CannotExchangeNoneZero);
        }
        if one_index == other_index {
            return Ok(());
        }

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
        let temp = self.nums[one_i];
        self.nums[one_i].n = self.nums[other_i].n;
        self.nums[other_i].n = temp.n;
        Ok(())
    }

    /// 两个块是否相邻
    pub fn is_neighbouring(&self,  one: &usize, other: &usize) -> bool {
        let diff = (*one as i64 - *other as i64).abs();
        diff == 1 || diff == self.size as i64
    }

    ///
    /// get Num ref object by index of nums
    /// 
    pub fn num_by_index(&self, n: &usize) -> Option<&Num> {
        if *n > self.nums.len() {
            return None;
        }
        Some(&self.nums[*n])
    }

    ///
    /// get Num ref object by point(row_index, col_index) of nums
    /// 
    pub fn num_by_point(&self, p: (u8, u8)) -> Option<&Num> {
        self.num_by_index(&self.index_by_point(p))
    }

    pub fn data(&self) -> &Vec<Num> {
        &self.nums
    }

    pub fn index_by_point(&self, p: (u8, u8)) -> usize {
        (p.0 * self.size + p.1) as usize
    }

    pub fn size(&self) -> &u8 {
        &self.size
    }
    /// 
    /// 得到某个数字的索引
    /// 
    pub fn index_by_n(&self, n: &usize) -> Option<usize> {
        self.nums.iter().position(|x| x.n == *n)
    }
    /// 判断是否成功
    /// 
    pub fn is_win(&self) -> bool {
        self.nums == Self::new(&self.size).nums
    }

    pub fn len(&self) -> usize {
        self.size as usize * self.size as usize
    }

    ///
    /// 移动空格所在的位置
    /// d: direction 空格想移动的方向
    pub fn zero_move(&mut self, d: &Direction) -> Result<bool, ErrorKind> {
        let zero_index_opt = self.index_by_n(&0);
        match zero_index_opt {
            Some(zero_index) => {
                if let Some(other_index) = self.get_dirction_index(&zero_index, d) {
                    self.exchange(&zero_index, &other_index)?;
                }
                Ok(true)
            },
            None => {
                Err(ErrorKind::ZeroNotFound)
            }
        }
    }

    ///得到指定index位置指定方向的index
    /// index 表示指定的索引
    /// d 表示指定的方向
    fn get_dirction_index(&self, index: &usize, d: &Direction) -> Option<usize> {
        let tmp_size = self.size as usize;
        match d {
            Direction::Left => {
                if index % tmp_size == 0 {
                    return None;
                } else {
                    return Some(index - 1);
                }
            },
            Direction::Right => {
                if index % tmp_size + 1 == tmp_size {
                    return None;
                } else {
                    return Some(index + 1);
                }
            },
            Direction::Top => {
                if index / tmp_size == 0 {
                    return None;
                } else {
                    return Some(index - tmp_size);
                }
            },
            Direction::Bottom => {
                if index / tmp_size == tmp_size - 1 {
                    return None;
                } else {
                    return Some(index + tmp_size);
                }
            }
        }
    }

    ///
    /// 移动指定索引的块
    /// 
    pub fn move_num(&mut self, index: usize) -> bool {
        if let Some(zero_index) = self.index_by_n(&0) {
            return match self.exchange(&index, &zero_index) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        false
    }

    pub fn move_num_by_point(&mut self, point: (u8, u8)) -> bool {
        let index = self.index_by_point(point);
        self.move_num(index)
    }
}

/// 表示一个数字块
/// pos: 坐标位置， 表示第m行，第n列  从0开始计数
/// n: 表示具体数字
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Num {
    n: usize,
}

impl Num {

    /// 生成一个新的块
    /// n 表示生成的数字
    /// s 表示整个华容道的大小， 用来确定数字的初始位置
    pub fn new(n: &usize) -> Self {
        Self {
            n: *n,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn get_n(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod tests {
    mod num_tests {

        use super::super::*;

        #[test]
        fn new_works() {
            let num = Num::new(&1);
            assert_eq!(num.n, 1);
            let num = Num::new(&2);
            assert_eq!(num.n, 2);
            let num = Num::new(&3);
            assert_eq!(num.n, 3);
            let num = Num::new(&6);
            assert_eq!(num.n, 6);
            let num = Num::new(&4);
            assert_eq!(num.n, 4);
            let num = Num::new(&5);
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
        }

        #[test]
        fn num_by_point_works() {
            let num_hrd = NumHrd::new(&3);
            assert_eq!(num_hrd.num_by_point((0, 0)), Some(&Num { n: 1}));
            assert_eq!(num_hrd.num_by_point((0, 1)), Some(&Num { n: 2}));
            assert_eq!(num_hrd.num_by_point((0, 2)), Some(&Num { n: 3}));
            assert_eq!(num_hrd.num_by_point((1, 0)), Some(&Num { n: 4}));
            assert_eq!(num_hrd.num_by_point((1, 1)), Some(&Num { n: 5}));
            assert_eq!(num_hrd.num_by_point((1, 2)), Some(&Num { n: 6}));
            assert_eq!(num_hrd.num_by_point((2, 0)), Some(&Num { n: 7}));
            assert_eq!(num_hrd.num_by_point((2, 1)), Some(&Num { n: 8}));
            assert_eq!(num_hrd.num_by_point((2, 2)), Some(&Num { n: 0}));
        }
        #[test]
        fn exchange_works() {
            let mut num_hrd = NumHrd::new(&3);
            let _ = num_hrd.exchange(&0, &8);
            println!("{:?}", num_hrd);
            assert_eq!(num_hrd.num_by_index(&8), Some(&Num {
                n: 0,
            }));

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
            assert_eq!(num_hrd.is_win(), true);
            num_hrd.nums[0] = Num {
                n: 1,
            };
            num_hrd.nums[1] = Num {
                n: 2,
            };
            num_hrd.nums[2] = Num {
                n: 3,
            };
            num_hrd.nums[3] = Num {
                n: 4,
            };
            num_hrd.nums[4] = Num {
                n: 5,
            };
            num_hrd.nums[5] = Num {
                n: 6,
            };
            num_hrd.nums[6] = Num {
                n: 7,
            };
            num_hrd.nums[7] = Num {
                n: 8,
            };
            num_hrd.nums[8] = Num {
                n: 0,
            };
            assert_eq!(num_hrd.is_win(), true);
        }

        #[test]
        fn get_dirction_index_works() {
            let numhrd = NumHrd::new(&3);
            assert_eq!(numhrd.get_dirction_index(&0, &Direction::Top), None);
            assert_eq!(numhrd.get_dirction_index(&0, &Direction::Left), None);
            assert_eq!(numhrd.get_dirction_index(&0, &Direction::Right), Some(1));
            assert_eq!(numhrd.get_dirction_index(&0, &Direction::Bottom), Some(3));
            assert_eq!(numhrd.get_dirction_index(&4, &Direction::Top), Some(1));
            assert_eq!(numhrd.get_dirction_index(&4, &Direction::Left), Some(3));
            assert_eq!(numhrd.get_dirction_index(&4, &Direction::Right), Some(5));
            assert_eq!(numhrd.get_dirction_index(&4, &Direction::Bottom), Some(7));
            assert_eq!(numhrd.get_dirction_index(&8, &Direction::Top), Some(5));
            assert_eq!(numhrd.get_dirction_index(&8, &Direction::Left), Some(7));
            assert_eq!(numhrd.get_dirction_index(&8, &Direction::Right), None);
            assert_eq!(numhrd.get_dirction_index(&8, &Direction::Bottom), None);
        }

        #[test]
        fn zero_move_works() {
            let mut numhrd = NumHrd::new(&3);
            numhrd.zero_move(&Direction::Left).unwrap();
            assert_eq!(numhrd.num_by_index(&8), Some(&Num {
                n: 8,
            }));
            numhrd.zero_move(&Direction::Top).unwrap();
            assert_eq!(numhrd.num_by_index(&7), Some(&Num {
                n: 5,
            }));
        }
    }
}
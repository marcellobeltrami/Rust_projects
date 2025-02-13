use crate::sorting::*;

#[cfg(test)]
mod tests {

    #[test]
    fn test_bubble() {
        let mut vec = vec![4, 2, 3, 1];
        bubble(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![1, 2, 3, 4];
        bubble(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![4, 3, 2, 1];
        bubble(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec: Vec<i32> = vec![];
        bubble(&mut vec);
        assert_eq!(vec, vec![]);
    }
/* 
    #[test]
    fn test_quick() {
        let mut vec = vec![4, 2, 3, 1];
        quick(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![1, 2, 3, 4];
        quick(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![4, 3, 2, 1];
        quick(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec: Vec<i32> = vec![];
        quick(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_merge() {
        let mut vec = vec![4, 2, 3, 1];
        vec = merge(vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![1, 2, 3, 4];
        vec = merge(vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![4, 3, 2, 1];
        vec = merge(vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let vec: Vec<i32> = vec![];
        let sorted_vec = merge(vec);
        assert_eq!(sorted_vec, vec![]);
    }

    #[test]
    fn test_insertion() {
        let mut vec = vec![4, 2, 3, 1];
        insertion(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![1, 2, 3, 4];
        insertion(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![4, 3, 2, 1];
        insertion(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec: Vec<i32> = vec![];
        insertion(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_selection() {
        let mut vec = vec![4, 2, 3, 1];
        selection(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![1, 2, 3, 4];
        selection(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec = vec![4, 3, 2, 1];
        selection(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let mut vec: Vec<i32> = vec![];
        selection(&mut vec);
        assert_eq!(vec, vec![]);
    }
*/

}
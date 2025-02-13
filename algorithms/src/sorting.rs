
pub fn bubble<T: Ord>(arr: &mut [T]){

    if arr.len() != 0 {
        
        for i in 0..arr.len() {
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }

    } else {
        panic!("Array is empty")
    }
}


#[test]
fn test_1() {
    let mut vec = vec![4, 2, 3, 1];
    bubble(&mut vec);
    assert_eq!(vec, vec![1, 2, 3, 4]);

  
}

#[test]
fn test_2() {
    let mut vec = vec![1, 2, 3, 4];
    bubble(&mut vec);
    assert_eq!(vec, vec![1, 2, 3, 4]);
  
}

#[test]
fn test_3(){
    let mut vec = vec![4, 3, 2, 1];
    bubble(&mut vec);
    assert_eq!(vec, vec![1, 2, 3, 4]);

    }

#[test]
fn test_4(){
    let mut vec: Vec<i32> = vec![];
    bubble(&mut vec);
    assert_eq!(vec, vec![]);
}
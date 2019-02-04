use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn merge_trees(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    return merge_node(t1, t2);
}

fn merge_node(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let tuple = (t1, t2);

    match tuple {
        (Some(n1), Some(n2)) => {
            let n1 = Rc::try_unwrap(n1)
                .unwrap()
                .into_inner();

            let n2 = Rc::try_unwrap(n2)
                .unwrap()
                .into_inner();

            Some(Rc::new(RefCell::new(TreeNode {
                val: n1.val + n2.val,
                left: merge_node(n1.left, n2.left),
                right: merge_node(n1.right, n2.right),
            })))
        },
        (Some(n1), None) => {
            let n1 = Rc::try_unwrap(n1)
                .unwrap()
                .into_inner();

            Some(Rc::new(RefCell::new(TreeNode {
                val: n1.val,
                left: merge_node(n1.left, None),
                right: merge_node(n1.right, None),
            })))
        },
        (None, Some(n2)) => {
            let n2 = Rc::try_unwrap(n2)
                .unwrap()
                .into_inner();

            Some(Rc::new(RefCell::new(TreeNode {
                val: n2.val,
                left: merge_node(None, n2.left),
                right: merge_node(None, n2.right),
            })))
        },
        _ => {
            None
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));

        let t2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(merge_trees(t1, t2), result);
    }
}

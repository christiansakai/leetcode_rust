pub fn judge_circle(moves: String) -> bool {
    let mut vertical = 0;
    let mut horizontal = 0;

    for m in moves.chars() {
        match m {
            'U' => {
                vertical += 1;
            },
            'D' => {
                vertical -= 1;
            },
            'L' => {
                horizontal -= 1;
            },
            _ => {
                horizontal += 1;
            },

        }
    }

    vertical == 0 && horizontal == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(judge_circle(String::from("UD")), true);
        assert_eq!(judge_circle(String::from("LL")), false);
        assert_eq!(
            judge_circle(String::from("DRLLDLLRURLURULLLRULLRLULLLDDUDLUUUDLLDLDRLDRURDURRLRDLDRDLDDURDUURLLUUURLDRLUULUUDRDRUDURLLRDRRDLDU")),
            false
        );
    }
}

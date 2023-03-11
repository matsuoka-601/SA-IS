pub mod SA_IS {
    // implementation of SA-IS
    // memory consumption is worse than the original paper's implementation 
    fn sa_is (s: &[i32], sa: &mut [i32] , k: i32) { 
        assert_eq!(*s.last().unwrap(), 0); // sentinel

        let n = s.len();
        let mut tp = vec![false; n];

        tp[n-1] = true; // sentinel is S-type

        for i in (0..=n-2).rev() {
            // check whether s[i] is S-type or L-type
            tp[i] = s[i] < s[i+1] || (s[i]==s[i+1] && tp[i+1]);
        }

        // stage 1: reduce the problem by at least 1/2
        // sort all the LMS-substrings
        let mut bkt: Vec<i32> = vec![0; k as usize];
        get_buckets(s, &mut bkt, true);
        for i in 0..n { sa[i] = -1; }
        for i in 1..n {
            if is_lms(i, &tp) { // check whether s[i] is an LMS-substring
                bkt[s[i] as usize] -= 1;
                sa[bkt[s[i] as usize] as usize] = i as i32;
            }
        }
        induced_sorting(s, sa, &tp, &mut bkt); // sort the LMS-substrings


        // find the lexicographic names of substrings
        // init the name array buffer
        let mut new_s: Vec<i32> = vec![-1; n/2 + 1];

        // for sentinel (i = 0)
        new_s[(sa[0] / 2) as usize] = 0;
        let mut lms_cnt: usize = 1;
        let mut name: usize = 1;
        let mut prev_pos_init: i32 = sa[0];

        for i in 1..n {
            let cur_pos_init = sa[i];
            if cur_pos_init > 0 && is_lms(cur_pos_init as usize, &tp) {
                // d = 0
                let mut cur_pos = (cur_pos_init + 0) as usize;
                let mut prev_pos = (prev_pos_init + 0) as usize;
                let mut diff = s[cur_pos] != s[prev_pos] || tp[cur_pos] != tp[prev_pos];

                if !diff {
                    // d > 0
                    for d in 1..(n as i32) {
                        cur_pos = (cur_pos_init + d) as usize;
                        prev_pos = (prev_pos_init + d) as usize;
                        if s[cur_pos] != s[prev_pos] || tp[cur_pos] != tp[prev_pos] {
                            diff = true;
                            break;
                        } else if is_lms(cur_pos, &tp) || is_lms(prev_pos, &tp) {
                            break;
                        }
                    }
                }

                if diff { name += 1; prev_pos_init = cur_pos_init; }
                new_s[(cur_pos_init / 2) as usize] = (name - 1) as i32;
                lms_cnt += 1;
            }
        }


        // bring names to the front
        let mut j = 0;
        for i in 0..(n/2+1) {
            if new_s[i] >= 0 {
                new_s[j] = new_s[i];
                j += 1;
            }
        }

        // stage 2: solve the reduced problem
        // recurse if names are not yet unique
        if name < lms_cnt {
            sa_is(&new_s[0..lms_cnt], &mut sa[0..lms_cnt], name as i32); 
        } else {
            for i in 0..lms_cnt {
                sa[new_s[i] as usize] = i as i32;
            }
        }

        // stage 3: induce the result for the original problem
        get_buckets(s, &mut bkt, true);
        j = 0;
        for i in 1..n {
            if is_lms(i, &tp) {
                new_s[j] = i as i32;
                j += 1;
            }
        }
        for i in 0..lms_cnt {
            sa[i] = new_s[sa[i] as usize];
        }
        for i in lms_cnt..n {
            sa[i] = -1; 
        }
        for i in (0..lms_cnt).rev() {
            j = sa[i] as usize;
            sa[i] = -1;
            bkt[s[j] as usize] -= 1;
            sa[bkt[s[j] as usize] as usize] = j as i32;
        }

        induced_sorting(s, sa, &tp, &mut bkt);
    }

    #[inline(always)]
    fn is_lms(i: usize, tp: &[bool]) -> bool {
        tp[i] && !tp[i-1]
    }

    fn induced_sorting(s: &[i32], sa: &mut [i32], tp: &[bool], bkt: &mut [i32]) {
        let n = s.len();

        // find starts of buckets
        get_buckets(s, bkt, false);
        for i in 0..n {
            let j = sa[i] - 1;
            if j >= 0 && !tp[j as usize] {
                sa[bkt[s[j as usize] as usize] as usize] = j;
                bkt[s[j as usize] as usize] += 1;
            }
        }

        // find ends of buckets
        get_buckets(s, bkt, true);
        for i in (0..n).rev() {
            let j = sa[i] - 1;
            if j >= 0 && tp[j as usize] {
                bkt[s[j as usize] as usize] -= 1;
                sa[bkt[s[j as usize] as usize] as usize] = j;
            }
        }
    }

    // find a start or end of each bucket
    fn get_buckets(s: &[i32], bkt: &mut [i32], end: bool) {
        let n = s.len();
        let k = bkt.len();
        // clear all buckets
        for i in 0..k {
            bkt[i] = 0;
        }

        for i in 0..n {
            bkt[s[i] as usize] += 1;
        }

        let mut sum = 0 as i32;
        if end {
            for i in 0..k {
                sum += bkt[i];
                bkt[i] = sum;
            }
        } else {
            for i in 0..k {
                sum += bkt[i];
                bkt[i] = sum - bkt[i];
            }
        }
    }

    pub fn build(s: &String) -> Vec<i32> {
        let mut s_int: Vec<i32> = s.chars().map(|c| c as i32).collect();
        s_int.push(0);
        let n = s_int.len();
        let mut sa: Vec<i32> = vec![-1; n];
        sa_is(&s_int, &mut sa, 128);
        sa
    }
}
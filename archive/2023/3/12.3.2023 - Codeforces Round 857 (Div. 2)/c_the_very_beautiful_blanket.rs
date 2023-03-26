//{"name":"C. The Very Beautiful Blanket","group":"Codeforces - Codeforces Round 857 (Div. 2)","url":"https://codeforces.com/contest/1802/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 5\n4 4\n4 6\n6 6\n","output":"25\n9740 1549 9744 1553 9748\n1550 1551 1554 1555 1558\n10252 2061 10256 2065 10260\n2062 2063 2066 2067 2070\n10764 2573 10768 2577 10772\n16\n3108 3109 3112 3113\n3110 3111 3114 3115\n3620 3621 3624 3625\n3622 3623 3626 3627\n24\n548 549 552 553 556 557\n550 551 554 555 558 559\n1060 1061 1064 1065 1068 1069\n1062 1063 1066 1067 1070 1071\n36\n25800 25801 25804 25805 25808 25809\n25802 4294993099 25806 4294993103 25810 4294993107\n26312 26313 26316 26317 26320 26321\n26314 4294993611 26318 4294993615 26322 4294993619\n26824 26825 26828 26829 26832 26833\n26826 4294994123 26830 4294994127 26834 4294994131\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheVeryBeautifulBlanket"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn highest_bit(mut n: usize)->usize {
    let mut count = 0;
    while n > 0 {
        n /= 2;
        count += 1;
    }
    count
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, mut m): (usize, usize) = input.read();
    let mx = n*m;
    let hb = highest_bit(m)+1;
    let hn = 2usize.pow(hb as u32);
    let mut answer = vec![];
    for i in 0..n {
        let mut result = vec![];
        for j in hn*i..hn*i+m {
            result.push(j); 
        }
        answer.push(result);
    }
    out_line!(n*m);
    for result in answer {
        out_line!(result);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
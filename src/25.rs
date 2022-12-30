fn main() {
    println!("Part 1 is {}", part1(input()));
}

fn snafu_to_normal(snafu: &str) -> i64 {
    let mut result = 0;
    for (position, char) in snafu.chars().rev().enumerate() {
        let exp = 5_i64.pow(position as u32);

        result += match char {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        } * exp;
    }

    result
}

fn normal_to_snafu(normal: i64) -> String {
    let mut min_max = Vec::new();
    loop {
        let min = snafu_to_normal(&String::from_utf8(vec![b'='; min_max.len() + 1]).unwrap());
        let max = snafu_to_normal(&String::from_utf8(vec![b'2'; min_max.len() + 1]).unwrap());

        min_max.push((min, max));

        if normal >= min && normal <= max {
            break;
        }
    }

    min_max.pop();

    min_max.reverse();
    min_max.push((0, 0));

    let mut result = String::new();
    let mut remainder = normal;
    'main_loop: for (i, (min, max)) in min_max.iter().enumerate() {
        let exp = 5_i64.pow(min_max.len() as u32 - i as u32 - 1);

        for value in [-2, -1, 0, 1, 2] {
            let new_remainder = remainder - (value * exp);
            if new_remainder >= *min && new_remainder <= *max {
                result.push(match value {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!(),
                });
                remainder = new_remainder;
                continue 'main_loop;
            }
        }

        panic!();
    }

    result
}

fn part1(input: &'static str) -> String {
    let result = input
        .trim()
        .lines()
        .map(|line| snafu_to_normal(line.trim()))
        .sum();

    normal_to_snafu(result)
}

#[test]
fn test() {
    assert_eq!(snafu_to_normal("1=-0-2"), 1747);
    assert_eq!(snafu_to_normal("12111"), 906);
    assert_eq!(snafu_to_normal("2=0="), 198);
    assert_eq!(snafu_to_normal("21"), 11);
    assert_eq!(snafu_to_normal("2=01"), 201);
    assert_eq!(snafu_to_normal("111"), 31);
    assert_eq!(snafu_to_normal("20012"), 1257);
    assert_eq!(snafu_to_normal("112"), 32);
    assert_eq!(snafu_to_normal("1=-1="), 353);
    assert_eq!(snafu_to_normal("1-12"), 107);
    assert_eq!(snafu_to_normal("12"), 7);
    assert_eq!(snafu_to_normal("1="), 3);
    assert_eq!(snafu_to_normal("122"), 37);

    assert_eq!("1=-0-2", &normal_to_snafu(1747));
    assert_eq!("12111", &normal_to_snafu(906));
    assert_eq!("2=0=", &normal_to_snafu(198));
    assert_eq!("21", &normal_to_snafu(11));
    assert_eq!("2=01", &normal_to_snafu(201));
    assert_eq!("111", &normal_to_snafu(31));
    assert_eq!("20012", &normal_to_snafu(1257));
    assert_eq!("112", &normal_to_snafu(32));
    assert_eq!("1=-1=", &normal_to_snafu(353));
    assert_eq!("1-12", &normal_to_snafu(107));
    assert_eq!("12", &normal_to_snafu(7));
    assert_eq!("1=", &normal_to_snafu(3));
    assert_eq!("122", &normal_to_snafu(37));

    assert_eq!(
        "2=-1=0",
        &part1(
            "
            1=-0-2
            12111
            2=0=
            21
            2=01
            111
            20012
            112
            1=-1=
            1-12
            12
            1=
            122

    "
        )
    )
}

fn input() -> &'static str {
    "
    1212-2==
    1=-=0111
    1=22-20=--
    12==001-
    1000=001222212
    1-00=-0=
    1==-=011==-200-1-1
    1=-20-
    1=110-
    1-20221-=00-002-1100
    20-12--=0
    21=202-2
    22
    2=121020110
    1101-12-1=0
    2-1-
    201==1-112
    102=110
    11-212=001-2=20=000
    211-210022021-0
    1-0
    1-2=0
    1==2211=000=-0=-
    102-222-0==-=-2-2=
    1=-=0=1-2=2=0-
    1=--1212
    21020==
    1-20=1=2=10-02=2=--
    11=001
    1-0-1012-10
    10221=0=2020
    12-111211021-0-
    1--=-1=11=212
    100112=21012=10-
    2=
    2-0021=1=2=
    2-1=-==-0=1
    12-=-=2--0
    1=02
    10=11
    1=-110=-==
    10-00=0121=-2-=
    1=1-=2-0011=2
    10011-0
    102102102
    1-0-0-1-2221
    1=00=011=2=0
    1-20==1-0
    12==01-1=--1-
    201=----=-1==-121-
    1--
    102
    21==0-2=020-=02-
    2-1010-=22=20=
    1=2
    222
    21-0-10-2=2-
    2=-==10=11
    1=22=01=-0-1=102
    122==20=1-0012
    10=100210
    1-===22==11=0=-
    20-=-2===2-10
    10--2=
    21=11-01
    1==--1-200
    1001
    11--=210220
    111212201
    1-=-2-=--
    1=1-22022101
    122=10=1
    2-2220--111=
    2-1
    1=1=1210-111=11112
    1-22=-2120
    22=02222-00=01-0=
    20=12
    1=100==11-=211-1=1
    11--011==210-00
    2121=011==-
    2-=002
    2=202-112=-
    1=-2=-0-01011
    102=2-==
    1===2=1=12-01-=-20=
    2-1021--=2
    11221--2
    100--=22101212-110
    21-00==11-2-1
    202--00021001
    10=-1=2121=-1
    1==11-
    21=
    1-0--
    1=1
    1--0-1
    1-2=2
    10-1220-00-11=1
    2200102=2212-02-22
    1-
    10-=01
    1=222=01==0=1
    1101
    2-==10
    2-0=0=1=0110100-1
    1=0=--==0200=0-=0
    121-12==-==-2-
    1==222
    10-22-=21==-2-
    "
}

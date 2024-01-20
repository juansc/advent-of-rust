use nom::bytes::complete::take_while1;
use nom::combinator::map_res;
use nom::IResult;

pub fn parse_number(i: &str) -> IResult<&str, usize> {
    // take_while1 grabs as many numbers as it can
    // map_res takes the result of take_while1 and tries to parse it as a usize
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}
use super::SharedMimeMagicRule;
use crate::db::{MagicRule, OwnedMagicRule};
use nom::{
  self,
  bytes::complete::{is_not, tag, take, take_while},
  character::is_digit,
  character::{self, complete::line_ending, streaming::newline},
  combinator::{map, map_res, opt, rest},
  multi::many0,
  number::complete::be_u16,
  sequence::{delimited, preceded, terminated, tuple},
  IResult,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MagicRuleParseError {}

struct PrioMime<'a> {
  priority: u32,
  mime_type: &'a str,
}
/// Parses the MIME type and priority from "[priority: mime]\r\n"
fn parse_mime<'a>(input: &'a [u8]) -> IResult<&'a [u8], PrioMime<'a>> {
  let (input, between_brackets) =
    delimited(character::complete::char('['), is_not("]"), tag("]"))(input)?;

  let (_, (priority, mime)) = tuple((
    terminated(
      nom::character::complete::u32,
      nom::character::complete::char(':'),
    ),
    map_res(rest, std::str::from_utf8),
  ))(between_brackets)?;

  Ok((
    input,
    PrioMime {
      mime_type: mime,
      priority,
    },
  ))
}

fn parse_magic_match_rule<'a>(
  input: &'a [u8],
  prio_mime: PrioMime<'a>,
) -> IResult<&'a [u8], SharedMimeMagicRule<'a>> {
  todo!()
  // let int_or = |default| {
  //   map(take_while(is_digit), move |digits| {
  //     str::from_utf8(digits).unwrap().parse().unwrap_or(default)
  //   })
  // };

  // let (input, (indent_level, start_off, val_len)) = tuple((
  //   terminated(int_or(0), tag(">")),
  //   terminated(int_or(0), tag("=")),
  //   be_u16,
  // ))(input)?;

  // let (input, (val, mask, word_len, region_len)) = terminated(
  //   tuple((
  //     take(val_len),
  //     opt(preceded(tag("&"), take(val_len))),
  //     opt(preceded(tag("~"), int_or(1))),
  //     opt(preceded(tag("+"), int_or(0))),
  //   )),
  //   tag("\n"),
  // )(input)?;

  // Ok((
  //   input,
  //   SharedMimeMagicRule::new(
  //     prio_mime.priority,
  //     prio_mime.mime_type,
  //     indent_level,
  //     start_off,
  //     val,
  //     mask,
  //     word_len.unwrap_or(1),
  //     region_len.unwrap_or(0),
  //   ),
  // ))
}

/// Converts a magic file given as a &[u8] array
/// to a vector of MagicEntry structs
/// Returns the amount of parsed magic rules
pub fn parse_magic_file1<'a>(
  input: impl Iterator<Item = u8>,
  mime_types: &'a mut Vec<String>,
  magic_rules: &mut Vec<SharedMimeMagicRule<'a>>,
) -> Result<usize, (usize, MagicRuleParseError)> {
  // let magic_entry = tuple((mime_parser, many0(parse_single_magic_rules)));
  todo!()
  // preceded(tag("MIME-Magic\0\n"), many0(magic_entry))(input)
}

/// Converts a magic file given as a &[u8] array
/// to a vector of MagicEntry structs
/// Returns the amount of parsed magic rules
pub fn parse_magic_file(
  input: impl Iterator<Item = u8>,
  magic_rules: &mut Vec<OwnedMagicRule>,
) -> Result<usize, (usize, MagicRuleParseError)> {
  // // Parse the MIME type from "[priority: mime]"
  // let mime_parser = map_res(
  //   terminated(
  //     delimited(
  //       delimited(tag("["), is_not(":"), tag(":")), // priority
  //       is_not("]"),                                // mime
  //       tag("]"),
  //     ),
  //     tag("\n"),
  //   ),
  //   std::str::from_utf8,
  // );

  // let magic_entry = tuple((mime_parser, many0(parse_single_magic_rules)));
  todo!()
  // preceded(tag("MIME-Magic\0\n"), many0(magic_entry))(input)
}

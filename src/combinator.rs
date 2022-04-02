use crate::parser::{Parser, PError, PResult};
use core::fmt::Debug;

macro_rules! succ (
    (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
    (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
    (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
    (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
    (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
    (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
    (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
    (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
    (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
    (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
    (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
    (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
    (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
    (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
    (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
    (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
    (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
    (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
    (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
    (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
    (20, $submac:ident ! ($($rest:tt)*)) => ($submac!(21, $($rest)*));
  );

pub trait Seq<'a, Error:Clone> {
    fn parse(&mut self, input: &'a str) -> PResult<&'a str, Vec<&'a str>, Error>;
}

macro_rules! seq_trait(
    ($first:ident $second:ident $($id: ident)+) => (
      seq_trait!(__impl $first $second; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
      seq_trait_impl!($($current)*);
  
      seq_trait!(__impl $($current)* $head; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident) => (
      seq_trait_impl!($($current)*);
      seq_trait_impl!($($current)* $head);
    );
  );
  
macro_rules! seq_trait_impl(
    ($($id:ident)+) => (
      impl<'a, Error:Clone+Debug, 
        $($id: Parser<&'a str, &'a str, Error>),+
      > Seq<'a, Error> for ( $($id),+ ) {
        fn parse(&mut self, input: &'a str) ->  PResult <&'a str, Vec<&'a str>, Error> {
            let mut result = vec![];
            let (rem, res) = self.0.run(input)?;
            if res.len() > 0 { result.push(res)}
            seq_trait_inner!(1, self, rem, result, $($id)+)
        }
      }
    );
  );
  
  macro_rules! seq_trait_inner(
    ($it:tt, $self:expr, $rem:expr, $result:expr, $head:ident $($id:ident)+) => (
        match $self.$it.run($rem) {
            Ok((rem, res)) => {
                if res.len() > 0 {$result.push(res)};
                succ!($it, seq_trait_inner!($self, rem,  $result, $($id)+))
            }
            Err(e) => return Err(e)
        }
    );
    ($it:tt, $self:expr, $rem:expr, $result:expr, $head:ident) => (
        return Ok(($rem, $result))
    );
  );
  
  seq_trait!(A B C D E F G H I J K L M N O P Q R S T U);
  
pub trait SeqJoined<'a, Error:Clone+Debug> {
    fn parse(&mut self, input: &'a str) -> PResult<&'a str, &'a str, Error>;
}

macro_rules! seq_joined_trait(
    ($first:ident $second:ident $($id: ident)+) => (
      seq_joined_trait!(__impl $first $second; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
      seq_joined_trait_impl!($($current)*);
  
      seq_joined_trait!(__impl $($current)* $head; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident) => (
      seq_joined_trait_impl!($($current)*);
      seq_joined_trait_impl!($($current)* $head);
    );
  );
  
macro_rules! seq_joined_trait_impl(
    ($($id:ident)+) => (
      impl<'a, Error:Clone+Debug,
        $($id: Parser<&'a str, &'a str, Error>),+
      > SeqJoined<'a,Error> for ( $($id),+ ) {
        fn parse(&mut self, input: &'a str) ->  PResult <&'a str, &'a str, Error> {
            let (rem, _) = self.0.run(input)?;
            seq_joined_trait_inner!(1, self, rem, input, $($id)+)
        }
      }
    );
  );
  
  macro_rules! seq_joined_trait_inner(
    ($it:tt, $self:expr, $rem:expr, $input:expr, $head:ident $($id:ident)+) => (
      match $self.$it.run($rem) {
        Ok((rem, _)) => succ!($it, seq_joined_trait_inner!($self, rem,  $input, $($id)+)),
        Err(e) => return Err(e)
      }
    );
    ($it:tt, $self:expr, $rem:expr, $input:expr, $head:ident) => (
        return Ok(($rem, &$input[..$input.len() - $rem.len()]))
    );
  );
  
seq_joined_trait!(A B C D E F G H I J K L M N O P Q R S T U);

pub trait SeqNamed<'a, T:Clone, Error:Clone+Debug> {
    fn parse(&mut self, input: &'a str) -> PResult<&'a str, Vec<(T, &'a str)>, Error>;
}

macro_rules! seq_named_trait(
    ($first:ident $second:ident $($id:ident)+) => (
      seq_named_trait!(__impl $first $second; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident $($id:ident)+) => (
      seq_named_trait_impl!($($current)*);
      seq_named_trait!(__impl $($current)* $head; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident) => (
      seq_named_trait_impl!($($current)*);
      seq_named_trait_impl!($($current)* $head);
    );
  );
  
macro_rules! seq_named_trait_impl(
    ($($id:ident)+) => (
      impl<'a, Name: Clone, Error:Clone+Debug,
        $($id: Parser<&'a str, &'a str, Error>),+
      > SeqNamed<'a,Name,Error> for ( $((Name,$id)),+ ) {
        fn parse(&mut self, input: &'a str) ->  PResult <&'a str, Vec<(Name,&'a str)>, Error> {
            let mut result = vec![];
            let (rem, res) = self.0.1.run(input)?;
            if res.len() > 0 { result.push((self.0.0.clone(), res))}
            seq_named_trait_inner!(1, self, rem, result, $($id)+)
        }
      }
    );
  );
  
  macro_rules! seq_named_trait_inner(
    ($it:tt, $self:expr, $rem:expr, $result:expr, $head:ident $($id:ident)+) => (
      match $self.$it.1.run($rem) {
        Ok((rem, res)) => {
            if res.len() > 0 {$result.push(($self.$it.0.clone(), res))};
            succ!($it, seq_named_trait_inner!($self, rem, $result, $($id)+))
        }
        Err(e) => return Err(e)
      }
    );
    ($it:tt, $self:expr, $rem:expr, $result:expr, $head:ident) => (
        return Ok(($rem, $result))
    );
  );
  
seq_named_trait!(A B C D E F G H I J K L M N O P Q R S T U);

pub trait Alt<'a,Error:Clone+Debug> {
    fn parse(&mut self, input: &'a str) -> PResult<&'a str, &'a str, Error>;
}

macro_rules! alt_trait(
    ($first:ident $second:ident $($id: ident)+) => (
      alt_trait!(__impl $first $second; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
      alt_trait_impl!($($current)*);
  
      alt_trait!(__impl $($current)* $head; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident) => (
      alt_trait_impl!($($current)*);
      alt_trait_impl!($($current)* $head);
    );
  );
  
macro_rules! alt_trait_impl(
    ($($id:ident)+) => (
      impl<'a, Error:Clone+Debug,
        $($id: Parser<&'a str, &'a str, Error>),+
      > Alt<'a,Error> for ( $($id),+ ) {
        fn parse(&mut self, input: &'a str) ->  PResult <&'a str, &'a str, Error> {
            match self.0.run(input) {
                Ok((rem, res)) => return Ok((rem, res)),
                Err(_e) => alt_trait_inner!(1, self, input, $($id)+, _e)
            }
        }
      }
    );
  );
  
  macro_rules! alt_trait_inner(
    ($it:tt, $self:expr, $rem:expr, $head:ident $($id:ident)+, $err:expr) => (
      match $self.$it.run($rem) {
        Ok((rem, res)) =>  return Ok((rem, res)),
        Err(_e) => succ!($it, alt_trait_inner!($self, $rem, $($id)+, _e)),
      }
    );
    ($it:tt, $self:expr, $rem:expr, $head:ident, $err:expr) => (
        Err($err)
    );
  );
  
  alt_trait!(A B C D E F G H I J K L M N O P Q R S T U);

pub trait AltNamed<'a,T:Clone,Error:Clone+Debug> {
    fn parse(&mut self, input: &'a str) -> PResult<&'a str, (T,&'a str), Error>;
}

macro_rules! alt_named_trait(
    ($first:ident $second:ident $($id: ident)+) => (
      alt_named_trait!(__impl $first $second; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
      alt_named_trait_impl!($($current)*);
  
      alt_named_trait!(__impl $($current)* $head; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident) => (
      alt_named_trait_impl!($($current)*);
      alt_named_trait_impl!($($current)* $head);
    );
  );
  
macro_rules! alt_named_trait_impl(
    ($($id:ident)+) => (
      impl<'a, Name:Clone, Error:Clone+Debug,
        $($id: Parser<&'a str, &'a str, Error>),+
      > AltNamed<'a,Name,Error> for ( $((Name,$id)),+ ) {
        fn parse(&mut self, input: &'a str) ->  PResult <&'a str, (Name,&'a str), Error> {
          match self.0.1.run(input) {
           Ok((rem, res)) => return Ok((rem, (self.0.0.clone(), res))),
           Err(_e) => alt_named_trait_inner!(1, self, input, $($id)+, _e)
          }
        }
      }
    );
  );
  
  macro_rules! alt_named_trait_inner(
    ($it:tt, $self:expr, $rem:expr, $head:ident $($id:ident)+, $err:expr) => (
      match $self.$it.1.run($rem) {
        Ok((rem, res)) =>  return Ok((rem, ($self.$it.0.clone(), res))),
        Err(_e) => succ!($it, alt_named_trait_inner!($self, $rem, $($id)+, _e)),
      }
    );
    ($it:tt, $self:expr, $rem:expr, $head:ident, $err:expr) => (
        Err($err)
    );
  );
  
  alt_named_trait!(A B C D E F G H I J K L M N O P Q R S T U);
pub trait Pipe<'a,Error:Clone+Debug> {
    fn parse(&mut self, input: &'a str) -> PResult<&'a str, &'a str, Error>;
}

macro_rules! pipe_trait(
    ($first:ident $second:ident $($id: ident)+) => (
      pipe_trait!(__impl $first $second; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
      pipe_trait_impl!($($current)*);
  
      pipe_trait!(__impl $($current)* $head; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident) => (
      pipe_trait_impl!($($current)*);
      pipe_trait_impl!($($current)* $head);
    );
  );
  
macro_rules! pipe_trait_impl(
    ($($id:ident)+) => (
      impl<'a,Error:Clone+Debug,
        $($id: Parser<&'a str, &'a str, Error>),+
      > Pipe<'a,Error> for ( $($id),+ ) {
        fn parse(&mut self, input: &'a str) ->  PResult <&'a str, &'a str, Error> {
            let (rem, _res) = self.0.run(input)?;
            pipe_trait_inner!(1, self, rem, _res, $($id)+)
        }
      }
    );
  );

  macro_rules! pipe_trait_inner(
    ($it:tt, $self:expr, $rem:expr, $result:expr, $head:ident $($id:ident)+) => (
      match $self.$it.run($rem) {
        Ok((rem, _res)) => {
            succ!($it, pipe_trait_inner!($self, rem,  _res, $($id)+))
        }
        Err(e) => return Err(e)
      }
    );
    ($it:tt, $self:expr, $rem:expr, $result:expr, $head:ident) => (
        return Ok(($rem, $result))
    );
  );
pipe_trait!(A B C D E F G H I J K L M N O P Q R S T U);

pub fn sequence_of<'a, Error:Clone+Debug, List: Seq<'a, Error>>(mut l: List,) -> impl Parser<&'a str, Vec<&'a str>, Error> {
    move |input: &'a str| l.parse(input)
}

pub fn sequence_of_joined<'a, Error:Clone+Debug,List: SeqJoined<'a,Error>>(mut l: List,) -> impl Parser<&'a str, &'a str, Error> {
    move |input: &'a str| l.parse(input)
}

pub fn named_sequence_of<'a, Name:Clone, Error:Clone+Debug, List: SeqNamed<'a,Name,Error>>(mut l: List,)
 -> impl Parser<&'a str, Vec<(Name,&'a str)>, Error>
{
    move |input: &'a str| l.parse(input)
}

pub fn choice<'a,Error:Clone+Debug,List: Alt<'a,Error>>(mut l: List,)
 -> impl Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| l.parse(input)
}

pub fn named_choice<'a,T:Clone,Error:Clone+Debug,List:AltNamed<'a,T,Error>>(mut l: List,)
 -> impl Parser<&'a str, (T, &'a str), Error>
{
    move |input: &'a str| l.parse(input)
}

pub fn pipe_parsers<'a,Error:Clone+Debug,List:Pipe<'a,Error>>(mut l: List,)
 -> impl Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| l.parse(input)
}

pub fn look_ahead<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, &'a str, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let (_, res) = parser.run(input)?;
        Ok((input, res))
    }
}

pub fn sep_by<'a,Error,P,S>(mut parser: P, mut seperator: S) -> impl Parser<&'a str, Vec<&'a str>, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>,
 S: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result = vec![];
        while remainder.len() > 0 {
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder =  rem;
                    if res.len() > 0 { result.push(res); }
                },
                Err(_) => return Ok((remainder, result))
            }
            match seperator.run(remainder) {
                Ok((rem, _)) => remainder =  rem,
                Err(_) => return Ok((remainder, result))
            }
        }
        Ok((remainder, result))
    }
}

pub fn named_sep_by<'a,T,Error,P,S>(name: T, mut parser: P, mut seperator: S) 
    -> impl Parser<&'a str, Vec<(T, &'a str)>, Error> where
 T: Clone,
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>,
 S: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result  = vec![];
        while remainder.len() > 0 {
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder =  rem;
                    if res.len() > 0 { result.push((name.clone(), res)); }
                },
                Err(_) => return Ok((remainder, result))
            }
            match seperator.run(remainder) {
                Ok((rem, _)) => remainder =  rem,
                Err(_) => return Ok((remainder, result))
            }
        }
        Ok((remainder, result))
    }
}

pub fn sep_by1<'a,Error,P,S>(mut parser: P, mut seperator: S) -> impl Parser<&'a str, Vec<&'a str>, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>,
 S: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result = vec![];
        let (rem, res) = parser.run(remainder)?;
        remainder = rem;
        if res.len() > 0 { result.push(res); }
        while remainder.len() > 0 {
            match seperator.run(remainder) {
                Ok((rem, _)) => remainder =  rem,
                Err(_) => return Ok((remainder, result))
            }
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder =  rem;
                    if res.len() > 0 { result.push(res); }
                },
                Err(_) => return Ok((remainder, result))
            }
        }
        Ok((remainder, result))
    }
}

pub fn named_sep_by1<'a,T,Error,P,S>(name: T, mut parser: P, mut seperator: S)
 -> impl Parser<&'a str, Vec<(T, &'a str)>, Error> where 
 T: Clone,
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>,
 S: Parser<&'a str, &'a str, Error>,
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result = vec![];
        let (rem, res) = parser.run(remainder)?;
        remainder = rem;
        if res.len() > 0 { result.push((name.clone(), res)); }
        while remainder.len() > 0 {
            match seperator.run(remainder) {
                Ok((rem, _)) => remainder =  rem,
                Err(_) => return Ok((remainder, result))
            }
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder =  rem;
                    if res.len() > 0 { result.push((name.clone(), res)); }
                },
                Err(_) => return Ok((remainder, result))
            }
        }
        Ok((remainder, result))
    }
}

pub fn exactly<'a,Error,P>(times: usize, mut parser: P) -> impl Parser<&'a str, Vec<&'a str>, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut output : Vec<&str> = vec![];
        for _ in  0..times {
            let (rem, res) = parser.run(remainder)?;
            remainder = rem;
            if res.len() > 0 {output.push(res);}
        }
        Ok((remainder, output))
    }
}

pub fn named_exactly<'a,T,Error,P>(times: usize, name: T, mut parser: P) -> impl Parser<&'a str, Vec<(T, &'a str)>, Error> where
 T: Clone,
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut output = vec![];
        for _ in  0..times {
            let (rem, res) = parser.run(remainder)?;
            remainder = rem;
            if res.len() > 0 {output.push((name.clone(), res));}
        }
        Ok((remainder, output))
    }
}

pub fn many_joined<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, &'a str, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        loop {
            match parser.run(remainder) {
                Ok((rem, _)) => remainder = rem,
                Err(_) => return Ok((remainder, &input[..input.len() - remainder.len()]))
            } 
        }
    }
}

pub fn many1_joined<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, &'a str, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let (rem, _) = parser.run(remainder)?;
        remainder = rem;
        loop {
            match parser.run(remainder) {
                Ok((rem, _)) => remainder = rem,
                Err(_) => return Ok((remainder, &input[..input.len() - remainder.len()]))
            } 
        }
    }
}

pub fn named_many<'a,T,Error,P>(mut parser: P)
 -> impl Parser<&'a str, Vec<(T, &'a str)>, Error> where 
    T: Clone,
    Error: Clone + Debug,
    P: Parser<&'a str, (T, &'a str), Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut output : Vec<(T, &str)> = vec![];
        loop {
            match parser.run(remainder) {
                Ok((rem, (name, res))) => {
                    remainder = rem;
                    if res.len() > 0 {output.push((name.clone(), res));}
                }
                Err(_) => return Ok((remainder, output))
            } 
        }
    }
}

pub fn named_many1<'a,T,Error,P>(mut parser: P)
 -> impl Parser<&'a str, Vec<(T, &'a str)>, Error> where
    T: Clone,
    Error: Clone + Debug,
    P: Parser<&'a str, (T, &'a str), Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut output : Vec<(T, &str)> = vec![];
        let (rem, (name, res)) = parser.run(remainder)?;
        remainder = rem;
        if res.len() > 0 {output.push((name.clone(), res));}
        loop {
            match parser.run(remainder) {
                Ok((rem, (name, res))) => {
                    remainder = rem;
                    if res.len() > 0 {output.push((name.clone(), res));}
                }
                Err(_) => return Ok((remainder, output))
            } 
        }
    }
}

pub fn many<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, Vec<&'a str>, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut output : Vec<&str> = vec![];
        loop {
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder = rem;
                    if res.len() > 0 { output.push(res); }
                }
                Err(_) => return Ok((remainder, output))
            } 
        }
    }
}

pub fn many1<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, Vec<&'a str>, Error> where
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut output : Vec<&str> = vec![];
        let (rem, res) = parser.run(remainder)?;
        remainder = rem;
        if res.len() > 0 {output.push(res);}
        loop {
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder = rem;
                    if res.len() > 0 {output.push(res);}
                }
                Err(_) => return Ok((remainder, output))
            } 
        }
    }
}

pub fn between<'a,Error,L,R,P>(mut left_delim: L, mut right_delim: R, mut parser: P)
 -> impl Parser<&'a str, Vec<&'a str>, Error> where
 Error: Clone + Debug,
 L: Parser<&'a str, &'a str, Error>,
 R: Parser<&'a str, &'a str, Error>,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result: Vec<&'a str> = vec![];
        let (rem, _) = left_delim.run(remainder)?;
        remainder = rem;
        while remainder.len() > 0 {
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder =  rem;
                    if res.len() > 0 {result.push(res);}
                },
                Err(_) => {
                    let (rem, _) = right_delim.run(remainder)?;
                    return Ok((rem, result))
                }
            }
        }
        let (rem, _) = right_delim.run(remainder)?;
        Ok((rem, result))
    }
}

pub fn named_between<'a,T,Error,L,R,P>(name: T, mut delim_left:L, mut delim_right:R, mut parser: P)
 -> impl Parser<&'a str, Vec<(T, &'a str)>, Error> where 
 T: Clone,
 Error: Clone + Debug,
 L: Parser<&'a str, &'a str, Error>,
 R: Parser<&'a str, &'a str, Error>,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result= vec![];
        let (rem, _) =  delim_left.run(remainder)?; 
        remainder = rem;
        while remainder.len() > 0 {
            match parser.run(remainder) {
                Ok((rem, res)) => {
                    remainder =  rem;
                    if res.len() > 0 {result.push((name.clone(), res));}
                },
                Err(_) => {
                    let (rem, _) = delim_right.run(remainder)?;
                    return Ok((rem, result))
                }
            }
        }
        delim_right.run(remainder)?;
        Ok((rem, result))
    }
}

pub fn every_char_until<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, Vec<&'a str>, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result : Vec<&str> = vec![];
        for (i, _) in input.chars().enumerate() {
            match parser.run(&input[i..i+1]) {
                Err(_) => {
                    result.push(&input[i..i+1]);
                    remainder = &input[i+1..];
                },
                Ok(_) => return Ok((remainder, result))
            }
        }
        Ok((remainder, result))
    }
}

pub fn named_every_char_until<'a,T,Error,P>(name: T, mut parser: P)
 -> impl Parser<&'a str, Vec<(T, &'a str)>, Error> where
 T: Clone,
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let mut remainder = input;
        let mut result = vec![];
        for (i, _) in input.chars().enumerate() {
            match parser.run(&input[i..i+1]) {
                Err(_) => {
                    result.push((name.clone(), &input[i..i+1]));
                    remainder = &input[i+1..];
                },
                Ok(_) => return Ok((remainder, result))
            }
        }
        Ok((remainder, result))
    }
}

pub fn any_char_except<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, &'a str, PError> where 
 P: Parser<&'a str, &'a str, Error> 
{
    move |input: &'a str| {
        match parser.run(&input[..1]){
            Ok(_) => return Err(PError::UnexpectedInput),
            Err(_) => return Ok((&input[1..], &input[..1]))
        }
    }
}


pub fn possibly<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, &'a str, Error> where
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{   
    move |input: &'a str| {
        if input.len() == 0 { return Ok((input, "")); }
        match parser.run(input) {
            Ok((rem, res)) => Ok((rem, res)),
            Err(_) => Ok((input, ""))
        }
    }
}

pub fn skip<'a,Error,P>(mut parser: P) -> impl Parser<&'a str, &'a str, Error>  where 
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let (rem, _) = parser.run(input)?;
        Ok((rem, ""))
    }
}

pub fn name<'a,T,Error,P>(name: T, mut parser: P) 
    -> impl Parser<&'a str, (T, &'a str), Error> where 
 T: Clone,
 Error: Clone + Debug,
 P: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let (rem, res) = parser.run(input)?;
        Ok((rem, (name.clone(), res)))
    }
}

pub fn take_right<'a,Error,L,R>(mut left: L, mut right: R) -> impl Parser<&'a str, &'a str, Error> where
 Error: Clone + Debug,
 L: Parser<&'a str, &'a str, Error>,
 R: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let (rem, _) = left.run(input)?;
        let (rem, res) = right.run(rem)?;
        Ok((rem, res))
    }
}

pub fn take_left<'a,Error,L,R>(mut left: L, mut right: R) -> impl Parser<&'a str, &'a str, Error> where
 Error: Clone + Debug,
 L: Parser<&'a str, &'a str, Error>,
 R: Parser<&'a str, &'a str, Error>
{
    move |input: &'a str| {
        let (rem, res) =  left.run(input)?; 
        let (rem, _) = right.run(rem)?;
        Ok((rem, res))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{string, digit, digits, letter, letters, char, whitespace};

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    #[repr(u8)]
    enum Test {
        Peter,
        Hure
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    #[repr(u8)]
    pub enum ParseCodes {
        Number,
        Operator,
        None
    }
    fn parse_operator<'a>(input: &'a str) -> PResult <&'a str, &'a str, PError> {
        choice((char('+'),
            char('-'),
            char('/'),
            char('*'),
            char('('), 
            char(')'))).run(input) 
    }
    fn parse_number<'a>(input: &'a str) -> PResult <&'a str, &'a str, PError> {
        sequence_of_joined((digits(10), possibly(char('.')), possibly(digits(10)))).run(input)  
    }

    fn no_whitespace<'a>(input: &'a str) -> PResult <&'a str, &'a str, PError> {
        skip(whitespace()).run(input)
    }

    fn calc_parser<'a>(input: &'a str) -> PResult <&'a str, Vec<(ParseCodes, &'a str)>, PError> {
        named_many(named_choice((
                (ParseCodes::Number, parse_number),
                (ParseCodes::Operator, parse_operator),
                (ParseCodes::None, no_whitespace)))).run(input)
    }

    #[test]
    fn test2() {
        let ret = calc_parser.run("10.2111/20+3-10.234").unwrap();

        for el in ret.1.iter() {
            println!("{:?} : {:?}", el.0, el.1)
        }
        
    }

    #[test]
    fn parse_test() {
        test_parser_success(name(Test::Hure, take_right(string("0x"), digits(16u32))),
        "0x10F2C", "", (Test::Hure, "10F2C"));

        test_parser_success(name(Test::Hure, pipe_parsers((string("0x"), digits(16u32)))),
        "0x10F2C", "", (Test::Hure, "10F2C"));

        test_parser_success(named_choice(((Test::Hure, string("hure")), (Test::Peter, string("peter")))), "hure",
        "", (Test::Hure, "hure"));

        let mut hex_parser = 
            named_sequence_of((("trash", string("0x")), ("num", digits(16u32))));
        let ret = hex_parser.run("0x10F2C").unwrap();
        
        for el in ret.1.iter() {
            println!("{} : {}", el.0, el.1)
        }
        
        test_parser_success(take_right(string("0x"), digits(16u32)), "0x10F2C",
            "", "10F2C");
        test_parser_success(choice((letter(), digit(10))), "5abv", "abv", "5");
        test_parser_success(choice((letter(), digit(10))), "a5bv", "5bv", "a");
        test_parser_failure(choice((letter(), digit(10))), "+");

        test_parser_success( sequence_of((
                                string ("hello "),
                                look_ahead(string ("world")),
                                string ("wor")
                            )), 
                            "hello world", "ld", vec!["hello ", "world", "wor"]);
        test_parser_success(sep_by(letters(), char(',')), "some,comma,seperated,words", 
            "", vec!["some", "comma", "seperated", "words"]);
        test_parser_success(sep_by(char(','), letters()), "12345", 
            "12345", vec![]);
        
        test_parser_failure(sep_by1(letters(), char(',')), "1,2,3,4");
        test_parser_success(sep_by1(letters(), char(',')),
         "a,2,3,4", "2,3,4", vec!["a"]);
        
        test_parser_success(exactly(4, letter()), "abcdef", "ef", vec!["a", "b", "c", "d"]);
        test_parser_failure(exactly(4, letter()), "abc");

        test_parser_success(many(string("abc")), "abcabcabcabc", "", vec!["abc", "abc", "abc", "abc"]);
        test_parser_success(many(string("abc")), "", "", vec![]);
        test_parser_success(many(string("abc")), "12345", "12345", vec![]);

        test_parser_success(between(string("<"), string(">"), choice((
            string("0x"), digit(16)))), "<0x10F2C>",
         "", vec!["0x", "1", "0", "F","2", "C"]);
        test_parser_failure(between(char('('), char(')'), letter()), "(hello world)");

        test_parser_success(every_char_until(digit(10)), "abc9", "9", vec!["a", "b", "c"]);

        test_parser_success(any_char_except(digit(10)), "abc9", "bc9", "a");
        
    }

    fn test_parser_success<I,O,E>(mut parser: impl Parser<I,O,E>,input: I, eq_rem: I, eq_parsed: O) where 
    E: Debug,
    I: Debug + PartialEq,
    O: Debug + PartialEq 
{
    let ret = parser.run(input);
    assert!(ret.is_ok());
    let (remainder, parsed) = ret.unwrap();
    assert_eq!(remainder, eq_rem);
    assert_eq!(parsed, eq_parsed);
}

fn test_parser_failure<I,O,E>(mut parser: impl Parser<I,O,E>, input: I) where 
    E: Debug,
    I: Debug,
    O: Debug 
{
    let res = parser.run(input);
    assert!(res.is_err());
    println!("{:?}", res.unwrap_err());
}       
}
       
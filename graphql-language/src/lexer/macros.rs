#[macro_export]
macro_rules! punctuator {
    ($lexer: ident, $p: ident) => ({
      $lexer.iter.next();
      Ok(Token::Punctuator(Punctuator::$p, $lexer.lo, $lexer.lo + 1))
    })
}

// Advance iter and hi position if pattern match
#[macro_export]
macro_rules! take {
  ($lexer: ident, $($p: pat)|*) => (
    match $lexer.iter.peek() {
        $(
          Some(&(i, $p)) => {
            $lexer.iter.next();
            $lexer.hi = i + 1;
            true
          }
        )*
        _ => false
    });
}

#[macro_export]
macro_rules! take_while {
  ($lexer: ident, $($p: pat)|*) => ({
    let mut is_taken = false;
    while let Some(&(p, c)) = $lexer.iter.peek() {
      match c {
        $(
          $p => {
            $lexer.iter.next();
            $lexer.hi = p + 1;
            is_taken = true;
          }
        )*
        _ => {
          break;
        }
      }
    }
    is_taken
  });
}

#[macro_export]
macro_rules! take_while_not {
  ($lexer: ident, $($p: pat)|*) => ({
    let mut is_taken = false;
    while let Some(&(p, c)) = $lexer.iter.peek() {
      match c {
        $(
          $p => {
            $lexer.hi = p;
            break;
          }
        )*
        _ => {
            $lexer.iter.next();
            is_taken = true;
        }
      }
    }
    is_taken
  });
}

#[macro_export]
macro_rules! peek {
  ($lexer: ident, $($p: pat)|*) => (
    match $lexer.iter.peek() {
        $(
          Some(&(_, $p)) => true,
        )*
        _ => false
    });
}

// Advance iter if pattern match, keep hi position unchange
#[macro_export]
macro_rules! follow_by {
  ($lexer: ident, $($p: pat)|*) => (
    match $lexer.iter.peek() {
        $(
          Some(&(_, $p)) => {
            $lexer.iter.next();
            true
          }
        )*
        _ => false
    });
}

mod token;

#[cfg(test)]
mod tests;

use crate::{error::Error, location::Location};
pub use token::*;

// Check if we should merge tokens
static MERGE_TOKEN_CASES: &[(TokenValue, TokenValue, TokenValue)] = &[
    (TokenValue::Plus, TokenValue::Assign, TokenValue::PlusAssign),
    (TokenValue::Minus, TokenValue::Assign, TokenValue::SubAssign),
    (
        TokenValue::Asterisk,
        TokenValue::Assign,
        TokenValue::MulAssign,
    ),
    (TokenValue::Slash, TokenValue::Assign, TokenValue::DivAssign),
    (
        TokenValue::Percent,
        TokenValue::Assign,
        TokenValue::ModAssign,
    ),
    (
        TokenValue::Ampersand,
        TokenValue::Assign,
        TokenValue::AndAssign,
    ),
    (TokenValue::Pipe, TokenValue::Assign, TokenValue::PipeAssign),
    (
        TokenValue::Carrot,
        TokenValue::Assign,
        TokenValue::CarrotAssign,
    ),
    (TokenValue::Plus, TokenValue::Plus, TokenValue::PlusPlus),
    (TokenValue::Minus, TokenValue::Minus, TokenValue::MinusMinus),
    (
        TokenValue::ExclamationMark,
        TokenValue::ExclamationMark,
        TokenValue::NotNot,
    ),
    (TokenValue::Pipe, TokenValue::Pipe, TokenValue::Or),
    (
        TokenValue::Ampersand,
        TokenValue::Ampersand,
        TokenValue::And,
    ),
    (
        TokenValue::LChevron,
        TokenValue::Assign,
        TokenValue::LessThanEqual,
    ),
    (
        TokenValue::RChevron,
        TokenValue::Assign,
        TokenValue::GreaterThanEqual,
    ),
    (TokenValue::Assign, TokenValue::Assign, TokenValue::Equal),
    (
        TokenValue::ExclamationMark,
        TokenValue::Assign,
        TokenValue::NotEqual,
    ),
    (
        TokenValue::LChevron,
        TokenValue::LChevron,
        TokenValue::LShift,
    ),
    (
        TokenValue::RChevron,
        TokenValue::RChevron,
        TokenValue::RShift,
    ),
];

#[derive(Debug, PartialEq, Clone)]
enum State {
    String { s: String, location: Location },
    Comment { s: String, location: Location },
    Identifier { s: String, location: Location },
}

#[cfg(test)]
pub fn lex_str(input: &str) -> Result<Vec<Token>, Error> {
    lex(input, (0, 0).into())
}

/// Lexes the given string into a set of tokens.
pub fn lex(input: &str, mut working_location: Location) -> Result<Vec<Token>, Error> {
    let input = input.replace("\r\n", "\n");
    let mut tokens = Vec::new();
    let mut stack: Vec<State> = Vec::new();

    for c in input.chars() {
        match c {
            '"' => handle_quote(&mut stack, &mut tokens, c, &working_location)?,
            '#' => handle_comment(&mut stack, &mut tokens, c, &working_location)?,
            _ => handle_character(&mut stack, &mut tokens, c, &working_location)?,
        }

        if c == '\n' {
            working_location.increment_line();
        } else {
            working_location.increment_column();
        }
    }

    while let Some(state) = stack.pop() {
        handle_remaining_stack(state, &mut tokens, working_location.clone())?;
    }

    let tokens: Vec<Token> = tokens.iter_mut().map(|t| t.finalize()).collect();

    Ok(merge_tokens(tokens)?)
}

fn merge_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
    let mut final_tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        let next_index = i + 1;

        let mut merged_tokens = false;
        if let TokenValue::Comment(comments) = &token.value {
            let mut comments = comments.clone();
            let start_location = token.start_location.clone();
            let mut end_location = token.end_location.clone();
            let mut next_index = i + 1;
            let mut keep_going = true;

            let mut last_token = token.clone();

            while keep_going && next_index < tokens.len() {
                let next_token = &tokens[next_index];
                if let TokenValue::Comment(next_comments) = &next_token.value {
                    let should_extend = last_token.is_subsequent_line(&next_token)
                        && last_token.start_location.column() == next_token.start_location.column();

                    if should_extend {
                        merged_tokens = true;
                        for comment in next_comments {
                            comments.push(comment.clone());
                        }
                        end_location = next_token.end_location.clone();
                        last_token = next_token.clone();
                    } else {
                        keep_going = false;
                    }

                    next_index += 1;
                } else {
                    keep_going = false;
                }
            }

            if merged_tokens {
                final_tokens.push(Token::comments(comments, start_location, end_location));
                i = next_index;
                continue;
            }
        }

        if !merged_tokens && next_index < tokens.len() && token.is_back_to_back(&tokens[next_index])
        {
            let next_token = &tokens[next_index];

            // Attempt to merge tokens
            for (token_value, next_token_value, merged_token_value) in MERGE_TOKEN_CASES {
                if token.value == *token_value && next_token.value == *next_token_value {
                    final_tokens.push(Token {
                        value: merged_token_value.clone(),
                        start_location: token.start_location.clone(),
                        end_location: next_token.end_location.clone(),
                    });
                    i += 2;
                    merged_tokens = true;
                    continue;
                }
            }

            // Attempt to make a number
            if !merged_tokens {
                if let TokenValue::Number(n) = token.value {
                    let start_location = token.start_location.clone();
                    let mut end_location = token.end_location.clone();
                    let mut working_string = n.to_string();

                    // Merge next if it's a period
                    if next_token.value == TokenValue::Period && token.is_back_to_back(&next_token)
                    {
                        end_location = next_token.end_location.clone();
                        working_string.push('.');
                        merged_tokens = true;

                        // Check for third number
                        let third_index = next_index + 1;
                        if third_index < tokens.len()
                            && next_token.is_back_to_back(&tokens[third_index])
                        {
                            let third_token = &tokens[third_index];
                            if let TokenValue::Number(n) = third_token.value {
                                working_string.push_str(&n.to_string());
                                end_location = third_token.end_location.clone();
                                i += 3;
                            }
                        } else {
                            i += 2;
                        }
                    }

                    if merged_tokens {
                        let token = Token {
                            value: TokenValue::Number(working_string.parse::<f64>().unwrap()),
                            start_location,
                            end_location,
                        };

                        let next_index = i;
                        if next_index < tokens.len() && token.is_back_to_back(&tokens[next_index]) {
                            let next = &tokens[next_index];
                            if let TokenValue::Period = next.value {
                                return Err(Error {
                                    message: "Cannot have multiple periods in a number".to_string(),
                                    location: next.start_location.clone(),
                                });
                            }
                        }

                        final_tokens.push(token);
                        continue;
                    }
                }
            }
        }
        if !merged_tokens {
            final_tokens.push(token.clone());
            i += 1;
        }
    }

    Ok(final_tokens)
}

fn handle_quote(
    stack: &mut Vec<State>,
    tokens: &mut Vec<Token>,
    c: char,
    working_location: &Location,
) -> Result<(), Error> {
    match stack.pop() {
        Some(State::String { mut s, location }) => {
            let last_char_is_escape = {
                if s.is_empty() {
                    false
                } else {
                    s.chars().last().unwrap() == '\\'
                }
            };

            if last_char_is_escape {
                s.pop();
                s.push(c);
                stack.push(State::String { s, location });
            } else {
                tokens.push(Token::string(s, location, working_location.clone()));
            }
        }
        Some(State::Comment { mut s, location }) => {
            s.push(c);
            stack.push(State::Comment { s, location });
        }
        Some(State::Identifier { .. }) => {
            return Err(Error {
                message: "Cannot have strings inside identifiers".to_string(),
                location: working_location.clone(),
            })
        }
        None => {
            stack.push(State::String {
                s: "".to_string(),
                location: working_location.clone(),
            });
        }
    }

    Ok(())
}

fn handle_comment(
    stack: &mut Vec<State>,
    tokens: &mut Vec<Token>,
    c: char,
    working_location: &Location,
) -> Result<(), Error> {
    match stack.pop() {
        Some(State::String { .. }) => {
            return Err(Error {
                message: "Cannot have comments inside strings".to_string(),
                location: working_location.clone(),
            });
        }
        Some(State::Comment { mut s, location }) => {
            s.push(c);
            stack.push(State::Comment { s, location });
        }
        Some(State::Identifier { s, location }) => {
            tokens.push(Token::identifier(s, location, working_location.clone()));
            stack.push(State::Comment {
                s: "".to_string(),
                location: working_location.clone(),
            });
        }
        None => {
            stack.push(State::Comment {
                s: "".to_string(),
                location: working_location.clone(),
            });
        }
    }

    Ok(())
}

fn handle_character(
    stack: &mut Vec<State>,
    tokens: &mut Vec<Token>,
    c: char,
    working_location: &Location,
) -> Result<(), Error> {
    type TokCtor = fn(Location, Location) -> token::Token;

    const TERMINAL_TOKEN_CASES: [(char, fn(Location, Location) -> token::Token); 22] = [
        ('{', Token::lcurlybrace as TokCtor),
        ('}', Token::rcurlybrace as TokCtor),
        (',', Token::comma as TokCtor),
        (':', Token::colon as TokCtor),
        ('(', Token::lparen as TokCtor),
        (')', Token::rparen as TokCtor),
        ('[', Token::lsquarebracket as TokCtor),
        (']', Token::rsquarebracket as TokCtor),
        ('.', Token::period as TokCtor),
        ('=', Token::assign as TokCtor),
        ('!', Token::exclamationmark as TokCtor),
        ('<', Token::lchevron as TokCtor),
        ('>', Token::rchevron as TokCtor),
        ('*', Token::asterisk as TokCtor),
        ('+', Token::plus as TokCtor),
        ('-', Token::minus as TokCtor),
        ('/', Token::slash as TokCtor),
        ('%', Token::percent as TokCtor),
        ('&', Token::ampersand as TokCtor),
        ('|', Token::pipe as TokCtor),
        (';', Token::semicolon as TokCtor),
        ('^', Token::carrot as TokCtor),
    ];

    match stack.pop() {
        Some(State::String { mut s, location }) => {
            s.push(c);
            stack.push(State::String { s, location });
        }
        Some(State::Comment { mut s, location }) => {
            if c == '\n' {
                tokens.push(Token::comment(s, location, working_location.clone()));
            } else {
                s.push(c);
                stack.push(State::Comment { s, location });
            }
        }
        Some(State::Identifier { mut s, location }) => {
            if c.is_whitespace() {
                tokens.push(Token::identifier(s, location, working_location.clone()));
            } else {
                let end_location = {
                    let mut end_location = working_location.clone();
                    end_location.increment_column();
                    end_location
                };

                // Check if we should create a terminal token and identifier
                for (char, constructor) in TERMINAL_TOKEN_CASES {
                    if c == char {
                        tokens.push(Token::identifier(s, location, working_location.clone()));
                        tokens.push(constructor(working_location.clone(), end_location));
                        return Ok(());
                    }
                }

                // Otherwise just build identifier
                s.push(c);
                stack.push(State::Identifier { s, location });
            }
        }
        None => {
            if !c.is_whitespace() {
                let end_location = {
                    let mut end_location = working_location.clone();
                    end_location.increment_column();
                    end_location
                };

                // Check if we should create a terminal token
                for (char, constructor) in TERMINAL_TOKEN_CASES {
                    if c == char {
                        tokens.push(constructor(working_location.clone(), end_location));
                        return Ok(());
                    }
                }

                // Otherwise just build identifier
                stack.push(State::Identifier {
                    s: c.to_string(),
                    location: working_location.clone(),
                });
            }
        }
    }
    Ok(())
}

fn handle_remaining_stack(
    state: State,
    tokens: &mut Vec<Token>,
    mut working_location: Location,
) -> Result<(), Error> {
    match state {
        State::String { .. } => {
            working_location.subtract_column();
            return Err(Error {
                message: "Unterminated string".to_string(),
                location: working_location.clone(),
            });
        }
        State::Comment { s, location } => {
            tokens.push(Token::comment(s, location, working_location.clone()));
        }
        State::Identifier { s, location } => {
            tokens.push(Token::identifier(s, location, working_location.clone()));
        }
    }
    Ok(())
}


use super::*;

#[macro_export]
macro_rules! lexer_double_symbol{
    ($test_name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $test_name(){
            let result = lex($input, Location::default()).unwrap();

            let expected = vec![
                Token{
                    value: $expected,
                    start_location: Location::default(),
                    end_location: (0, 2).into()
                }
            ];

            assert_eq!(expected, result);
        }
    }
}

/// A macro for testing a single symbol.
/// Tests just parsing the symbol as well as that it ends an identifier.
#[macro_export]
macro_rules! lexer_single_symbol {
    ($token_type1:path, $test_name1:ident, $input1:expr,  
        $test_name2:ident) => {
        
        #[test]
        fn $test_name1() {
            let input = $input1;
            let result = lex(input, Location::default()).unwrap();

            let expected = vec![
                $token_type1(Location::default(), (0, 1).into())
            ];

            assert_eq!(expected, result);
        }

        #[test]
        fn $test_name2() {
            let mut input = "hello".to_string();
            input.push_str($input1);
            let result = lex(&input, Location::default()).unwrap();

            let expected = vec![
                Token::identifier("hello".to_string(), Location::default(), (0, 5).into()),
                $token_type1((0,5).into(), (0, 6).into())
            ];

            assert_eq!(expected, result);
        }
    };
}

lexer_single_symbol!(Token::colon, lex_colon, ":", lex_colon_ends_identifier);
lexer_single_symbol!(Token::lcurlybrace, lex_lcurlybrace, "{", lex_lcurlybrace_ends_identifier);
lexer_single_symbol!(Token::rcurlybrace, lex_rcurlybrace, "}", lex_rcurlybrace_ends_identifier);
lexer_single_symbol!(Token::comma, lex_comma, ",", lex_comma_ends_identifier);
lexer_single_symbol!(Token::lparen, lex_lparen, "(", lex_lparen_ends_identifier);
lexer_single_symbol!(Token::rparen, lex_rparen, ")", lex_rparen_ends_identifier);
lexer_single_symbol!(Token::lsquarebracket, lex_lsquarebracket, "[", lex_lsquarebracket_ends_identifier);
lexer_single_symbol!(Token::rsquarebracket, lex_rsquarebracket, "]", lex_rsquarebracket_ends_identifier);
lexer_single_symbol!(Token::period, lex_period, ".", lex_period_ends_identifier);
lexer_single_symbol!(Token::assign, lex_assign, "=", lex_assign_ends_identifier);
lexer_single_symbol!(Token::exclamationmark, lex_exclamationmark, "!", lex_exclamationmark_ends_identifier);
lexer_single_symbol!(Token::lchevron, lex_lchevron, "<", lex_lchevron_ends_identifier);
lexer_single_symbol!(Token::rchevron, lex_rchevron, ">", lex_rchevron_ends_identifier);
lexer_single_symbol!(Token::asterisk, lex_asterisk, "*", lex_asterisk_ends_identifier);
lexer_single_symbol!(Token::plus, lex_plus, "+", lex_plus_ends_identifier);
lexer_single_symbol!(Token::minus, lex_minus, "-", lex_minus_ends_identifier);
lexer_single_symbol!(Token::slash, lex_slash, "/", lex_slash_ends_identifier);
lexer_single_symbol!(Token::percent, lex_percent, "%", lex_percent_ends_identifier);
lexer_single_symbol!(Token::ampersand, lex_ampersand, "&", lex_ampersand_ends_identifier);
lexer_single_symbol!(Token::pipe, lex_pipe, "|", lex_pipe_ends_identifier);
lexer_single_symbol!(Token::semicolon, lex_semicolon, ";", lex_semicolon_ends_identifier);
lexer_single_symbol!(Token::carrot, lex_carrot, "^", lex_carrot_ends_identifier);



#[test]
fn plus_assign_isntconstructed(){
    let input = "+ =";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::plus((0,0).into(), (0, 1).into()),
        Token::assign((0,2).into(), (0, 3).into())
    ];

    assert_eq!(expected, result);
}

lexer_double_symbol!(lex_plus_assign, "+=", TokenValue::PlusAssign);
lexer_double_symbol!(lex_sub_assign, "-=", TokenValue::SubAssign);
lexer_double_symbol!(lex_mul_assign, "*=", TokenValue::MulAssign);
lexer_double_symbol!(lex_div_assign, "/=", TokenValue::DivAssign);
lexer_double_symbol!(lex_mod_assign, "%=", TokenValue::ModAssign);
lexer_double_symbol!(lex_and_assign, "&=", TokenValue::AndAssign);
lexer_double_symbol!(lex_or_assign, "|=", TokenValue::PipeAssign);
lexer_double_symbol!(lex_xor_assign, "^=", TokenValue::CarrotAssign);
lexer_double_symbol!(lex_plus_plus, "++", TokenValue::PlusPlus);
lexer_double_symbol!(lex_minus_minus, "--", TokenValue::MinusMinus);
lexer_double_symbol!(lex_not_not, "!!", TokenValue::NotNot);
lexer_double_symbol!(lex_or, "||", TokenValue::Or);
lexer_double_symbol!(lex_and, "&&", TokenValue::And);
lexer_double_symbol!(lex_less_than_equal, "<=", TokenValue::LessThanEqual);
lexer_double_symbol!(lex_greater_than_equal, ">=", TokenValue::GreaterThanEqual);
lexer_double_symbol!(lex_equal, "==", TokenValue::Equal);
lexer_double_symbol!(lex_not_equal, "!=", TokenValue::NotEqual);
lexer_double_symbol!(lex_lshift, "<<", TokenValue::LShift);
lexer_double_symbol!(lex_rshift, ">>", TokenValue::RShift);


#[test]
fn joins_comments_on_subsequent_lines(){
    let input = "\t# hello\n\t# world\n\t# foo";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::comments(vec!["hello".to_string(), "world".to_string(), "foo".to_string()], (0, 1).into(), (2, 6).into())
    ];

    assert_eq!(expected, result);
}

#[test]
fn joins_comments_on_multiple_subsequent_lines(){
    let input = "# hello\n# world";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::comments(vec!["hello".to_string(), "world".to_string()], (0, 0).into(), (1, 7).into())
    ];

    assert_eq!(expected, result);
}

#[test]
fn doesnt_join_comments_if_broken_up(){
    let input = "# hello\nfoo # world";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::comments(vec!["hello".to_string()], (0, 0).into(), (0, 7).into()),
        Token::identifier("foo".to_string(), (1, 0).into(), (1, 3).into()),
        Token::comment("world".to_string(), (1, 4).into(), (1, 11).into()),
    ];

    assert_eq!(expected, result);
}



#[test]
fn skips_whitespace() {
    let input = "  ";

    let result = lex(input, Location::default()).unwrap();

    let expected: Vec<Token> = vec![];
    assert_eq!(expected, result);
}

#[test]
fn zero_returns_number(){
    let input = "0";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::number(0.0, (0, 0).into(), (0, 1).into())
    ];

    assert_eq!(expected, result);

}

#[test]
fn positive_num_returns_number(){
    let input = "123.456";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::number(123.456, (0, 0).into(), (0, 7).into())
    ];

    assert_eq!(expected, result);
}

#[test]
fn complicated_nums(){
    let input = "123.456 78 . 9 10";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::number(123.456, (0, 0).into(), (0, 7).into()),
        Token::number(78.0, (0, 8).into(), (0, 10).into()),
        Token::period((0, 11).into(), (0, 12).into()),
        Token::number(9.0, (0, 13).into(), (0, 14).into()),
        Token::number(10.0, (0, 15).into(), (0, 17).into()),
    ];

    assert_eq!(expected, result);
}

#[test]
fn negative_num_returns_neg_and_number(){
    let input = "-123.456";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::minus((0, 0).into(), (0, 1).into()),
        Token::number(123.456, (0, 1).into(), (0, 8).into())
    ];

    assert_eq!(expected, result);
}

#[test]
fn multiple_periods_returns_err(){
    let input = "123.456.450";
    let result = lex(input, Location::default());

    let expected = Err(Error{
        message: "Cannot have multiple periods in a number".to_string(),
        location: (0, 7).into()
    });

    assert_eq!(expected, result);
}


#[test]
fn lex_linebreaks_identifiers() {
    let input = "hello\nworld bob";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::identifier("hello".to_string(), (0, 0).into(), (0, 5).into()),
        Token::identifier("world".to_string(), (1, 0).into(), (1, 5).into()),
        Token::identifier("bob".to_string(), (1, 6).into(), (1, 9).into()),
    ];
    assert_eq!(expected, result);
}

#[test]
fn lex_strings_and_symbols() {
    let input = "hello, \"world\", {";
    let result = lex(input, Location::default()).unwrap();
    let expected = vec![
        Token::identifier("hello".to_string(), (0, 0).into(), (0, 5).into()),
        Token::comma((0, 5).into(), (0, 6).into()),
        Token::string("world".to_string(), (0, 7).into(), (0, 13).into()),
        Token::comma((0, 14).into(), (0, 15).into()),
        Token::lcurlybrace((0, 16).into(), (0, 17).into()),
    ];
    assert_eq!(expected, result);
}

#[test]
fn lex_string_unclosed_returns_err() {
    let input = "\"hello";
    let result = lex(input, Location::default());

    let expected = Err(Error {
        message: "Unterminated string".to_string(),
        location: (0, 5).into(),
    });

    assert_eq!(expected, result);
}

#[test]
fn lex_strings_separated_by_symbols() {
    let input = r#""hello","hi"}"there"{"#;
    let result = lex(input, Location::default()).unwrap();
    let expected = vec![
        Token::string("hello".to_string(), (0, 0).into(), (0, 6).into()),
        Token::comma((0, 7).into(), (0, 8).into()),
        Token::string("hi".to_string(), (0, 8).into(), (0, 11).into()),
        Token::rcurlybrace((0, 12).into(), (0, 13).into()),
        Token::string("there".to_string(), (0, 13).into(), (0, 19).into()),
        Token::lcurlybrace((0, 20).into(), (0, 21).into()),
    ];
    assert_eq!(expected, result);
}

#[test]
fn lex_string() {
    let input = "\"hello\"";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![Token::string(
        "hello".to_string(),
        (0, 0).into(),
        (0, 6).into(),
    )];
    assert_eq!(expected, result);
}

#[test]
fn lex_string_nested_quote() {
    let input = "\"hello \\\"world\"";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![Token::string(
        "hello \"world".to_string(),
        (0, 0).into(),
        (0, 14).into(),
    )];
    assert_eq!(expected, result);
}

#[test]
fn lex_string_nested_string() {
    let input = "\"hello \\\"world\\\" bob\"";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![Token::string(
        "hello \"world\" bob".to_string(),
        (0, 0).into(),
        (0, 20).into(),
    )];
    assert_eq!(expected, result);
}

#[test]
fn lex_comment() {
    let input = "# hello ";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![Token::comment(
        "hello".to_string(),
        (0, 0).into(),
        (0, 8).into(),
    )];
    assert_eq!(expected, result);
}

#[test]
fn lex_comment_nested_comment() {
    let input = "# hello #hi";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![Token::comment(
        "hello #hi".to_string(),
        (0, 0).into(),
        (0, 11).into(),
    )];
    assert_eq!(expected, result);
}

#[test]
fn lex_comment_ends_w_newline() {
    let input = "# hello\nabc";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::comment("hello".to_string(), (0, 0).into(), (0, 7).into()),
        Token::identifier("abc".to_string(), (1, 0).into(), (1, 3).into()),
    ];

    assert_eq!(expected, result);
}

#[test]
fn lex_comment_nested_string() {
    let input = "# hello \"hi\"";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![Token::comment(
        "hello \"hi\"".to_string(),
        (0, 0).into(),
        (0, 12).into(),
    )];
    assert_eq!(expected, result);
}

#[test]
fn lex_identifier() {
    let input = "hello";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![Token::identifier(
        "hello".to_string(),
        (0, 0).into(),
        (0, 5).into(),
    )];
    assert_eq!(expected, result);
}

#[test]
fn lex_multiple_identifiers() {
    let input = "hello world";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::identifier("hello".to_string(), (0, 0).into(), (0, 5).into()),
        Token::identifier("world".to_string(), (0, 6).into(), (0, 11).into()),
    ];
    assert_eq!(expected, result);
}

#[test]
fn lex_multiple_identifiers_with_whitespace() {
    let input = "hello      world";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::identifier("hello".to_string(), (0, 0).into(), (0, 5).into()),
        Token::identifier("world".to_string(), (0, 11).into(), (0, 16).into()),
    ];
    assert_eq!(expected, result);
}

#[test]
fn lex_identifier_ends_with_comment_returns_err() {
    let input = "hello# world";
    let result = lex(input, Location::default()).unwrap();

    let expected = vec![
        Token::identifier("hello".to_string(), (0, 0).into(), (0, 5).into()),
        Token::comment("world".to_string(), (0, 5).into(), (0, 12).into()),
    ];
    assert_eq!(expected, result);
}

#[test]
fn lex_identifier_ends_with_string_returns_err() {
    let input = "hello\"world\"";
    let result = lex(input, Location::default());

    let expected = Err(Error {
        message: "Cannot have strings inside identifiers".to_string(),
        location: (0, 5).into(),
    });

    assert_eq!(expected, result);
}

#[test]
fn lex_string_ends_with_comment_returns_err() {
    let input = "\"hello# world\"";
    let result = lex(input, Location::default());

    let expected = Err(Error {
        message: "Cannot have comments inside strings".to_string(),
        location: (0, 6).into(),
    });

    assert_eq!(expected, result);
}
use QuotedParts;

#[test]
fn no_quotes() {
  assert!(QuotedParts::all("").is_empty());
  assert_eq!(vec!["hello"], QuotedParts::all("hello"));
  assert_eq!(vec!["these", "are", "words"], QuotedParts::all("these are words"));
  assert!(QuotedParts::all("     ").is_empty());
  assert_eq!(vec!["you", "don't", "know", "me"], QuotedParts::all("you    don't       know me"));
}

#[test]
fn quotes() {
  assert_eq!(vec![""], QuotedParts::all(r#""""#));
  assert_eq!(vec!["hello"], QuotedParts::all(r#""hello""#));
  assert_eq!(vec!["hello", "friend", "guy"], QuotedParts::all(r#"hello "friend" guy"#));
  assert_eq!(vec!["I have", "some quotes", "here", "for", "all of us"], QuotedParts::all(r#""I have" "some quotes" here for "all of us""#));
  assert_eq!(vec!["   ", "a"], QuotedParts::all(r#""   "    a"#));
}

#[test]
fn invalid_quotes() {
  assert_eq!(vec!["he\"llo"], QuotedParts::all("he\"llo"));
  assert_eq!(vec!["\"how", "are", "you", "doing?"], QuotedParts::all("\"how are you doing?"));
  assert_eq!(vec!["I", "am", "well\""], QuotedParts::all("I am well\""));
  assert_eq!(vec!["\""], QuotedParts::all("      \""));
  assert_eq!(vec!["\""], QuotedParts::all("\"      "));
}

use ex1::capitalize;

#[test]
fn more_words() {
    assert_eq!(capitalize("ciao ciao bye"), "Ciao Ciao Bye");
}

#[test]
fn no_space() {
    assert_eq!(capitalize("ciao"), "Ciao");
}

#[test]
fn accent_word() {
    assert_eq!(capitalize("questa è una frase"), "Questa È Una Frase");
}

#[test]
fn void_string() {
    assert_eq!(capitalize(""), "");
}

#[test]
fn more_spaces() {
    assert_eq!(capitalize("questa è una  frase"), "Questa È Una  Frase");
}
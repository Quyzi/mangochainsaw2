use cucumber::{given, then, when, World as _};
use mc::label::{DefaultLabel, Label};

#[derive(cucumber::World, Debug, Default)]
struct World {
    lhs: String,
    rhs: String,
    label: Option<DefaultLabel>,
}

#[given(expr = "{string}={string}")]
async fn is_label(w: &mut World, lhs: String, rhs: String) {
    w.lhs = lhs;
    w.rhs = rhs;
}

#[when(expr = "parser is {word}")]
async fn make_label(w: &mut World, parser: String) {
    assert!(!w.lhs.is_empty(), "label is missing left side.");
    assert!(!w.rhs.is_empty(), "label is missing right side.");

    match parser.as_str() {
        "parse_string" => {
            let this = DefaultLabel::parse_string(format!("{}={}", w.lhs, w.rhs));
            assert!(
                this.is_ok(),
                "parse_string returned error {}",
                this.unwrap_err()
            );
            w.label = Some(this.unwrap());
        }
        "parse_str" => {
            let this = DefaultLabel::parse_str(&format!("{}={}", w.lhs, w.rhs));
            assert!(
                this.is_ok(),
                "parse_str returned error {}",
                this.unwrap_err()
            );
            w.label = Some(this.unwrap());
        }
        _ => panic!("unknown parser {parser}"),
    }
}

#[then(expr = "label is {word}")]
async fn check_label(w: &mut World, word: String) {
    match word.as_str() {
        "valid" => {
            assert!(w.label.is_some());
            let this = w.label.clone().unwrap();
            assert_eq!(this.0, w.lhs, "label lhs not equal");
            assert_eq!(this.1, w.rhs, "label rhs not equal");

            assert!(!this.0.is_empty(), "Label is missing lhs");
            assert!(!this.1.is_empty(), "Label is missing rhs");

            let bytes_ltr = this.to_bytes_ltr();
            assert!(bytes_ltr.is_ok(), "failed to convert label to Bytes");
            let bytes_ltr = bytes_ltr.unwrap();

            let that_ltr = DefaultLabel::from_bytes(bytes_ltr);
            assert!(that_ltr.is_ok(), "failed to convert Bytes to Label");
            let that = that_ltr.unwrap();
            assert_eq!(this.0, that.0);
            assert_eq!(this.1, that.1);

            let bytes_rtl = this.to_bytes_rtl();
            assert!(bytes_rtl.is_ok(), "failed to convert label to Bytes");
            let bytes_rtl = bytes_rtl.unwrap();

            let that_rtl = DefaultLabel::from_bytes_rtl(bytes_rtl);
            assert!(that_rtl.is_ok(), "failed to convert Bytes to Label");
            let that = that_rtl.unwrap();
            assert_eq!(this.0, that.0);
            assert_eq!(this.1, that.1);
        }
        "invalid" => {
            if let Some(this) = w.label.clone() {
                assert!(
                    (this.0 == w.lhs) && (this.1 == w.rhs),
                    "label is valid. expected invalid"
                )
            }
        }
        _ => panic!("unknown label state {}", word),
    }
}

#[tokio::main]
async fn main() {
    World::run("tests/features/label").await;
}

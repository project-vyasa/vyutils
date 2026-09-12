use vyasa_sanskrit_wasm::*;

#[test]
fn test_transliterate_and_detection() {
    let deva = "अ॒ग्निमी॑ळे पु॒रोहि॑तम्";
    let detected = detect_script(deva);
    assert_eq!(detected, Some("Devanagari".to_string()));

    let telu = transliterate(deva, "devanagari", "telugu").unwrap();
    assert_eq!(telu, "అ॒గ్నిమీ॑ళే పు॒రోహి॑తమ్");

    let roundtrip = transliterate(&telu, "telugu", "devanagari").unwrap();
    assert_eq!(roundtrip, deva);

    let iast = transliterate(deva, "devanagari", "iast").unwrap();
    assert!(iast.contains("gnimī"));
}

#[test]
fn test_krama_generation() {
    let padas = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";
    let krama_text = generate_krama_text(padas, "devanagari").unwrap();
    assert!(krama_text.contains("अ॒ग्निमी॑ळे"));
    assert!(krama_text.contains("ई॒ळे॒ पु॒रोहि॑तम्"));
    assert!(krama_text.contains("पु॒रो-हि॑तम्"));

    let krama_telu = generate_krama_text(padas, "telugu").unwrap();
    assert!(krama_telu.contains("అ॒గ్నిమీ॑ళే"));
}

#[test]
fn test_pratyahara_checks() {
    assert!(check_pratyahara_contains("ac", "a").unwrap());
    assert!(check_pratyahara_contains("ac", "i").unwrap());
    assert!(check_pratyahara_contains("ac", "u").unwrap());
    assert!(!check_pratyahara_contains("ac", "k").unwrap());

    assert!(check_pratyahara_contains("hal", "k").unwrap());
    assert!(check_pratyahara_contains("hal", "h").unwrap());
    assert!(!check_pratyahara_contains("hal", "a").unwrap());

    assert!(check_pratyahara_contains("yaṇ", "y").unwrap());
    assert!(check_pratyahara_contains("yaṇ", "v").unwrap());
    assert!(check_pratyahara_contains("yaṇ", "ra").unwrap());
    assert!(check_pratyahara_contains("yaṇ", "la").unwrap());
    assert!(!check_pratyahara_contains("yaṇ", "k").unwrap());
}

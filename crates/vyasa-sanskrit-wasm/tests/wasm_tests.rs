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
fn test_jata_generation() {
    let padas = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";
    let jata_deva = generate_jata_text(padas, "devanagari").unwrap();
    assert!(jata_deva.contains("अ॒ग्निमी॑ळे ई॒ळे॒ऽग्निम् अ॒ग्निमी॑ळे"));
    assert!(jata_deva.contains("पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्"));

    let jata_telu = generate_jata_text(padas, "telugu").unwrap();
    assert!(jata_telu.contains("అ॒గ్నిమీ॑ళే"));
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

#[test]
fn test_taittiriya_wasm_apis() {
    // Svarita types retrieval via JSON
    let svaritas_json = get_taittiriya_svaritas_json().unwrap();
    let svaritas: Vec<dto::TaittiriyaSvaritaDto> = serde_json::from_str(&svaritas_json).unwrap();
    assert_eq!(svaritas.len(), 8);
    assert!(svaritas.iter().any(|s| s.id == "Jatya" && s.is_nitya));
    assert!(svaritas.iter().any(|s| s.id == "Kshaipra" && s.is_nitya));
    assert!(svaritas
        .iter()
        .any(|s| s.id == "Tairovyanjana" && !s.is_nitya));

    // Varṇa inspection with TPr Karaṇa via JSON
    let t_json = inspect_taittiriya_varna_json("t").unwrap();
    let t_dto: dto::TaittiriyaVarnaDto = serde_json::from_str(&t_json).unwrap();
    assert!(t_dto.karana.contains("Jihvāgram"));
    assert_eq!(t_dto.sthana, vec!["Danta (Dental)"]);

    let c_json = inspect_taittiriya_varna_json("c").unwrap();
    let c_dto: dto::TaittiriyaVarnaDto = serde_json::from_str(&c_json).unwrap();
    assert!(c_dto.karana.contains("Jihvopamadhya"));

    // Direct DTO function
    let k_dto = inspect_taittiriya_varna_dto("k").unwrap();
    assert!(k_dto.karana.contains("Jihvāmādhya"));

    // Svarita context classification
    let res = classify_taittiriya_svarita_by_context("SemivowelSandhi").unwrap();
    assert_eq!(res, Some("Kshaipra".to_string()));

    let res_abh = classify_taittiriya_svarita_by_context("AbhinihitaElision").unwrap();
    assert_eq!(res_abh, Some("Abhinihita".to_string()));

    // Dvitva checks
    let dvitva_double = check_taittiriya_dvitva(Some("r".to_string()), "k", None).unwrap();
    assert!(dvitva_double);

    let dvitva_no_double = check_taittiriya_dvitva(None, "k", Some("a".to_string())).unwrap();
    assert!(!dvitva_no_double);
}

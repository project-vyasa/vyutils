use vyasa_lipi::Script;
use vyasa_patha::generate_krama_in_script;
use vyasa_patha::prakriti::{generate_krama_patha, parse_pada_patha};

#[test]
fn test_rigveda_1_1_1_full_krama_patha() {
    let pada_input = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् । य॒ज्ञस्य॑ । दे॒वम् । ऋ॒त्विज॑म् । होता॑रम् । रत्न॒-धात॑मम् ॥";
    let padas = parse_pada_patha(pada_input);
    assert_eq!(padas.len(), 8);

    let steps = generate_krama_patha(&padas);

    // 8 padas -> 7 pairs + 1 terminal iti step = 8 steps total
    assert_eq!(steps.len(), 8);

    // Step 1: 1-2 (अ॒ग्निमी॑ळे) - Udātta + Anudātta -> Svarita
    assert_eq!(steps[0].first_index, 1);
    assert_eq!(steps[0].second_index, Some(2));
    assert_eq!(steps[0].text, "अ॒ग्निमी॑ळे");

    // Step 2: 2-3 (ई॒ळे॒ पु॒रोहि॑तम्)
    assert_eq!(steps[1].first_index, 2);
    assert_eq!(steps[1].second_index, Some(3));
    assert_eq!(steps[1].text, "ई॒ळे॒ पु॒रोहि॑तम्");

    // Step 3: 3-4 (पु॒रोहि॑तं य॒ज्ञस्य॑)
    assert_eq!(steps[2].first_index, 3);
    assert_eq!(steps[2].second_index, Some(4));
    assert_eq!(steps[2].text, "पु॒रोहि॑तं य॒ज्ञस्य॑");

    // Step 4: 4-5 (य॒ज्ञस्य॑ दे॒वम्)
    assert_eq!(steps[3].first_index, 4);
    assert_eq!(steps[3].second_index, Some(5));
    assert_eq!(steps[3].text, "य॒ज्ञस्य॑ दे॒वम्");

    // Step 5: 5-6 (दे॒वमृ॒त्विज॑म्)
    assert_eq!(steps[4].first_index, 5);
    assert_eq!(steps[4].second_index, Some(6));
    assert_eq!(steps[4].text, "दे॒वमृ॒त्विज॑म्");

    // Step 6: 6-7 (ऋ॒त्विजं॑ होता॑रम्)
    assert_eq!(steps[5].first_index, 6);
    assert_eq!(steps[5].second_index, Some(7));
    assert_eq!(steps[5].text, "ऋ॒त्विजं॑ होता॑रम्");

    // Step 7: 7-8 (होता॑रं रत्न॒धात॑मम्)
    assert_eq!(steps[6].first_index, 7);
    assert_eq!(steps[6].second_index, Some(8));
    assert_eq!(steps[6].text, "होता॑रं रत्न॒धात॑मम्");

    // Step 8: Terminal Parigraha on 8 (रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्)
    assert_eq!(steps[7].first_index, 8);
    assert_eq!(steps[7].second_index, None);
    assert!(steps[7].is_parigraha);
    assert_eq!(steps[7].text, "रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्");
}

#[test]
fn test_krama_with_pragrhya_internal_parigraha() {
    // 1: harī (Pragṛhya), 2: etau, 3: viharataḥ
    let input = "हरी । एतौ । विहरतः ॥";
    let padas = parse_pada_patha(input);
    let steps = generate_krama_patha(&padas);

    // Pair 1: 1-2 (हरी एतौ)
    assert_eq!(steps[0].text, "हरी एतौ");

    // Pair 2: 2-3 (एतौ विहरतः)
    assert_eq!(steps[1].text, "एतौ विहरतः");

    // Step 3: Terminal iti on 3 (विहरत इति विहरतः)
    assert!(steps[2].is_parigraha);
    assert_eq!(steps[2].text, "विहरतः इति॑ विहरतः");
}

#[test]
fn test_krama_generation_in_telugu_and_kannada() {
    let pada_input = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";

    let telugu_krama = generate_krama_in_script(pada_input, Script::Telugu);
    assert!(telugu_krama.contains("మితి॑"));

    let kannada_krama = generate_krama_in_script(pada_input, Script::Kannada);
    assert!(kannada_krama.contains("ಮಿತಿ॑"));
}

#[test]
fn test_krama_generation_in_iast_and_iso15919() {
    let pada_input = "अ॒ग्निम् । ई॒ळे॒ ।";
    let iast_krama = generate_krama_in_script(pada_input, Script::Iast);
    assert!(iast_krama.contains("gnimī"));
    assert!(iast_krama.contains("ḷe"));
    assert!(iast_krama.contains("iti"));

    let iso_krama = generate_krama_in_script(pada_input, Script::Iso15919);
    assert!(iso_krama.contains("gnimī"));
    assert!(iso_krama.contains("ḻē"));
    assert!(iso_krama.contains("iti"));
}

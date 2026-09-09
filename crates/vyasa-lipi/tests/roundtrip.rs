use pretty_assertions::assert_eq;
use vyasa_lipi::{
    transliterate, transliterate_with_options, AccentMode, Script, TransliterateOptions,
};

#[test]
fn test_gita_1_1_roundtrip() {
    let deva = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः ।\nमामकाः पाण्डवाश्चैव किमकुर्वत सञ्जय ॥";

    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
    let knda = transliterate(deva, Script::Devanagari, Script::Kannada);
    let iso = transliterate(deva, Script::Devanagari, Script::Iso15919);
    let iast = transliterate(deva, Script::Devanagari, Script::Iast);

    // Verify Telugu
    assert_eq!(
        telu,
        "ధర్మక్షేత్రే కురుక్షేత్రే సమవేతా యుయుత్సవః ।\nమామకాః పాణ్డవాశ్చైవ కిమకుర్వత సఞ్జయ ॥"
    );

    // Verify Kannada
    assert_eq!(
        knda,
        "ಧರ್ಮಕ್ಷೇತ್ರೇ ಕುರುಕ್ಷೇತ್ರೇ ಸಮವೇತಾ ಯುಯುತ್ಸವಃ ।\nಮಾಮಕಾಃ ಪಾಣ್ಡವಾಶ್ಚೈವ ಕಿಮಕುರ್ವತ ಸಞ್ಜಯ ॥"
    );

    // Verify ISO 15919 (macrons on ē, ṁ for anusvāra)
    assert_eq!(
        iso,
        "dharmakṣētrē kurukṣētrē samavētā yuyutsavaḥ |\nmāmakāḥ pāṇḍavāścaiva kimakurvata sañjaya ||"
    );

    // Verify IAST (bare e, ṃ for anusvāra)
    assert_eq!(
        iast,
        "dharmakṣetre kurukṣetre samavetā yuyutsavaḥ |\nmāmakāḥ pāṇḍavāścaiva kimakurvata sañjaya ||"
    );

    // Round-trip back from Indic scripts
    assert_eq!(
        transliterate(&telu, Script::Telugu, Script::Devanagari),
        deva
    );
    assert_eq!(
        transliterate(&knda, Script::Kannada, Script::Devanagari),
        deva
    );
    assert_eq!(transliterate(&telu, Script::Telugu, Script::Kannada), knda);

    // Round-trip back from ISO 15919
    assert_eq!(
        transliterate(&iso, Script::Iso15919, Script::Devanagari),
        deva
    );
    assert_eq!(transliterate(&iso, Script::Iso15919, Script::Telugu), telu);

    // Round-trip back from IAST
    assert_eq!(transliterate(&iast, Script::Iast, Script::Devanagari), deva);
}

#[test]
fn test_rigveda_1_1_1_all_scripts() {
    let deva = "ॐ अ॒ग्निमी॑ळे पु॒रोहि॑तं य॒ज्ञस्य॑ दे॒वमृ॒त्विज॑म् । होता॑रं रत्न॒धात॑मम् ॥";

    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
    let knda = transliterate(deva, Script::Devanagari, Script::Kannada);

    // Indic round-trips
    assert_eq!(telu, "ఓం అ॒గ్నిమీ॑ళే పు॒రోహి॑తం య॒జ్ఞస్య॑ దే॒వమృ॒త్విజ॑మ్ । హోతా॑రం రత్న॒ధాత॑మమ్ ॥");

    // Verify roundtrip back to Devanagari from Telugu and Kannada
    assert_eq!(
        transliterate(&telu, Script::Telugu, Script::Devanagari),
        deva
    );
    assert_eq!(
        transliterate(&knda, Script::Kannada, Script::Devanagari),
        deva
    );
    assert_eq!(transliterate(&telu, Script::Telugu, Script::Kannada), knda);

    let opts = TransliterateOptions {
        accent_mode: AccentMode::PreserveUnicode,
    };
    let iso_unicode = transliterate_with_options(deva, Script::Devanagari, Script::Iso15919, &opts);
    assert_eq!(
        iso_unicode,
        "ōṁ a\u{0952}gnimī\u{0951}ḻē pu\u{0952}rōhi\u{0951}taṁ ya\u{0952}jñasya\u{0951} dē\u{0952}vamr̥\u{0952}tvija\u{0951}m | hōtā\u{0951}raṁ ratna\u{0952}dhāta\u{0951}mam ||"
    );

    // Round-trip back from ISO Unicode mode to Devanagari
    assert_eq!(
        transliterate(&iso_unicode, Script::Iso15919, Script::Devanagari),
        deva
    );

    // Roman transliteration with scholarly diacritics
    let iso_scholarly = transliterate(deva, Script::Devanagari, Script::Iso15919);
    assert!(iso_scholarly.contains("a̱")); // Anudatta
    assert!(iso_scholarly.contains("mī́")); // Svarita
    assert!(iso_scholarly.contains("ḻē")); // Lateral flap + long ē
    assert!(iso_scholarly.contains("r̥̱")); // Vocalic r̥ with anudatta under-line

    // Round trip scholarly Roman back to Devanagari
    assert_eq!(
        transliterate(&iso_scholarly, Script::Iso15919, Script::Devanagari),
        deva
    );
}

#[test]
fn test_iso15919_vs_iast_vocalic_liquids() {
    let deva = "ऋषिः ॠकारः ऌकारः";

    let iso = transliterate(deva, Script::Devanagari, Script::Iso15919);
    let iast = transliterate(deva, Script::Devanagari, Script::Iast);

    // ISO 15919 MUST have under-rings
    assert_eq!(iso, "r̥ṣiḥ r̥̄kāraḥ l̥kāraḥ");

    // IAST MUST have under-dots
    assert_eq!(iast, "ṛṣiḥ ṝkāraḥ ḷkāraḥ");

    // Round-trip both back to Devanagari
    assert_eq!(
        transliterate(&iso, Script::Iso15919, Script::Devanagari),
        deva
    );
    assert_eq!(transliterate(&iast, Script::Iast, Script::Devanagari), deva);
}

#[test]
fn test_dravidian_short_vowels() {
    // Telugu short/long e and o
    let telu_short_e = "ఎలుక"; // rat (short e)
    let telu_long_e = "ఏనుగు"; // elephant (long ē)

    let iso_short = transliterate(telu_short_e, Script::Telugu, Script::Iso15919);
    let iso_long = transliterate(telu_long_e, Script::Telugu, Script::Iso15919);

    assert_eq!(iso_short, "eluka");
    assert_eq!(iso_long, "ēnugu");

    // Round-trip
    assert_eq!(
        transliterate(&iso_short, Script::Iso15919, Script::Telugu),
        telu_short_e
    );
    assert_eq!(
        transliterate(&iso_long, Script::Iso15919, Script::Telugu),
        telu_long_e
    );

    // Kannada short/long o
    let knda_short_o = "ಒಂಟೆ";
    let knda_long_o = "ಓದು";

    let iso_knda_short = transliterate(knda_short_o, Script::Kannada, Script::Iso15919);
    let iso_knda_long = transliterate(knda_long_o, Script::Kannada, Script::Iso15919);

    assert_eq!(iso_knda_short, "oṁṭe");
    assert_eq!(iso_knda_long, "ōdu");

    // Round-trip
    assert_eq!(
        transliterate(&iso_knda_short, Script::Iso15919, Script::Kannada),
        knda_short_o
    );
    assert_eq!(
        transliterate(&iso_knda_long, Script::Iso15919, Script::Kannada),
        knda_long_o
    );
}

#[test]
fn test_vedic_lateral_flap_and_aspirate() {
    // ळ and ळ्ह
    let deva = "ईळे ळ्हा";

    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
    let knda = transliterate(deva, Script::Devanagari, Script::Kannada);
    let iso = transliterate(deva, Script::Devanagari, Script::Iso15919);
    let iast = transliterate(deva, Script::Devanagari, Script::Iast);

    assert_eq!(telu, "ఈళే ళ్హా");
    assert_eq!(knda, "ಈಳೇ ಳ್ಹಾ");
    assert_eq!(iso, "īḻē ḻhā");
    assert_eq!(iast, "īḷe ḷhā");

    // Round trip
    assert_eq!(
        transliterate(&telu, Script::Telugu, Script::Devanagari),
        deva
    );
    assert_eq!(
        transliterate(&knda, Script::Kannada, Script::Devanagari),
        deva
    );
    assert_eq!(
        transliterate(&iso, Script::Iso15919, Script::Devanagari),
        deva
    );
    assert_eq!(transliterate(&iast, Script::Iast, Script::Devanagari), deva);
}

#[test]
fn test_jihvamuliya_and_upadhmaniya() {
    let deva = "ᳵक ᳶप";

    let iso = transliterate(deva, Script::Devanagari, Script::Iso15919);
    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);

    assert_eq!(iso, "ḫka ẖpa");
    assert_eq!(telu, "ᳵక ᳶప");

    // Round trip
    assert_eq!(
        transliterate(&iso, Script::Iso15919, Script::Devanagari),
        deva
    );
    assert_eq!(
        transliterate(&telu, Script::Telugu, Script::Devanagari),
        deva
    );
}

#[test]
fn test_avagraha_and_sandhi() {
    let deva = "सोऽहम्";
    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
    let knda = transliterate(deva, Script::Devanagari, Script::Kannada);
    let iso = transliterate(deva, Script::Devanagari, Script::Iso15919);

    assert_eq!(telu, "సోఽహమ్");
    assert_eq!(knda, "ಸೋಽಹಮ್");
    assert_eq!(iso, "sō'ham");

    // Round trip
    assert_eq!(
        transliterate(&telu, Script::Telugu, Script::Devanagari),
        deva
    );
    assert_eq!(
        transliterate(&knda, Script::Kannada, Script::Devanagari),
        deva
    );
    assert_eq!(
        transliterate(&iso, Script::Iso15919, Script::Devanagari),
        deva
    );
}

#[test]
fn test_grantha_malayalam_bengali_roundtrip() {
    let deva = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः ।";

    // 1. Malayalam
    let mlym = transliterate(deva, Script::Devanagari, Script::Malayalam);
    assert_eq!(mlym, "ധര്മക്ഷേത്രേ കുരുക്ഷേത്രേ സമവേതാ യുയുത്സവഃ ।");
    assert_eq!(
        transliterate(&mlym, Script::Malayalam, Script::Devanagari),
        deva
    );

    // 2. Grantha
    let gran = transliterate(deva, Script::Devanagari, Script::Grantha);
    assert_eq!(
        transliterate(&gran, Script::Grantha, Script::Devanagari),
        deva
    );

    // 3. Bengali
    let beng = transliterate(deva, Script::Devanagari, Script::Bengali);
    assert_eq!(beng, "ধর্মক্ষেত্রে কুরুক্ষেত্রে সমবেতা যুযুত্সবঃ ।");

    // 4. Vedic Rigveda 1.1.1 in Grantha and Malayalam
    let rv = "ॐ अ॒ग्निमी॑ळे पु॒रोहि॑तं";
    let gran_rv = transliterate(rv, Script::Devanagari, Script::Grantha);
    let mlym_rv = transliterate(rv, Script::Devanagari, Script::Malayalam);

    assert_eq!(
        transliterate(&gran_rv, Script::Grantha, Script::Devanagari),
        rv
    );
    assert_eq!(
        transliterate(&mlym_rv, Script::Malayalam, Script::Devanagari),
        rv
    );
}

#[test]
fn test_ascii_slp1_roundtrip() {
    let deva = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः ।";
    let slp1 = transliterate(deva, Script::Devanagari, Script::Slp1);
    assert_eq!(slp1, "Darmakzetre kurukzetre samavetA yuyutsavaH .");

    // Roundtrip back to Devanagari
    assert_eq!(transliterate(&slp1, Script::Slp1, Script::Devanagari), deva);

    // Rigveda 1.1.1 with Vedic accents in SLP1
    let rv_deva = "ॐ अ॒ग्निमी॑ळे पु॒रोहि॑तं";
    let rv_slp1 = transliterate(rv_deva, Script::Devanagari, Script::Slp1);
    assert_eq!(rv_slp1, "oM a\\gnimI^Le pu\\rohi^taM");

    // Roundtrip back from SLP1 to Devanagari with accents intact!
    assert_eq!(
        transliterate(&rv_slp1, Script::Slp1, Script::Devanagari),
        rv_deva
    );
}

#[test]
fn test_ascii_harvard_kyoto_roundtrip() {
    let deva = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः ।";
    let hk = transliterate(deva, Script::Devanagari, Script::HarvardKyoto);
    assert_eq!(hk, "dharmakSetre kurukSetre samavetA yuyutsavaH |");

    // Roundtrip back to Devanagari
    assert_eq!(
        transliterate(&hk, Script::HarvardKyoto, Script::Devanagari),
        deva
    );
}

#[test]
fn test_ascii_wx_roundtrip() {
    let deva = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः ।";
    let wx = transliterate(deva, Script::Devanagari, Script::Wx);
    assert_eq!(wx, "XarmakRewre kurukRewre samavewA yuyuwsavaH |");

    // Roundtrip back to Devanagari
    assert_eq!(transliterate(&wx, Script::Wx, Script::Devanagari), deva);
}

//! Pre-Pāṇinian phonology and phonetic rules of the *Taittirīya-Prātiśākhya* (Kṛṣṇa Yajurveda).
//!
//! The *Taittirīya-Prātiśākhya* (TPr) represents the phonetic authority for ~75-80% of all
//! living Vedic ritual recitations (including *Śrī Rudram*, *Camakam*, *Taittirīya Upaniṣad*,
//! and *Puruṣa Sūkta*).
//!
//! Major classical commentaries:
//! - *Tribhāṣyaratna* by Somayārya
//! - *Vaidikābharaṇa* by Gārgya Gopālayajvan

use crate::sound::{Ayogavaha, Consonant, Svara, Varna, Vowel, VowelLength};

/// Active Articulator (*Karaṇa*) as distinguished from Passive Place (*Sthāna*).
///
/// In *Taittirīya-Prātiśākhya* Chapter 2 (sūtras 2.33–45), the active organ (*karaṇa*)
/// that moves to contact or approach the passive place (*sthāna*) is systematically identified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Karana {
    /// *Jihvāgram* (Tip of the tongue):
    /// Articulates dentals (*t, th, d, dh, n, l, s*) against teeth / tooth-roots (TPr 2.38).
    Jihvagram,

    /// *Jihvopamadhya* (Blade / forward edges of the tongue):
    /// Articulates palatals (*c, ch, j, jh, ñ, y, ś*) against the hard palate (TPr 2.36).
    Jihvopamadhya,

    /// *Jihvāmādhya* (Middle of the tongue dorsum):
    /// Articulates velars (*k, kh, g, gh, ṅ*) against the velum/throat (TPr 2.35).
    Jihvamadhya,

    /// *Jihvāmūla* (Root of the tongue):
    /// Articulates the Jihvāmūlīya fricative (TPr 2.44).
    Jihvamula,

    /// *Prativeṣṭitam* (Curled tongue tip):
    /// The tongue tip curls backwards to articulate retroflexes (*ṭ, ṭh, ḍ, ḍh, ṇ, r, ṣ, ḷ*) (TPr 2.37).
    Prativestitam,

    /// *Adharoṣṭha* (Lower lip):
    /// Articulates bilabials (*p, ph, b, bh, m, upadhmānīya*) against the upper lip,
    /// and dentolabial (*v*) against upper teeth (TPr 2.39, 2.43).
    Adharostha,

    /// *Hanu* (Jaws / open vocal tract):
    /// Open vocal tract without active consonantal contact (vowels) (TPr 2.12).
    Hanu,

    /// *Nāsikābila* (Nasal cavity):
    /// Pure resonance in the nasal cavity without oral contact (*Nāsikya*, *Anusvāra*) (TPr 2.50).
    NasikaBila,
}

/// Returns the active articulator (*Karaṇa*) for a sound according to *Taittirīya-Prātiśākhya* 2.33–45.
pub fn karana(varna: &Varna) -> Karana {
    match varna {
        Varna::Vowel(_) => Karana::Hanu,
        Varna::Consonant(c) => match c {
            Consonant::K | Consonant::Kh | Consonant::G | Consonant::Gh | Consonant::Ng => {
                Karana::Jihvamadhya
            }
            Consonant::C
            | Consonant::Ch
            | Consonant::J
            | Consonant::Jh
            | Consonant::Ny
            | Consonant::Y
            | Consonant::Sh => Karana::Jihvopamadhya,
            Consonant::Tt
            | Consonant::Tth
            | Consonant::Dd
            | Consonant::Ddh
            | Consonant::Nn
            | Consonant::R
            | Consonant::Ss
            | Consonant::LVedic
            | Consonant::LhVedic => Karana::Prativestitam,
            Consonant::T
            | Consonant::Th
            | Consonant::D
            | Consonant::Dh
            | Consonant::N
            | Consonant::L
            | Consonant::S => Karana::Jihvagram,
            Consonant::P
            | Consonant::Ph
            | Consonant::B
            | Consonant::Bh
            | Consonant::M
            | Consonant::V => Karana::Adharostha,
            Consonant::H => Karana::Hanu,
        },
        Varna::Ayogavaha(a) => match a {
            Ayogavaha::Jihvamuliya => Karana::Jihvamula,
            Ayogavaha::Upadhmaniya => Karana::Adharostha,
            Ayogavaha::Anusvara
            | Ayogavaha::Candrabindu
            | Ayogavaha::GomukhaAnusvara
            | Ayogavaha::DvibinduAnusvara
            | Ayogavaha::Nasikya
            | Ayogavaha::Ranga => Karana::NasikaBila,
            Ayogavaha::Visarga | Ayogavaha::Ardhavisarga => Karana::Hanu,
        },
    }
}

/// The 7 (or 8) Svarita Varieties according to *Taittirīya-Prātiśākhya* Chapter 20.
///
/// Sūtra 20.1:
/// *"Jātyo 'bhinihitaḥ kṣaipraḥ praśliṣṭas tairovyañjanas tairovirāmaḥ pādavṛttaḥ tathābhāvya iti svaritāḥ."*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaittiriyaSvarita {
    /// **Jātya** (Inherent / Nitya Svarita):
    /// Inherent within an uncompounded stem where a semivowel (*y/v*) replaced an original
    /// Udātta vowel internally without external Sandhi.
    /// Examples: *kva* (क्व॑), *kanyā* (क॒न्य॑), *rājyam* (रा॒ज्य॑म्), *nyañc* (न्य॑ञ्च्), *svar* (स्व॑र्).
    Jatya,

    /// **Kṣaipra** ("Quick" Svarita):
    /// Formed when an Udātta *i/ī* or *u/ū* merges into semivowel *y* or *v* before an unlike vowel.
    /// Examples: *vyaktam* (वि + अ॑क्तम् &rarr; व्य॑क्तम्), *agniṣvāgne* (अ॒ग्निषु॑ + अ॒ग्ने &rarr; अ॒ग्निष्वाग्ने॑).
    Kshaipra,

    /// **Abhinihita** Svarita:
    /// Formed when an Udātta *e* or *o* absorbs a following short *a-*, eliding it into avagraha (`ऽ`).
    /// Examples: *te 'bruvan* (ते + अ॑ब्रुवन् &rarr; तेऽ॑ब्रुवन्), *so 'bravīt* (सो॑ऽब्रवीत्).
    Abhinihita,

    /// **Praśliṣṭa** Svarita:
    /// Formed by coalescence of two vowels of the same class (especially *i + i &rarr; ī*)
    /// where the first vowel carried Udātta.
    /// Examples: *divīva* (दि॒वि + इ॑व &rarr; दि॒वीव॑), *sūktam* (सु॒ + उ॑क्तम् &rarr; सू॒क्तम्).
    Prashlishta,

    /// **Tairovyañjana** Svarita (Cross-consonantal enclitic):
    /// The standard dependent svarita where an unaccented (*anudātta*) syllable follows an Udātta
    /// separated by one or more consonants.
    /// Examples: *gobhiḥ* (गोभि॑ः), *agnimīḷe* (अ॒ग्निमी॑ळे).
    Tairovyanjana,

    /// **Tairovirāma** Svarita:
    /// An enclitic Svarita occurring across a word boundary or virāma pause.
    Tairovirama,

    /// **Pādavṛtta** Svarita:
    /// Occurs when an Udātta vowel meets an Anudātta vowel in **hiatus** (uncombined due to
    /// Pragṛhya status or dropped visarga).
    /// Examples: *pra ugam* (प्र उ॑गम्), *sa oṣadhīḥ* (स ओ॑षधीः).
    Padavrtta,

    /// **Tathābhāvya** Svarita:
    /// Secondary accentual transformation where a resulting sound undergoes svarita induction.
    Tathabhavya,
}

impl TaittiriyaSvarita {
    /// Returns `true` if this Svarita is an independent / primary (*Nitya*) Svarita.
    ///
    /// The four Nitya Svaritas (*Jātya, Kṣaipra, Abhinihita, Praśliṣṭa*) carry an intrinsic
    /// pitch-accent and do not depend on an immediately preceding Udātta word in the sentence.
    pub const fn is_nitya(&self) -> bool {
        matches!(
            self,
            Self::Jatya | Self::Kshaipra | Self::Abhinihita | Self::Prashlishta
        )
    }

    /// Returns `true` if this Svarita is an enclitic / secondary (*Kārya / Dependent*) Svarita.
    pub const fn is_enclitic(&self) -> bool {
        !self.is_nitya()
    }

    /// Returns the ASCII machine identifier (e.g. "Kshaipra", "Jatya").
    pub const fn id(&self) -> &'static str {
        match self {
            Self::Jatya => "Jatya",
            Self::Kshaipra => "Kshaipra",
            Self::Abhinihita => "Abhinihita",
            Self::Prashlishta => "Prashlishta",
            Self::Tairovyanjana => "Tairovyanjana",
            Self::Tairovirama => "Tairovirama",
            Self::Padavrtta => "Padavrtta",
            Self::Tathabhavya => "Tathabhavya",
        }
    }

    /// Canonical Sanskrit name in Devanagari.
    pub const fn name_deva(&self) -> &'static str {
        match self {
            Self::Jatya => "जात्य",
            Self::Kshaipra => "क्षैप्र",
            Self::Abhinihita => "अभिनिहित",
            Self::Prashlishta => "प्रश्लिष्ट",
            Self::Tairovyanjana => "तैरोव्यञ्जन",
            Self::Tairovirama => "तैरोविराम",
            Self::Padavrtta => "पादवृत्त",
            Self::Tathabhavya => "तथाभाव्य",
        }
    }

    /// Scholarly IAST designation.
    pub const fn name_iast(&self) -> &'static str {
        match self {
            Self::Jatya => "Jātya",
            Self::Kshaipra => "Kṣaipra",
            Self::Abhinihita => "Abhinihita",
            Self::Prashlishta => "Praśliṣṭa",
            Self::Tairovyanjana => "Tairovyañjana",
            Self::Tairovirama => "Tairovirāma",
            Self::Padavrtta => "Pādavṛtta",
            Self::Tathabhavya => "Tathābhāvya",
        }
    }
}

/// Svarita context helper for classifying a Svarita in a Vedic word or juncture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvaritaJunctureContext {
    /// Internal to a single uncombined stem containing y or v (e.g. kva, kanya, rājyam).
    InternalSemivowelStem,
    /// Vowel sandhi of i/u -> y/v before unlike vowel (e.g. vyaktam, agniṣvāgne).
    SemivowelSandhi,
    /// Abhinihita sandhi e/o + a -> e'/o' with avagraha (e.g. te 'bruvan, īḷe 'gnim).
    AbhinihitaElision,
    /// Homorganic vowel coalescence i + i -> ī or u + u -> ū (e.g. divīva).
    CoalescentLongVowel,
    /// Preceded by Udātta separated by consonants (e.g. gobhiḥ).
    PostUdattaConsonant,
    /// Preceded by Udātta separated by hiatus without sandhi (e.g. pra ugam).
    HiatusWithoutSandhi,
    /// Preceded by Udātta across a virāma / hemistich pause.
    AcrossVirama,
}

/// Classifies a Svarita into its canonical *Taittirīya-Prātiśākhya* variety.
pub fn classify_taittiriya_svarita(
    svara: Svara,
    context: SvaritaJunctureContext,
) -> Option<TaittiriyaSvarita> {
    if !matches!(svara, Svara::Svarita | Svara::DirghaSvarita) {
        return None;
    }

    let svarita_type = match context {
        SvaritaJunctureContext::InternalSemivowelStem => TaittiriyaSvarita::Jatya,
        SvaritaJunctureContext::SemivowelSandhi => TaittiriyaSvarita::Kshaipra,
        SvaritaJunctureContext::AbhinihitaElision => TaittiriyaSvarita::Abhinihita,
        SvaritaJunctureContext::CoalescentLongVowel => TaittiriyaSvarita::Prashlishta,
        SvaritaJunctureContext::PostUdattaConsonant => TaittiriyaSvarita::Tairovyanjana,
        SvaritaJunctureContext::HiatusWithoutSandhi => TaittiriyaSvarita::Padavrtta,
        SvaritaJunctureContext::AcrossVirama => TaittiriyaSvarita::Tairovirama,
    };

    Some(svarita_type)
}

/// Consonant Doubling (*Dvirvacana / Dvitva*) Engine according to *Taittirīya-Prātiśākhya* Chapter 14.
///
/// In traditional Krishna Yajurveda recitation, consonant gemination is an essential auditory marker:
/// 1. Stop following `r` (repha) or `h` (hakāra) doubles:
///    - *arkaḥ* &rarr; *arkkaḥ*
///    - *dharmaḥ* &rarr; *dharmmaḥ*
///    - *brahma* &rarr; *brahmma* (TPr 14.4)
/// 2. Sibilant (*ś, ṣ, s*) preceded by vowel doubles before a stop or consonant (TPr 14.8):
///    - *aśvaḥ* &rarr; *aśśvaḥ*
/// 3. Non-aspirate stop following a vowel and preceding a vowel doubles under TPr 14.1.
pub fn should_double_in_taittiriya(
    prev: Option<&Varna>,
    curr: Consonant,
    next: Option<&Varna>,
) -> bool {
    let prev = match prev {
        Some(p) => p,
        None => return false,
    };

    // Rule 1 (TPr 14.4): Any stop or sibilant preceded by repha (r) or hakāra (h) doubles.
    if let Varna::Consonant(Consonant::R | Consonant::H) = prev {
        if curr.varga().is_some() || curr.is_ushman() {
            return true;
        }
    }

    // Rule 2 (TPr 14.8): Sibilant preceded by a vowel doubles before a consonant.
    if let Varna::Vowel(_) = prev {
        if matches!(curr, Consonant::Sh | Consonant::Ss | Consonant::S) {
            if let Some(Varna::Consonant(_)) = next {
                return true;
            }
        }
    }

    // Rule 3 (TPr 14.1): First consonant of a cluster preceded by a vowel doubles
    // if it is an unaspirated stop.
    if let Varna::Vowel(_) = prev {
        if curr.varga().is_some() && crate::pratisakhya::is_alpaprana(&curr) {
            if let Some(Varna::Consonant(next_c)) = next {
                if *next_c != curr {
                    return true;
                }
            }
        }
    }

    false
}

/// Checks if a nasal sound qualifies as *Raṅga* (musical nasal prolongation)
/// under *Taittirīya-Prātiśākhya* 17.1–4 and *Vyāsa Śikṣā*.
///
/// An Anunāsika vowel / Anusvāra preceding a sibilant (*ś, ṣ, s*) or *h*
/// undergoes prolonged nasal resonance (*raṅga-uccāraṇa*).
pub fn is_ranga_context(vowel: &Vowel, next_consonant: Consonant) -> bool {
    vowel.nasalized
        && matches!(
            next_consonant,
            Consonant::Sh | Consonant::Ss | Consonant::S | Consonant::H
        )
}

/// Returns the phonetic duration (*Mātrā*) of *Raṅga* according to *Taittirīya-Prātiśākhya* 17.2:
/// - If preceded by a short vowel (*hrasva-pūrva*): 1.0 mātrā
/// - If preceded by a long vowel (*dīrgha-pūrva*): 0.5 mātrā (*ardha-mātrā*)
pub fn ranga_duration_matra(preceding_vowel_length: VowelLength) -> f32 {
    match preceding_vowel_length {
        VowelLength::Hrasva => 1.0,
        VowelLength::Dirgha | VowelLength::Pluta => 0.5,
    }
}

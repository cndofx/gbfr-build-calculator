use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct SearchQuery {
    pub desired_traits: HashMap<Trait, u8>,
    pub sigil_slots: u8,
}

#[derive(Debug, Clone)]
pub struct SearchPool {
    pub sigils: Vec<Sigil>,
    pub wrightstones: Vec<Wrightstone>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub sigils: Vec<Sigil>,
    pub wrightstone: Option<Wrightstone>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sigil {
    pub trait1: (Trait, u8),
    pub trait2: Option<(Trait, u8)>,
}

#[derive(Debug, Clone)]
pub struct Wrightstone {
    pub trait1: (Trait, u8),
    pub trait2: Option<(Trait, u8)>,
    pub trait3: Option<(Trait, u8)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trait {
    Aegis,
    Alpha,
    ATKDownResistance,
    ATK,
    Autorevive,
    Berserker,
    Beta,
    BlightResistance,
    BreakAssassin,
    BurnResistance,
    ButterflysGrace,
    ButterflysValor,
    Cascade,
    Catastrophe,
    ChargedAttackDMG,
    ComboBooster,
    ComboFinisherDMG,
    ConcentratedFire,
    CrabvestmentReturns,
    CrimsonsClout,
    CrimsonsFlight,
    CriticalHitDMG,
    CriticalHitRate,
    DarkflameResistance,
    DEFDownDesistance,
    DizzyResistance,
    DMGCap,
    DodgePayback,
    DragonslayersDominance,
    DragonslayersIngenuity,
    Drain,
    EbonysPoise,
    EbonysPresence,
    Enmity,
    EternalRagesEthos,
    EternalRagesMettle,
    FastLearner,
    FearlessDrive,
    FearlessSpirit,
    FirmStance,
    FlightOverFight,
    FoundersStrategy,
    FoundersTruth,
    Gamma,
    Garrison,
    GlaciateResistance,
    GlassCannon,
    GuardiansConviction,
    GuardiansHonor,
    GuardPayback,
    Guts,
    HeldUnderResistance,
    HelmsmansNavigation,
    HelmsmansTenacity,
    HerosCreed,
    HerosWill,
    HolyKnightsGrandeur,
    HolyKnightsLuster,
    HP,
    ImprovedDodge,
    ImprovedGuard,
    ImprovedHealing,
    InjuryToInsult,
    LessIsMore,
    LifeOnTheLine,
    LinkedTogether,
    LordsAmbition,
    LordsProcession,
    LowProfile,
    LuckyCharge,
    MagesAspiration,
    MagesSavvy,
    NaturalDefenses,
    NimbleDefense,
    NimbleOnslaught,
    OverdriveAssassin,
    ParalysisResistance,
    PathToMastery,
    PhantasmsConcord,
    PhantasmsHarmony,
    PoisonResistance,
    PotentGreens,
    PotionHoarder,
    PowerHungry,
    PreciseResilience,
    PreciseWrath,
    Provoke,
    QuickCharge,
    QuickCooldown,
    Regen,
    RollOfTheDie,
    RosesBlooming,
    RosesProfusion,
    RupieTycoon,
    SandtombResistance,
    SBASealedResistance,
    SigilBooster,
    SkilledAssault,
    SkillSealedResistance,
    SlowResistance,
    Stamina,
    SteadyFocus,
    SteelNerves,
    StoutHeart,
    StunPower,
    SupplementaryDamage,
    SwordmastersArt,
    SwordmastersProwess,
    ThrowDMG,
    Tyranny,
    Uplift,
    VersalisFoundation,
    VersalisIgnition,
    VeteransInsight,
    VeteransVision,
    WarElemental,
    WeakPointDMG,
    WhiteDragonsGlory,
    WhiteDragonsOath,
    // CrabbyResonance, // special case, deal with it later
    // unsure if the following traits exist:
    // fortifying vigor
    // instilling vigor
    // gilding vigor
    // seven net
    // stronghold
    // head start
}

impl Sigil {
    pub fn new_single(trait1: Trait, level: u8) -> Self {
        Sigil {
            trait1: (trait1, level),
            trait2: None,
        }
    }
}

impl FromStr for Trait {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Trait as T;
        match s {
            "Hero's Creed" => Ok(T::HerosCreed),
            "Hero's Will" => Ok(T::HerosWill),
            "Alpha" => Ok(T::Alpha),
            "Beta" => Ok(T::Beta),
            "Gamma" => Ok(T::Gamma),
            "War Elemental" => Ok(T::WarElemental),
            "Supplementary DMG" => Ok(T::SupplementaryDamage),
            "Aegis" => Ok(T::Aegis),
            "Potion Hoarder" => Ok(T::PotionHoarder),
            "Tyranny" => Ok(T::Tyranny),
            "Quick Cooldown" => Ok(T::QuickCooldown),
            "DMG Cap" => Ok(T::DMGCap),
            "Improved Guard" => Ok(T::ImprovedGuard),
            "Critical Hit Rate" => Ok(T::CriticalHitRate),
            "Autorevive" => Ok(T::Autorevive),
            "Drain" => Ok(T::Drain),
            _ => Err("invalid trait name"),
        }
    }
}

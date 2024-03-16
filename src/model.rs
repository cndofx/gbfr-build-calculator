use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchQuery {
    // pub desired_traits: Vec<(Trait, u8)>,
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

#[derive(Debug, Clone)]
pub struct Sigil {
    pub level: u8,
    pub trait1: Trait,
    pub trait2: Option<Trait>,
}

#[derive(Debug, Clone)]
pub struct Wrightstone {
    pub trait1: Trait,
    pub trait2: Option<Trait>,
    pub trait3: Option<Trait>,
    pub trait1_level: u8,
    pub trait2_level: u8,
    pub trait3_level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trait {
    Aegis,
    ATKDownResistance,
    ATK,
    Autorevive,
    Berserker,
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
            level,
            trait1,
            trait2: None,
        }
    }
}

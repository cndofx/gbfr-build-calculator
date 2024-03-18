use std::collections::hash_map::Iter as HashMapIter;
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct TraitSet {
    traits: HashMap<TraitKind, u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Trait {
    pub kind: TraitKind,
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitKind {
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
    CrabbyResonance,
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
    // unsure if the following traits exist:
    // fortifying vigor
    // instilling vigor
    // gilding vigor
    // seven net
    // stronghold
    // head start
}

impl Trait {
    pub fn new(kind: TraitKind, level: u8) -> Self {
        Trait { kind, level }
    }
}

impl TraitSet {
    pub fn new() -> Self {
        TraitSet {
            traits: HashMap::new(),
        }
    }

    pub fn add(&mut self, t: Trait) {
        self.traits
            .entry(t.kind)
            .and_modify(|l| *l += t.level as u16)
            .or_insert(t.level as u16);
    }

    pub fn level(&self, kind: TraitKind) -> u16 {
        self.traits.get(&kind).copied().unwrap_or(0)
    }

    pub fn contains(&self, kind: TraitKind) -> bool {
        self.traits.contains_key(&kind)
    }

    pub fn is_superset_of(&self, other: &TraitSet) -> bool {
        other.iter().all(|(desired_k, desired_l)| {
            self.traits.get(desired_k).is_some_and(|l| l >= desired_l)
        })
    }

    pub fn iter(&self) -> HashMapIter<'_, TraitKind, u16> {
        self.traits.iter()
    }
}

impl From<HashMap<TraitKind, u16>> for TraitSet {
    fn from(value: HashMap<TraitKind, u16>) -> Self {
        TraitSet { traits: value }
    }
}

impl FromIterator<Trait> for TraitSet {
    fn from_iter<T: IntoIterator<Item = Trait>>(iter: T) -> Self {
        let mut traits = TraitSet::new();
        for t in iter {
            traits.add(t);
        }
        traits
    }
}

impl Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind, self.level)
    }
}

impl FromStr for TraitKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TraitKind::*;
        match s {
            "Aegis" => Ok(Aegis),
            "Alpha" => Ok(Alpha),
            "ATK Down Resistance" => Ok(ATKDownResistance),
            "ATK" => Ok(ATK),
            "Autorevive" => Ok(Autorevive),
            "Berserker" => Ok(Berserker),
            "Beta" => Ok(Beta),
            "Blight Resistance" => Ok(BlightResistance),
            "Break Assassin" => Ok(BreakAssassin),
            "Burn Resistance" => Ok(BurnResistance),
            "Butterfly's Grace" => Ok(ButterflysGrace),
            "Butterfly's Valor" => Ok(ButterflysValor),
            "Cascade" => Ok(Cascade),
            "Catastrophe" => Ok(Catastrophe),
            "Charged Attack DMG" => Ok(ChargedAttackDMG),
            "Combo Booster" => Ok(ComboBooster),
            "Combo Finisher DMG" => Ok(ComboFinisherDMG),
            "Concentrated Fire" => Ok(ConcentratedFire),
            "Crabvestment Returns" => Ok(CrabvestmentReturns),
            "Crabby Resonance" => Ok(CrabbyResonance),
            "Crimson's Clout" => Ok(CrimsonsClout),
            "Crimson's Flight" => Ok(CrimsonsFlight),
            "Critical Hit DMG" => Ok(CriticalHitDMG),
            "Critical Hit Rate" => Ok(CriticalHitRate),
            "Darkflame Resistance" => Ok(DarkflameResistance),
            "DEF Down Resistance" => Ok(DEFDownDesistance),
            "Dizzy Resistance" => Ok(DizzyResistance),
            "DMG Cap" => Ok(DMGCap),
            "Dodge Payback" => Ok(DodgePayback),
            "Dragonslayer's Dominance" => Ok(DragonslayersDominance),
            "Dragonslayer's Ingenuity" => Ok(DragonslayersIngenuity),
            "Drain" => Ok(Drain),
            "Ebony's Poise" => Ok(EbonysPoise),
            "Ebony's Presence" => Ok(EbonysPresence),
            "Enmity" => Ok(Enmity),
            "Eternal Rage's Ethos" => Ok(EternalRagesEthos),
            "Eternal Rage's Mettle" => Ok(EternalRagesMettle),
            "Fast Learner" => Ok(FastLearner),
            "Fearless Drive" => Ok(FearlessDrive),
            "Fearless Spirit" => Ok(FearlessSpirit),
            "Firm Stance" => Ok(FirmStance),
            "Flight Over Fight" => Ok(FlightOverFight),
            "Founder's Strategy" => Ok(FoundersStrategy),
            "Founder's Truth" => Ok(FoundersTruth),
            "Gamma" => Ok(Gamma),
            "Garrison" => Ok(Garrison),
            "Glaciate Resistance" => Ok(GlaciateResistance),
            "Glass Cannon" => Ok(GlassCannon),
            "Guardian's Conviction" => Ok(GuardiansConviction),
            "Guardian's Honor" => Ok(GuardiansHonor),
            "Guard Payback" => Ok(GuardPayback),
            "Guts" => Ok(Guts),
            "Held Under Resistance" => Ok(HeldUnderResistance),
            "Helmsman's Navigation" => Ok(HelmsmansNavigation),
            "Helmsman's Tenacity" => Ok(HelmsmansTenacity),
            "Hero's Creed" => Ok(HerosCreed),
            "Hero's Will" => Ok(HerosWill),
            "Holy Knight's Grandeur" => Ok(HolyKnightsGrandeur),
            "Holy Knight's Luster" => Ok(HolyKnightsLuster),
            "HP" => Ok(HP),
            "Improved Dodge" => Ok(ImprovedDodge),
            "Improved Guard" => Ok(ImprovedGuard),
            "Improved Healing" => Ok(ImprovedHealing),
            "Injury To Insult" => Ok(InjuryToInsult),
            "Less Is More" => Ok(LessIsMore),
            "Life On The Line" => Ok(LifeOnTheLine),
            "LinkedTogether" => Ok(LinkedTogether),
            "Lord's Ambition" => Ok(LordsAmbition),
            "Lord's Procession" => Ok(LordsProcession),
            "Low Profile" => Ok(LowProfile),
            "Lucky Charge" => Ok(LuckyCharge),
            "Mage's Aspiration" => Ok(MagesAspiration),
            "Mage's Savvy" => Ok(MagesSavvy),
            "Natural Defenses" => Ok(NaturalDefenses),
            "Nimble Defense" => Ok(NimbleDefense),
            "Nimble Onslaught" => Ok(NimbleOnslaught),
            "Overdrive Assassin" => Ok(OverdriveAssassin),
            "Paralysis Resistance" => Ok(ParalysisResistance),
            "Path To Mastery" => Ok(PathToMastery),
            "Phantasm's Concord" => Ok(PhantasmsConcord),
            "Phantasm's Harmony" => Ok(PhantasmsHarmony),
            "Potion Hoarder" => Ok(PotionHoarder),
            "Power Hungry" => Ok(PowerHungry),
            "Precise Resilience" => Ok(PreciseResilience),
            "Precise Wrath" => Ok(PreciseWrath),
            "Provoke" => Ok(Provoke),
            "Quick Charge" => Ok(QuickCharge),
            "Quick Cooldown" => Ok(QuickCooldown),
            "Regen" => Ok(Regen),
            "Roll Of The Die" => Ok(RollOfTheDie),
            "Rose's Blooming" => Ok(RosesBlooming),
            "Rose's Profusion" => Ok(RosesProfusion),
            "Rupie Tycoon" => Ok(RupieTycoon),
            "Sandtomb Resistance" => Ok(SandtombResistance),
            "SBA Sealed Resistance" => Ok(SBASealedResistance),
            "Sigil Booster" => Ok(SigilBooster),
            "Skilled Assault" => Ok(SkilledAssault),
            "Skill Sealed Resistance" => Ok(SkillSealedResistance),
            "Slow Resistance" => Ok(SlowResistance),
            "Stamina" => Ok(Stamina),
            "Steady Focus" => Ok(SteadyFocus),
            "Steel Nerves" => Ok(SteelNerves),
            "Stout Heart" => Ok(StoutHeart),
            "Stun Power" => Ok(StunPower),
            "Supplementary DMG" => Ok(SupplementaryDamage),
            "Swordmaster's Art" => Ok(SwordmastersArt),
            "Swordmaster's Prowess" => Ok(SwordmastersProwess),
            "Throw DMG" => Ok(ThrowDMG),
            "Tyranny" => Ok(Tyranny),
            "Uplift" => Ok(Uplift),
            "Versalis Foundation" => Ok(VersalisFoundation),
            "Versalis Ignition" => Ok(VersalisIgnition),
            "Veteran's Insight" => Ok(VeteransInsight),
            "Veteran's Vision" => Ok(VeteransVision),
            "War Elemental" => Ok(WarElemental),
            "Weak Point DMG" => Ok(WeakPointDMG),
            "White Dragon's Glory" => Ok(WhiteDragonsGlory),
            "White Dragon's Oath" => Ok(WhiteDragonsOath),
            _ => Err("invalid trait name"),
        }
    }
}

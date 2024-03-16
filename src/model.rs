use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

impl SearchResult {
    pub fn trait_level(&self, t: Trait) -> u16 {
        let mut sum = 0;

        for sigil in &self.sigils {
            let (t1, l1) = sigil.trait1;
            if t1 == t {
                sum += l1 as u16;
            }

            if let Some((t2, l2)) = sigil.trait2 {
                if t2 == t {
                    sum += l2 as u16;
                }
            }
        }

        if let Some(Wrightstone {
            trait1,
            trait2,
            trait3,
        }) = &self.wrightstone
        {
            let (t1, l1) = trait1;
            if *t1 == t {
                sum += *l1 as u16;
            }

            if let Some((t2, l2)) = trait2 {
                if *t2 == t {
                    sum += *l2 as u16;
                }
            }

            if let Some((t3, l3)) = trait3 {
                if *t3 == t {
                    sum += *l3 as u16;
                }
            }
        }

        sum
    }

    pub fn traits(&self) -> Vec<Trait> {
        let mut traits = HashSet::new();

        for sigil in &self.sigils {
            let (t1, _) = sigil.trait1;
            traits.insert(t1);
            if let Some((t2, _)) = sigil.trait2 {
                traits.insert(t2);
            }
        }

        if let Some(Wrightstone {
            trait1,
            trait2,
            trait3,
        }) = &self.wrightstone
        {
            let (t1, _) = trait1;
            traits.insert(*t1);

            if let Some((t2, _)) = trait2 {
                traits.insert(*t2);
            }

            if let Some((t3, _)) = trait3 {
                traits.insert(*t3);
            }
        }

        traits.iter().copied().collect()
    }

    pub fn total_trait_level(&self) -> u16 {
        self.traits().into_iter().map(|t| self.trait_level(t)).sum()
    }
}

impl Sigil {
    pub fn new_single(trait1: Trait, level: u8) -> Self {
        Sigil {
            trait1: (trait1, level),
            trait2: None,
        }
    }
}

impl Display for Sigil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (t1, l1) = self.trait1;
        if let Some((t2, l2)) = self.trait2 {
            write!(f, "[{t1:?} {l1} + {t2:?} {l2}]")
        } else {
            write!(f, "[{t1:?} {l1}]")
        }
    }
}

impl Display for Wrightstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (t1, l1) = self.trait1;
        write!(f, "[{t1:?} {l1}")?;
        if let Some((t2, l2)) = self.trait2 {
            write!(f, " + {t2:?} {l2}")?;
        }
        if let Some((t3, l3)) = self.trait3 {
            write!(f, " + {t3:?} {l3}")?;
        }
        write!(f, "]")
    }
}

impl FromStr for Trait {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Trait::*;
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

// Copyright (C) 2024 Bjarne Seger
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0.txt>.

#[derive(Debug, rand_derive2::RandGen,Eq, PartialEq, strum_macros::EnumString,
         strum_macros::Display, Copy, Clone)]
pub enum Stratagem {
    // Mission Objectives
    Resupply,
    SOSBeacon,
    Reinforce,
    Hellbomb,
    SSSDDelivery,
    UploadData,
    EagleRearm,
    SeismicProbe,
    OrbitalIlluminationFlare,
    SEAFArtillery,

    // Supply: Backpacks
    LAS5GuardDogRover,
    AR23GuardDog,
    LIFT850JumpPack,
    B1SupplyPack,
    SH32ShieldGeneratorPack,
    SH20BallisticShieldBackpack,

    // Support: Support Weapons
    AC8Autocannon,
    EAT17ExpendableAntiTank,
    FLAM40Flamethrower,
    LAS98LaserCannon,
    M105Stalwart,
    MG43MachineGun,
    ARC3ArcThrower,
    GL21GrenadeLauncher,
    APW1AntiMaterielRifle,
    RS422Railgun,
    GR8RecoillessRifle,
    FAF14SPEARLauncher,
    LAS99QuasarCannon,
    MG206HeavyMachineGun,

    // Supply: Vehicles
    EXO45PatriotExosuit,

    // Defensive
    ARC3TeslaTower,
    M12MortarSentry,
    M23EMSMortarSentry,
    MG43MachineGunSentry,
    G16GatlingSentry,
    MD6AntiPersonnelMinefield,
    MD14IncendiaryMines,
    FX12ShieldGeneratorRelay,
    MG101HMGEmplacement,
    AC8AutocannonSentry,
    MLS4XRocketSentry,

    // Offensive: Orbital
    OrbitalPrecisionStrike,
    OrbitalAirburstStrike,
    Orbital120MMHEBarrage,
    Orbital380MMHEBarrage,
    OrbitalWalkingBarrage,
    OrbitalLaser,
    OrbitalRailcannonStrike,
    OrbitalGatlingBarrage,
    OrbitalGasStrike,
    OrbitalEMSStrike,
    OrbitalSmokeStrike,

    // Offensive: Eagle
    EagleStrafingRun,
    EagleAirstrike,
    EagleClusterBomb,
    EagleNapalmStrike,
    EagleSmokeStrike,
    Eagle110MMRocketPods,
    Eagle500kgBomb
}

impl Stratagem {
    // I hate this, but this is the easiest workaround to the MutexGuard
    pub fn get_keycode(&self) -> Vec<Code> {
        use Stratagem::*;
        use Code::*;
        match self {
            Resupply => vec!(Down, Down, Up, Right),
            SOSBeacon => vec!(Up, Down, Right, Up),
            Reinforce => vec!(Up, Down, Right, Left, Up),
            Hellbomb => vec!(Down, Up, Left, Down, Up, Right, Down, Up),
            SSSDDelivery => vec!(Down, Down, Down, Up, Up),
            UploadData => vec!(Down, Down, Up, Up, Up),
            EagleRearm => vec!(Up, Up, Left, Up, Right),
            SeismicProbe => vec!(Up, Up, Left, Right, Down, Down),
            OrbitalIlluminationFlare => vec!(Right, Right, Left, Left),
            SEAFArtillery => vec!(Right, Up, Up, Down),

            LAS5GuardDogRover => vec!(Down, Up, Left, Up, Right, Right),
            AR23GuardDog => vec!(Down, Up, Left, Up, Right, Down),
            LIFT850JumpPack => vec!(Down, Up, Up, Down, Up),
            B1SupplyPack => vec!(Down, Left, Down, Up, Up, Down),
            SH32ShieldGeneratorPack => vec!(Down, Up, Left, Right, Left, Right),
            SH20BallisticShieldBackpack => vec!(Down, Left, Down, Down, Up, Left),
            
            AC8Autocannon => vec!(Down, Left, Down, Up, Up, Right),
            EAT17ExpendableAntiTank => vec!(Down, Down, Left, Up, Right),
            FLAM40Flamethrower => vec!(Down, Left, Up, Down, Up),
            LAS98LaserCannon => vec!(Down, Left, Down, Up, Left),
            M105Stalwart => vec!(Down, Left, Down, Up, Up, Right),
            MG43MachineGun => vec!(Down, Left, Down, Up, Right),
            ARC3ArcThrower => vec!(Down, Right, Down, Up, Left, Left),
            GL21GrenadeLauncher => vec!(Down, Left, Up, Left, Down),
            APW1AntiMaterielRifle => vec!(Down, Left, Right, Up, Down),
            RS422Railgun => vec!(Down, Right, Down, Up, Left, Right),
            GR8RecoillessRifle => vec!(Down, Left, Right, Right, Left),
            FAF14SPEARLauncher => vec!(Down, Down, Up, Down, Down),
            LAS99QuasarCannon => vec!(Down, Down, Up, Left, Right),
            MG206HeavyMachineGun => vec!(Down, Left, Up, Down, Down),

            EXO45PatriotExosuit => vec!(Left, Down, Right, Up, Left, Down, Down),

            ARC3TeslaTower => vec!(Down, Up, Right, Up, Left, Right),
            M12MortarSentry => vec!(Down, Up, Right, Right, Down),
            M23EMSMortarSentry => vec!(Down, Up, Right, Down, Right),
            MG43MachineGunSentry => vec!(Down, Up, Right, Right, Up),
            G16GatlingSentry => vec!(Down, Up, Right, Left),
            MD6AntiPersonnelMinefield => vec!(Down, Left, Up, Right),
            MD14IncendiaryMines => vec!(Down, Left, Left, Down),
            FX12ShieldGeneratorRelay => vec!(Down, Down, Left, Right, Left, Right),
            MG101HMGEmplacement => vec!(Down, Up, Left, Right, Right, Left),
            AC8AutocannonSentry => vec!(Down, Up, Right, Up, Left, Up),
            MLS4XRocketSentry => vec!(Down, Up, Right, Right, Left),

            OrbitalPrecisionStrike => vec!(Right, Right, Up),
            OrbitalAirburstStrike => vec!(Right, Right, Right),
            Orbital120MMHEBarrage => vec!(Right, Right, Down, Left, Right, Down),
            Orbital380MMHEBarrage => vec!(Right, Down, Up, Up, Left, Down, Down),
            OrbitalWalkingBarrage => vec!(Right, Down, Right, Down, Right, Down),
            OrbitalLaser => vec!(Right, Down, Up, Right, Down),
            OrbitalRailcannonStrike => vec!(Right, Up, Down, Down, Right),
            OrbitalGatlingBarrage => vec!(Right, Down, Left, Up, Up),
            OrbitalGasStrike => vec!(Right, Right, Down, Right),
            OrbitalEMSStrike => vec!(Right, Right, Left, Down),
            OrbitalSmokeStrike => vec!(Right, Right, Down, Up),

            EagleStrafingRun => vec!(Up, Right, Right),
            EagleAirstrike => vec!(Up, Right, Down, Right),
            EagleClusterBomb => vec!(Up, Right, Down, Down, Right),
            EagleNapalmStrike => vec!(Up, Right, Down, Up),
            EagleSmokeStrike => vec!(Up, Right, Up, Down),
            Eagle110MMRocketPods => vec!(Up, Right, Up, Left),
            Eagle500kgBomb => vec!(Up, Right, Down, Down, Down),
            
        }
    }

    pub fn get_image_path(&self) -> String {
        use Stratagem::*;
        let asset_folder_path = "assets/Helldivers-2-Stratagems-icons-svg/";
        let extension = match self {
            Resupply => "GeneralStratagems/Resupply.svg",
            SOSBeacon => "GeneralStratagems/SOS Beacon.svg",
            Reinforce => "GeneralStratagems/Reinforce.svg",
            Hellbomb => "GeneralStratagems/Hellbomb.svg",
            // They are the same
            SSSDDelivery => "GeneralStratagems/Upload Data.svg",
            UploadData => "GeneralStratagems/Upload Data.svg",
            EagleRearm => "Hangar/EagleRearm.svg",
            SeismicProbe => "GeneralStratagems/Seismic Probe.svg",
            OrbitalIlluminationFlare => "GeneralStratagems/Orbital Illumination Flare.svg",
            SEAFArtillery => "GeneralStratagems/SEAF Artillery.svg",

            LAS5GuardDogRover => "EngineeringBay/Guard Dog Rover.svg",
            AR23GuardDog => "RoboticsWorkshop/Guard Dog.svg",
            LIFT850JumpPack => "Hangar/Jump Pack.svg",
            B1SupplyPack => "EngineeringBay/Supply Pack.svg",
            SH32ShieldGeneratorPack => "EngineeringBay/Shield Generator Pack.svg",
            SH20BallisticShieldBackpack => "EngineeringBay/Ballistic Shield Backpack.svg",

            AC8Autocannon => "PatrioticAdministrationCenter/Autocannon.svg",
            EAT17ExpendableAntiTank => "PatrioticAdministrationCenter/Expendable Anti-Tank.svg",
            FLAM40Flamethrower => "PatrioticAdministrationCenter/Flamethrower.svg",
            LAS98LaserCannon => "EngineeringBay/Laser Cannon.svg",
            M105Stalwart => "PatrioticAdministrationCenter/Stalwart.svg",
            MG43MachineGun => "PatrioticAdministrationCenter/Machine Gun.svg",
            ARC3ArcThrower => "EngineeringBay/Arc Thrower.svg",
            GL21GrenadeLauncher => "EngineeringBay/Grenade Launcher",
            APW1AntiMaterielRifle => "PatrioticAdministrationCenter/Anti-Materiel Rifle.svg",
            RS422Railgun => "PatrioticAdministrationCenter/Railgun.svg",
            GR8RecoillessRifle => "PatrioticAdministrationCenter/Recoilless Rifle.svg",
            FAF14SPEARLauncher => "PatrioticAdministrationCenter/Spear.svg",
            LAS99QuasarCannon => "EngineeringBay/Quasar Cannon.svg",
            MG206HeavyMachineGun => "PatrioticAdministrationCenter/Heavy Machine Gun.svg",

            EXO45PatriotExosuit => "RoboticsWorkshop/Patriotic Exosuit.svg",

            ARC3TeslaTower => "Bridge/Tesla Tower.svg",
            M12MortarSentry => "RoboticsWorkshop/Mortar Sentry.svg",
            M23EMSMortarSentry => "RoboticsWorkshop/EMS Mortar Sentry.svg",
            MG43MachineGunSentry => "RoboticsWorkshop/Machine Gun Sentry.svg",
            G16GatlingSentry => "RoboticsWorkshop/Gatling Sentry.svg",
            MD6AntiPersonnelMinefield => "EngineeringBay/Anti-Personnel Minefield.svg",
            MD14IncendiaryMines => "EngineeringBay/Incendiary Mines.svg",
            FX12ShieldGeneratorRelay => "Bridge/Shield Generator Relay.svg",
            MG101HMGEmplacement => "Bridge/HMG Emplacement.svg",
            AC8AutocannonSentry => "RoboticsWorkshop/Autocannon Sentry.svg",
            MLS4XRocketSentry => "RoboticsWorkshop/Rocket Sentry.svg",

            OrbitalPrecisionStrike => "Bridge/Orbital Precision Strike.svg",
            OrbitalAirburstStrike => "OrbitalCannons/Orbital Airburst Strike.svg",
            Orbital120MMHEBarrage => "OrbitalCannons/Orbital 120MM HE Barrage.svg",
            Orbital380MMHEBarrage => "OrbitalCannons/Orbital 380MM HE Barrage.svg",
            OrbitalWalkingBarrage => "OrbitalCannons/Orbital Walking Barrage.svg",
            OrbitalLaser => "OrbitalCannons/Orbital Laser.svg",
            OrbitalRailcannonStrike => "OrbitalCannons/Orbital Railcannon Strike.svg",
            OrbitalGatlingBarrage => "OrbitalCannons/Orbital Gatling Barrage.svg",
            OrbitalGasStrike => "Bridge/Orbital Gas Strike.svg",
            OrbitalEMSStrike => "Bridge/Orbital EMS Strike.svg",
            OrbitalSmokeStrike => "Bridge/Orbital Smoke Strike.svg",

            EagleStrafingRun => "Hangar/Eagle Strafing Run.svg",
            EagleAirstrike => "Hangar/Eagle Airstrike.svg",
            EagleClusterBomb => "Hangar/Eagle Cluster Bomb.svg",
            EagleNapalmStrike => "Hangar/Eagle Napalm Strike.svg",
            EagleSmokeStrike => "Hangar/Eagle Smoke Strike.svg",
            Eagle110MMRocketPods => "Hangar/Eagle 110MM Rocket Pods.svg",
            Eagle500kgBomb => "Hangar/Eagle 500KG Bomb.svg",
        };
        asset_folder_path.to_owned() + extension
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Code {
    Down,
    Left,
    Right,
    Up
}

impl Code {
    pub fn get_arrow(&self) -> &str {
        use Code::*;
        match self {
            Down => "↓",
            Left => "←",
            Right => "→",
            Up => "↑",
        }
    }
}

//! Integration tests for dftd4 interface.
//!
//! However, perhaps some functions can be captured in documentation.
//! So this file is here.
#![allow(clippy::excessive_precision)]

use approx::assert_abs_diff_eq;
use dftd4::prelude::*;

fn test_rational_damping_noargs() {
    // Check constructor of damping parameters for insufficient arguments
    assert!(DFTD4RationalDampingParamBuilder::default().build().is_err());

    let builder = DFTD4RationalDampingParamBuilder::default().a1(0.4).a2(5.0);
    assert!(builder.build().is_err_and(|e| match &e {
        DFTD4Error::BuilderError(f) => f.field_name() == "s8",
        _ => false,
    }));

    let builder = DFTD4RationalDampingParamBuilder::default().s8(1.0).a2(5.0);
    assert!(builder.build().is_err_and(|e| match &e {
        DFTD4Error::BuilderError(f) => f.field_name() == "a1",
        _ => false,
    }));

    let builder = DFTD4RationalDampingParamBuilder::default().s8(1.0).a1(0.4);
    assert!(builder.build().is_err_and(|e| match &e {
        DFTD4Error::BuilderError(f) => f.field_name() == "a2",
        _ => false,
    }));
}

fn test_structure() {
    // Check if the molecular structure data is working as expected
    let numbers = vec![6, 7, 6, 7, 6, 6, 6, 8, 7, 6, 8, 7, 6, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    #[rustfmt::skip]
    let positions = vec![
         2.02799738646442,  0.09231312124713, -0.14310895950963,
         4.75011007621000,  0.02373496014051, -0.14324124033844,
         6.33434307654413,  2.07098865582721, -0.14235306905930,
         8.72860718071825,  1.38002919517619, -0.14265542523943,
         8.65318821103610, -1.19324866489847, -0.14231527453678,
         6.23857175648671, -2.08353643730276, -0.14218299370797,
         5.63266886875962, -4.69950321056008, -0.13940509630299,
         3.44931709749015, -5.48092386085491, -0.14318454855466,
         7.77508917214346, -6.24427872938674, -0.13107140408805,
        10.30229550927022, -5.39739796609292, -0.13672168520430,
        12.07410272485492, -6.91573621641911, -0.13666499342053,
        10.70038521493902, -2.79078533715849, -0.14148379504141,
        13.24597858727017, -1.76969072232377, -0.14218299370797,
         7.40891694074004, -8.95905928176407, -0.11636933482904,
         1.38702118184179,  2.05575746325296, -0.14178615122154,
         1.34622199478497, -0.86356704498496,  1.55590600570783,
         1.34624089204623, -0.86133716815647, -1.84340893849267,
         5.65596919189118,  4.00172183859480, -0.14131371969009,
        14.67430918222276, -3.26230980007732, -0.14344911021228,
        13.50897177220290, -0.60815166181684,  1.54898960808727,
        13.50780014200488, -0.60614855212345, -1.83214617078268,
         5.41408424778406, -9.49239668625902, -0.11022772492007,
         8.31919801555568, -9.74947502841788,  1.56539243085954,
         8.31511620712388, -9.76854236502758, -1.79108242206824,
    ];

    // Constructor should raise an error for nuclear fusion input
    assert!(DFTD4Structure::new_f(&numbers, &vec![0.0; 72], None, None, None)
        .is_err_and(|e| e.to_string().contains("Too close interatomic distances found")));

    // The Rust struct should protect from garbage input like this
    assert!(DFTD4Structure::new_f(&[1, 1, 1], &positions, None, None, None)
        .is_err_and(|e| e.to_string().contains("Invalid dimension for positions")));

    // Also check for sane coordinate input
    assert!(DFTD4Structure::new_f(&numbers, &[0.0; 7], None, None, None)
        .is_err_and(|e| e.to_string().contains("Invalid dimension for positions")));

    // Construct real molecule
    let mut mol = DFTD4Structure::new(&numbers, &positions, None, None, None);

    // Try to update a structure with mismatched coordinates
    assert!(mol
        .update_f(&[0.0; 7], None)
        .is_err_and(|e| e.to_string().contains("Invalid dimension for positions")));

    // Try to add a mismatched lattice
    assert!(mol
        .update_f(&positions, Some(&[0.0; 7]))
        .is_err_and(|e| e.to_string().contains("Invalid dimension for lattice")));

    // Try to update a structure with nuclear fusion coordinates
    assert!(mol
        .update_f(&[0.0; 72], None)
        .is_err_and(|e| e.to_string().contains("Too close interatomic distances found"),));
}

fn test_blypd4() {
    // Use BLYP-D4 for a mindless molecule
    let thr = 1.0e-7;

    let numbers = vec![1, 1, 6, 5, 1, 15, 8, 17, 13, 15, 5, 1, 9, 15, 1, 15];
    #[rustfmt::skip]
    let positions = vec![
         2.79274810283778,  3.82998228828316, -2.79287054959216,
        -1.43447454186833,  0.43418729987882,  5.53854345129809,
        -3.26268343665218, -2.50644032426151, -1.56631149351046,
         2.14548759959147, -0.88798018953965, -2.24592534506187,
        -4.30233097423181, -3.93631518670031, -0.48930754109119,
         0.06107643564880, -3.82467931731366, -2.22333344469482,
         0.41168550401858,  0.58105573172764,  5.56854609916143,
         4.41363836635653,  3.92515871809283,  2.57961724984000,
         1.33707758998700,  1.40194471661647,  1.97530004949523,
         3.08342709834868,  1.72520024666801, -4.42666116106828,
        -3.02346932078505,  0.04438199934191, -0.27636197425010,
         1.11508390868455, -0.97617412809198,  6.25462847718180,
         0.61938955433011,  2.17903547389232, -6.21279842416963,
        -2.67491681346835,  3.00175899761859,  1.05038813614845,
        -4.13181080289514, -2.34226739863660, -3.44356159392859,
         2.85007173009739, -2.64884892757600,  0.71010806424206,
    ];

    let model = DFTD4Model::new(&numbers, &positions, None, None, None);

    let param = DFTD4Param::load_rational_damping("blyp", true);
    let res = model.get_dispersion(&param, false);

    assert_abs_diff_eq!(res.energy, -0.06991716314879085, epsilon = thr);

    let res = model.get_dispersion(&param, true);

    assert_abs_diff_eq!(res.energy, -0.06991716314879085, epsilon = thr);
}

#[cfg(feature = "api-v4_0")]
fn test_tpssd4s() {
    // Use TPSS-D4S for a mindless molecule
    let thr = 1.0e-7;

    let numbers = vec![1, 1, 6, 5, 1, 15, 8, 17, 13, 15, 5, 1, 9, 15, 1, 15];
    #[rustfmt::skip]
    let positions = vec![
         2.79274810283778,  3.82998228828316, -2.79287054959216,
        -1.43447454186833,  0.43418729987882,  5.53854345129809,
        -3.26268343665218, -2.50644032426151, -1.56631149351046,
         2.14548759959147, -0.88798018953965, -2.24592534506187,
        -4.30233097423181, -3.93631518670031, -0.48930754109119,
         0.06107643564880, -3.82467931731366, -2.22333344469482,
         0.41168550401858,  0.58105573172764,  5.56854609916143,
         4.41363836635653,  3.92515871809283,  2.57961724984000,
         1.33707758998700,  1.40194471661647,  1.97530004949523,
         3.08342709834868,  1.72520024666801, -4.42666116106828,
        -3.02346932078505,  0.04438199934191, -0.27636197425010,
         1.11508390868455, -0.97617412809198,  6.25462847718180,
         0.61938955433011,  2.17903547389232, -6.21279842416963,
        -2.67491681346835,  3.00175899761859,  1.05038813614845,
        -4.13181080289514, -2.34226739863660, -3.44356159392859,
         2.85007173009739, -2.64884892757600,  0.71010806424206,
    ];

    let model = DFTD4Model::new_d4s(&numbers, &positions, None, None, None);

    let param = DFTD4Param::load_rational_damping("tpss", false);
    let res = model.get_dispersion(&param, false);

    assert_abs_diff_eq!(res.energy, -0.046233140236052253, epsilon = thr);

    let res = model.get_dispersion(&param, true);

    assert_abs_diff_eq!(res.energy, -0.046233140236052253, epsilon = thr);
}

fn test_pbed4() {
    // Use PBE-D4 for a mindless molecule
    let thr = 1.0e-7;

    let numbers = vec![1, 9, 15, 13, 1, 1, 13, 5, 3, 15, 8, 1, 1, 5, 16, 1];
    #[rustfmt::skip]
    let positions = vec![
        -2.14132037405479, -1.34402701877044, -2.32492500904728,
         4.46671289205392, -2.04800110524830,  0.44422406067087,
        -4.92212517643478, -1.73734240529793,  0.96890323821450,
        -1.30966093045696, -0.52977363497805,  3.44453452239668,
        -4.34208759006189, -4.30470270977329,  0.39887431726215,
         0.61788392767516,  2.62484136683297, -3.28228926932647,
         4.23562873444840, -1.68839322682951, -3.53824299552792,
         2.23130060612446,  1.93579813100155, -1.80384647554323,
        -2.32285463652832,  2.90603947535842, -1.39684847191937,
         2.34557941578250,  2.86074312333371,  1.82827238641666,
        -3.66431367659153, -0.42910188232667, -1.81957402856634,
        -0.34927881505446, -1.75988134003940,  5.98017466326572,
         0.29500802281217, -2.00226104143537,  0.53023447931897,
         2.10449364205058, -0.56741404446633,  0.30975625014335,
        -1.59355304432499,  3.69176153150419,  2.87878226787916,
         4.34858700256050,  2.39171478113440, -2.61802993563738,
    ];

    let model = DFTD4Model::new(&numbers, &positions, None, None, None);

    let param = DFTD4Param::load_rational_damping("pbe", true);
    let res = model.get_dispersion(&param, false);

    assert_abs_diff_eq!(res.energy, -0.028415184156428127, epsilon = thr);

    let res = model.get_dispersion(&param, true);

    assert_abs_diff_eq!(res.energy, -0.028415184156428127, epsilon = thr);
}

#[cfg(feature = "api-v3_1")]
fn test_r2scan3c() {
    // Use r2SCAN-3c for a mindless molecule (custom D4 model)
    let thr = 1.0e-8;

    let numbers = vec![1, 9, 15, 13, 1, 1, 13, 5, 3, 15, 8, 1, 1, 5, 16, 1];
    #[rustfmt::skip]
    let positions = vec![
        -2.14132037405479, -1.34402701877044, -2.32492500904728,
         4.46671289205392, -2.04800110524830,  0.44422406067087,
        -4.92212517643478, -1.73734240529793,  0.96890323821450,
        -1.30966093045696, -0.52977363497805,  3.44453452239668,
        -4.34208759006189, -4.30470270977329,  0.39887431726215,
         0.61788392767516,  2.62484136683297, -3.28228926932647,
         4.23562873444840, -1.68839322682951, -3.53824299552792,
         2.23130060612446,  1.93579813100155, -1.80384647554323,
        -2.32285463652832,  2.90603947535842, -1.39684847191937,
         2.34557941578250,  2.86074312333371,  1.82827238641666,
        -3.66431367659153, -0.42910188232667, -1.81957402856634,
        -0.34927881505446, -1.75988134003940,  5.98017466326572,
         0.29500802281217, -2.00226104143537,  0.53023447931897,
         2.10449364205058, -0.56741404446633,  0.30975625014335,
        -1.59355304432499,  3.69176153150419,  2.87878226787916,
         4.34858700256050,  2.39171478113440, -2.61802993563738,
    ];

    // Python test uses custom D4 model with ga=2.0, gc=1.0
    let model = DFTD4Model::custom_d4(&numbers, &positions, 2.0, 1.0, 6.0, None, None, None);

    let param = DFTD4RationalDampingParamBuilder::default()
        .s8(0.0)
        .a1(0.42)
        .a2(5.65)
        .s9(2.0)
        .build()
        .unwrap()
        .new_param();
    let res = model.get_dispersion(&param, false);

    assert_abs_diff_eq!(res.energy, -0.008016697276824889, epsilon = thr);

    let res = model.get_dispersion(&param, true);

    assert_abs_diff_eq!(res.energy, -0.008016697276824889, epsilon = thr);
}

#[cfg(feature = "api-v3_2")]
fn test_pair_resolved() {
    // Calculate pairwise resolved dispersion energy for a molecule
    let thr = 1.0e-8;

    let numbers = vec![16, 16, 16, 16, 16, 16, 16, 16];
    #[rustfmt::skip]
    let positions = vec![
        -4.15128787379191,  1.71951973863958, -0.93066267097296,
        -4.15128787379191, -1.71951973863958,  0.93066267097296,
        -1.71951973863958, -4.15128787379191, -0.93066267097296,
         1.71951973863958, -4.15128787379191,  0.93066267097296,
         4.15128787379191, -1.71951973863958, -0.93066267097296,
         4.15128787379191,  1.71951973863958,  0.93066267097296,
         1.71951973863958,  4.15128787379191, -0.93066267097296,
        -1.71951973863958,  4.15128787379191,  0.93066267097296,
    ];
    #[rustfmt::skip]
    let pair_disp2 = vec![
        -0.00000000e-0, -5.80599854e-4, -4.74689854e-4, -2.11149449e-4, -1.63163128e-4, -2.11149449e-4, -4.74689854e-4, -5.80599854e-4,
        -5.80599854e-4, -0.00000000e-0, -5.80599854e-4, -4.74689854e-4, -2.11149449e-4, -1.63163128e-4, -2.11149449e-4, -4.74689854e-4,
        -4.74689854e-4, -5.80599854e-4, -0.00000000e-0, -5.80599854e-4, -4.74689854e-4, -2.11149449e-4, -1.63163128e-4, -2.11149449e-4,
        -2.11149449e-4, -4.74689854e-4, -5.80599854e-4, -0.00000000e-0, -5.80599854e-4, -4.74689854e-4, -2.11149449e-4, -1.63163128e-4,
        -1.63163128e-4, -2.11149449e-4, -4.74689854e-4, -5.80599854e-4, -0.00000000e-0, -5.80599854e-4, -4.74689854e-4, -2.11149449e-4,
        -2.11149449e-4, -1.63163128e-4, -2.11149449e-4, -4.74689854e-4, -5.80599854e-4, -0.00000000e-0, -5.80599854e-4, -4.74689854e-4,
        -4.74689854e-4, -2.11149449e-4, -1.63163128e-4, -2.11149449e-4, -4.74689854e-4, -5.80599854e-4, -0.00000000e-0, -5.80599854e-4,
        -5.80599854e-4, -4.74689854e-4, -2.11149449e-4, -1.63163128e-4, -2.11149449e-4, -4.74689854e-4, -5.80599854e-4, -0.00000000e-0,
    ];
    #[rustfmt::skip]
    let pair_disp3 = vec![
        0.00000000e-0, 3.39353850e-7, 8.74462839e-7, 1.17634100e-6, 9.86937310e-7, 1.17634100e-6, 8.74462839e-7, 3.39353850e-7,
        3.39353850e-7, 0.00000000e-0, 3.39353850e-7, 8.74462839e-7, 1.17634100e-6, 9.86937310e-7, 1.17634100e-6, 8.74462839e-7,
        8.74462839e-7, 3.39353850e-7, 0.00000000e-0, 3.39353850e-7, 8.74462839e-7, 1.17634100e-6, 9.86937310e-7, 1.17634100e-6,
        1.17634100e-6, 8.74462839e-7, 3.39353850e-7, 0.00000000e-0, 3.39353850e-7, 8.74462839e-7, 1.17634100e-6, 9.86937310e-7,
        9.86937310e-7, 1.17634100e-6, 8.74462839e-7, 3.39353850e-7, 0.00000000e-0, 3.39353850e-7, 8.74462839e-7, 1.17634100e-6,
        1.17634100e-6, 9.86937310e-7, 1.17634100e-6, 8.74462839e-7, 3.39353850e-7, 0.00000000e-0, 3.39353850e-7, 8.74462839e-7,
        8.74462839e-7, 1.17634100e-6, 9.86937310e-7, 1.17634100e-6, 8.74462839e-7, 3.39353850e-7, 0.00000000e-0, 3.39353850e-7,
        3.39353850e-7, 8.74462839e-7, 1.17634100e-6, 9.86937310e-7, 1.17634100e-6, 8.74462839e-7, 3.39353850e-7, 0.00000000e-0,
    ];

    let model = DFTD4Model::new(&numbers, &positions, None, None, None);

    let param = DFTD4RationalDampingParamBuilder::default()
        .s8(1.20065498)
        .a1(0.40085597)
        .a2(5.02928789)
        .build()
        .unwrap()
        .new_param();
    let res = model.get_pairwise_dispersion(&param);

    for (a, b) in res.pair_energy2.iter().zip(pair_disp2.iter()) {
        assert_abs_diff_eq!(a, b, epsilon = thr);
    }

    for (a, b) in res.pair_energy3.iter().zip(pair_disp3.iter()) {
        assert_abs_diff_eq!(a, b, epsilon = thr);
    }
}

#[cfg(feature = "api-v3_1")]
fn test_properties() {
    // Calculate dispersion related properties for a molecule
    let thr = 1.0e-7;

    let numbers = vec![7, 7, 7, 7, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    #[rustfmt::skip]
    let positions = vec![
        1.97420621099560, 1.97415497783241, 1.97424596974304,
        6.82182427659395, 2.87346383480995, 7.72099517560089,
        7.72104957181201, 6.82177051521773, 2.87336561318016,
        2.87343220660781, 7.72108897828386, 6.82187093171878,
        3.51863272100286, 2.63865333484548, 1.00652979981286,
        2.63877594964754, 1.00647313885594, 3.51882748086447,
        1.00639728563189, 3.51850454450845, 2.63869202592387,
        8.36624975982697, 2.20896711017229, 8.68870955681018,
        7.48639684558259, 3.84114715917956, 6.17640982573725,
        5.85401675167715, 1.32911569888797, 7.05654606696031,
        7.05646299938990, 5.85409590282274, 1.32879923864813,
        8.68882633853582, 8.36611541129785, 2.20894120662207,
        3.84121223226912, 6.17673669892998, 7.48629723649480,
        1.32897854262127, 7.05658604099926, 5.85414031368096,
        2.20884896069885, 8.68875820985799, 8.36643568423387,
        6.17659142004652, 7.48627051643848, 3.84109594690835,
    ];
    #[rustfmt::skip]
    let lattice = vec![
        9.69523775911749, 0.0, 0.0,
        0.0, 9.69523775911749, 0.0,
        0.0, 0.0, 9.69523775911749,
    ];
    #[rustfmt::skip]
    let cn = [
        2.5775156287150218,
        2.5775155620078536,
        2.5775157938667150,
        2.5775157704485387,
        0.8591731475439074,
        0.8591680526841657,
        0.8591744284869478,
        0.8591732359038715,
        0.8591678667769283,
        0.8591744593270527,
        0.8591684383407867,
        0.8591754863625011,
        0.8591751690053771,
        0.8591719636497469,
        0.8591686377934131,
        0.8591718691634261,
    ];
    #[rustfmt::skip]
    let charges = [
        -0.86974543285813199,
        -0.86974501326130316,
        -0.86974658178316333,
        -0.86974643466661739,
         0.28992010784471012,
         0.28989847135893759,
         0.28992738637932841,
         0.28992042841052362,
         0.28989746859382759,
         0.28992753735979965,
         0.28990032127437559,
         0.28993196014690176,
         0.28993044356403513,
         0.28991421277784613,
         0.28990119160907674,
         0.28991393324985348,
    ];
    #[rustfmt::skip]
    let alpha = [
        9.9853045768095097,
        9.9853025181287016,
        9.9853102162086920,
        9.9853094944097140,
        1.3513315505023076,
        1.3513939255952379,
        1.3513106323935196,
        1.3513306244831862,
        1.3513968091133417,
        1.3513101978580895,
        1.3513885997538553,
        1.3512974505020905,
        1.3513018164322617,
        1.3513485145877058,
        1.3513860914395939,
        1.3513493246534483,
    ];

    let model = DFTD4Model::new(&numbers, &positions, None, Some(&lattice), None);

    let res = model.get_properties();

    for (a, b) in res.cn.iter().zip(cn.iter()) {
        assert_abs_diff_eq!(a, b, epsilon = thr);
    }

    for (a, b) in res.charges.iter().zip(charges.iter()) {
        assert_abs_diff_eq!(a, b, epsilon = thr);
    }

    for (a, b) in res.alpha.iter().zip(alpha.iter()) {
        assert_abs_diff_eq!(a, b, epsilon = thr);
    }
}

fn test_error_model() {
    // Test loading an unknown method produces an error
    let param = DFTD4Param::load_rational_damping_f("UNKNOWN_METHOD", false);
    assert!(param.is_err_and(|e| e.to_string().contains("not known")));
}

#[test]
fn test() {
    test_rational_damping_noargs();
    test_structure();
    test_blypd4();
    #[cfg(feature = "api-v4_0")]
    test_tpssd4s();
    test_pbed4();
    #[cfg(feature = "api-v3_1")]
    test_r2scan3c();
    #[cfg(feature = "api-v3_2")]
    test_pair_resolved();
    #[cfg(feature = "api-v3_1")]
    test_properties();
    test_error_model();
}

fn main() {
    test_rational_damping_noargs();
    test_structure();
    test_blypd4();
    #[cfg(feature = "api-v4_0")]
    test_tpssd4s();
    test_pbed4();
    #[cfg(feature = "api-v3_1")]
    test_r2scan3c();
    #[cfg(feature = "api-v3_2")]
    test_pair_resolved();
    #[cfg(feature = "api-v3_1")]
    test_properties();
    test_error_model();
}

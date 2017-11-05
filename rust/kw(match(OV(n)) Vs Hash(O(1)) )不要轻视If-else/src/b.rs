#[derive(Debug,PartialEq)]
enum Kw {
V00,V01,V02,V03,V04,V05,V06,V07,V08,V09,V10,V11,V12,V13,V14,V15,V16,V17,V18,V19,V20,V21,V22,V23,V24,V25,
// V26,V27,V28,V29,V30,V31,V32,V33,V34,V35,V36,V37,V38,V39,V40,V41,V42,V43,V44,V45,V46,V47,V48,V49,V50,V51,V52,V53,V54,V55,V56,V57,V58,V59,V60,V61,V62,V63,V64,V65,V66,V67,V68,V69,V70,V71,V72,V73,V74,V75,V76,V77,V78,V79,V80,V81,V82,V83,V84,V85,V86,V87,V88,V89,V90,V91,V92,V93,V94,V95,V96,V97,V98,V99
}

fn vec_parse_kw(kw: &str)-> Option<&'static Kw> {
   let kws = &[("prifix_0_suffix",Kw::V00),("prifix_1_suffix",Kw::V01),("prifix_2_suffix",Kw::V02),("prifix_3_suffix",Kw::V03),("prifix_4_suffix",Kw::V04),("prifix_5_suffix",Kw::V05),("prifix_6_suffix",Kw::V06),("prifix_7_suffix",Kw::V07),("prifix_8_suffix",Kw::V08),("prifix_9_suffix",Kw::V09),("prifix_10_suffix",Kw::V10),("prifix_11_suffix",Kw::V11),("prifix_12_suffix",Kw::V12),("prifix_13_suffix",Kw::V13),("prifix_14_suffix",Kw::V14),("prifix_15_suffix",Kw::V15),("prifix_16_suffix",Kw::V16),("prifix_17_suffix",Kw::V17),("prifix_18_suffix",Kw::V18),("prifix_19_suffix",Kw::V19),
   ("prifix_20_suffix",Kw::V20),("prifix_21_suffix",Kw::V21),("prifix_22_suffix",Kw::V22),("prifix_23_suffix",Kw::V23),("prifix_24_suffix",Kw::V24),("prifix_25_suffix",Kw::V25),
//    ("prifix_26_suffix",Kw::V26),("prifix_27_suffix",Kw::V27),("prifix_28_suffix",Kw::V28),("prifix_29_suffix",Kw::V29),("prifix_30_suffix",Kw::V30),("prifix_31_suffix",Kw::V31),("prifix_32_suffix",Kw::V32),("prifix_33_suffix",Kw::V33),("prifix_34_suffix",Kw::V34),("prifix_35_suffix",Kw::V35),("prifix_36_suffix",Kw::V36),("prifix_37_suffix",Kw::V37),("prifix_38_suffix",Kw::V38),("prifix_39_suffix",Kw::V39),("prifix_40_suffix",Kw::V40),("prifix_41_suffix",Kw::V41),("prifix_42_suffix",Kw::V42),("prifix_43_suffix",Kw::V43),("prifix_44_suffix",Kw::V44),("prifix_45_suffix",Kw::V45),("prifix_46_suffix",Kw::V46),("prifix_47_suffix",Kw::V47),("prifix_48_suffix",Kw::V48),("prifix_49_suffix",Kw::V49),("prifix_50_suffix",Kw::V50),("prifix_51_suffix",Kw::V51),("prifix_52_suffix",Kw::V52),("prifix_53_suffix",Kw::V53),("prifix_54_suffix",Kw::V54),("prifix_55_suffix",Kw::V55),("prifix_56_suffix",Kw::V56),("prifix_57_suffix",Kw::V57),("prifix_58_suffix",Kw::V58),("prifix_59_suffix",Kw::V59),("prifix_60_suffix",Kw::V60),("prifix_61_suffix",Kw::V61),("prifix_62_suffix",Kw::V62),("prifix_63_suffix",Kw::V63),("prifix_64_suffix",Kw::V64),("prifix_65_suffix",Kw::V65),("prifix_66_suffix",Kw::V66),("prifix_67_suffix",Kw::V67),("prifix_68_suffix",Kw::V68),("prifix_69_suffix",Kw::V69),("prifix_70_suffix",Kw::V70),("prifix_71_suffix",Kw::V71),("prifix_72_suffix",Kw::V72),("prifix_73_suffix",Kw::V73),("prifix_74_suffix",Kw::V74),("prifix_75_suffix",Kw::V75),("prifix_76_suffix",Kw::V76),("prifix_77_suffix",Kw::V77),("prifix_78_suffix",Kw::V78),("prifix_79_suffix",Kw::V79),("prifix_80_suffix",Kw::V80),("prifix_81_suffix",Kw::V81),("prifix_82_suffix",Kw::V82),("prifix_83_suffix",Kw::V83),("prifix_84_suffix",Kw::V84),("prifix_85_suffix",Kw::V85),("prifix_86_suffix",Kw::V86),("prifix_87_suffix",Kw::V87),("prifix_88_suffix",Kw::V88),("prifix_89_suffix",Kw::V89),("prifix_90_suffix",Kw::V90),("prifix_91_suffix",Kw::V91),("prifix_92_suffix",Kw::V92),("prifix_93_suffix",Kw::V93),("prifix_94_suffix",Kw::V94),("prifix_95_suffix",Kw::V95),("prifix_96_suffix",Kw::V96),("prifix_97_suffix",Kw::V97),("prifix_98_suffix",Kw::V98),("prifix_99_suffix",Kw::V99)
   ][..];
   kws.iter().find(|kv|kv.0==kw ).map(|ref kv|&kv.1)
}

#[bench]
fn vec_well_one(b: &mut test::Bencher) {
    b.iter(|| {
        [("prifix_0_suffix",Some(& Kw::V00)),].into_iter().for_each(|e|assert_eq!( vec_parse_kw(e.0),e.1))
    })
}
#[bench]
fn vec_badly_one(b: &mut test::Bencher) {
    b.iter(|| {
        [("prifix_25_suffix.",None),].into_iter().for_each(|e|assert_eq!( vec_parse_kw(e.0),e.1))
    })
}
#[bench]
fn vec_badly_one_long(b: &mut test::Bencher) {
    b.iter(|| {
        [("prifix_25_suffix.zncmzulkambb7982hoiHsbxajysnzmzyixhhhhhhhhhhhhhhhhxabxmmxvyauwfcg",None),].into_iter().for_each(|e|assert_eq!( vec_parse_kw(e.0),e.1))
    })
}
use super::test;
use super::phf;
fn phf_parse_kw(kw: &str)-> Option<&'static Kw> {
    static KWS: phf::Map<&'static str, &'static Kw> = phf_map!{
"prifix_0_suffix" => &Kw::V00,"prifix_1_suffix" => &Kw::V01,"prifix_2_suffix" => &Kw::V02,"prifix_3_suffix" => &Kw::V03,"prifix_4_suffix" => &Kw::V04,"prifix_5_suffix" => &Kw::V05,"prifix_6_suffix" => &Kw::V06,"prifix_7_suffix" => &Kw::V07,"prifix_8_suffix" => &Kw::V08,"prifix_9_suffix" => &Kw::V09,"prifix_10_suffix" => &Kw::V10,"prifix_11_suffix" => &Kw::V11,"prifix_12_suffix" => &Kw::V12,"prifix_13_suffix" => &Kw::V13,"prifix_14_suffix" => &Kw::V14,"prifix_15_suffix" => &Kw::V15,"prifix_16_suffix" => &Kw::V16,"prifix_17_suffix" => &Kw::V17,"prifix_18_suffix" => &Kw::V18,"prifix_19_suffix" => &Kw::V19,
"prifix_20_suffix" => &Kw::V20,"prifix_21_suffix" => &Kw::V21,"prifix_22_suffix" => &Kw::V22,"prifix_23_suffix" => &Kw::V23,"prifix_24_suffix" => &Kw::V24,"prifix_25_suffix" => &Kw::V25,
// "prifix_26_suffix" => &Kw::V26,"prifix_27_suffix" => &Kw::V27,"prifix_28_suffix" => &Kw::V28,"prifix_29_suffix" => &Kw::V29,"prifix_30_suffix" => &Kw::V30,"prifix_31_suffix" => &Kw::V31,"prifix_32_suffix" => &Kw::V32,"prifix_33_suffix" => &Kw::V33,"prifix_34_suffix" => &Kw::V34,"prifix_35_suffix" => &Kw::V35,"prifix_36_suffix" => &Kw::V36,"prifix_37_suffix" => &Kw::V37,"prifix_38_suffix" => &Kw::V38,"prifix_39_suffix" => &Kw::V39,"prifix_40_suffix" => &Kw::V40,"prifix_41_suffix" => &Kw::V41,"prifix_42_suffix" => &Kw::V42,"prifix_43_suffix" => &Kw::V43,"prifix_44_suffix" => &Kw::V44,"prifix_45_suffix" => &Kw::V45,"prifix_46_suffix" => &Kw::V46,"prifix_47_suffix" => &Kw::V47,"prifix_48_suffix" => &Kw::V48,"prifix_49_suffix" => &Kw::V49,"prifix_50_suffix" => &Kw::V50,"prifix_51_suffix" => &Kw::V51,"prifix_52_suffix" => &Kw::V52,"prifix_53_suffix" => &Kw::V53,"prifix_54_suffix" => &Kw::V54,"prifix_55_suffix" => &Kw::V55,"prifix_56_suffix" => &Kw::V56,"prifix_57_suffix" => &Kw::V57,"prifix_58_suffix" => &Kw::V58,"prifix_59_suffix" => &Kw::V59,"prifix_60_suffix" => &Kw::V60,"prifix_61_suffix" => &Kw::V61,"prifix_62_suffix" => &Kw::V62,"prifix_63_suffix" => &Kw::V63,"prifix_64_suffix" => &Kw::V64,"prifix_65_suffix" => &Kw::V65,"prifix_66_suffix" => &Kw::V66,"prifix_67_suffix" => &Kw::V67,"prifix_68_suffix" => &Kw::V68,"prifix_69_suffix" => &Kw::V69,"prifix_70_suffix" => &Kw::V70,"prifix_71_suffix" => &Kw::V71,"prifix_72_suffix" => &Kw::V72,"prifix_73_suffix" => &Kw::V73,"prifix_74_suffix" => &Kw::V74,"prifix_75_suffix" => &Kw::V75,"prifix_76_suffix" => &Kw::V76,"prifix_77_suffix" => &Kw::V77,"prifix_78_suffix" => &Kw::V78,"prifix_79_suffix" => &Kw::V79,"prifix_80_suffix" => &Kw::V80,"prifix_81_suffix" => &Kw::V81,"prifix_82_suffix" => &Kw::V82,"prifix_83_suffix" => &Kw::V83,"prifix_84_suffix" => &Kw::V84,"prifix_85_suffix" => &Kw::V85,"prifix_86_suffix" => &Kw::V86,"prifix_87_suffix" => &Kw::V87,"prifix_88_suffix" => &Kw::V88,"prifix_89_suffix" => &Kw::V89,"prifix_90_suffix" => &Kw::V90,"prifix_91_suffix" => &Kw::V91,"prifix_92_suffix" => &Kw::V92,"prifix_93_suffix" => &Kw::V93,"prifix_94_suffix" => &Kw::V94,"prifix_95_suffix" => &Kw::V95,"prifix_96_suffix" => &Kw::V96,"prifix_97_suffix" => &Kw::V97,"prifix_98_suffix" => &Kw::V98,"prifix_99_suffix" => &Kw::V99
    };
    KWS.get(kw).map(|x|*x )
}

#[bench]
fn phf_badly_one(b: &mut test::Bencher) {
    b.iter(|| {
        [("prifix_25_suffix.",None),].into_iter().for_each(|e|assert_eq!( phf_parse_kw(e.0),e.1))
    })
}
#[bench]
fn phf_badly_one_long(b: &mut test::Bencher) {
    b.iter(|| {
        [("prifix_25_suffix.zncmzulkambb7982hoiHsbxajysnzmzyixhhhhhhhhhhhhhhhhxabxmmxvyauwfcg",None),].into_iter().for_each(|e|assert_eq!( phf_parse_kw(e.0),e.1))
    })
}
#[bench]
fn phf_well_one(b: &mut test::Bencher) {
    b.iter(|| {
        [("prifix_0_suffix",Some(& Kw::V00)),].into_iter().for_each(|e|assert_eq!( phf_parse_kw(e.0),e.1))
    })
}


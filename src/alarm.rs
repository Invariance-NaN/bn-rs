use crate::graph::Digraph;

pub fn alarm() -> Digraph {
    let mut result = Digraph::unconnected([
        "HISTORY",
        "CVP",
        "PCWP",
        "HYPOVOLEMIA",
        "LVEDVOLUME",
        "LVFAILURE",
        "STROKEVOLUME",
        "ERRLOWOUTPUT",
        "HRBP",
        "HREKG",
        "ERRCAUTER",
        "HRSAT",
        "INSUFFANESTH",
        "ANAPHYLAXIS",
        "TPR",
        "EXPCO2",
        "KINKEDTUBE",
        "MINVOL",
        "FIO2",
        "PVSAT",
        "SAO2",
        "PAP",
        "PULMEMBOLUS",
        "SHUNT",
        "INTUBATION",
        "PRESS",
        "DISCONNECT",
        "MINVOLSET",
        "VENTMACH",
        "VENTTUBE",
        "VENTLUNG",
        "VENTALV",
        "ARTCO2",
        "CATECHOL",
        "HR",
        "CO",
        "BP"
    ].iter().map(|x| x.to_string()).collect());

    result.add_edge( 0, 5);
    result.add_edge( 1, 4);
    result.add_edge( 2, 4);
    result.add_edge( 4, 3);
    result.add_edge( 4, 5);
    result.add_edge( 6, 3);
    result.add_edge( 6, 5);
    result.add_edge( 8, 7);
    result.add_edge( 8, 3);
    result.add_edge( 9, 1);
    result.add_edge( 9, 3);
    result.add_edge(11, 1);
    result.add_edge(11, 3);
    result.add_edge(14, 1);
    result.add_edge(15, 3);
    result.add_edge(15, 3);
    result.add_edge(17, 2);
    result.add_edge(17, 3);
    result.add_edge(19, 1);
    result.add_edge(19, 3);
    result.add_edge(20, 1);
    result.add_edge(20, 2);
    result.add_edge(21, 2);
    result.add_edge(23, 2);
    result.add_edge(23, 2);
    result.add_edge(25, 2);
    result.add_edge(25, 1);
    result.add_edge(25, 2);
    result.add_edge(28, 2);
    result.add_edge(29, 2);
    result.add_edge(29, 2);
    result.add_edge(30, 2);
    result.add_edge(30, 1);
    result.add_edge(30, 2);
    result.add_edge(31, 2);
    result.add_edge(31, 3);
    result.add_edge(32, 3);
    result.add_edge(33, 3);
    result.add_edge(33, 1);
    result.add_edge(33, 2);
    result.add_edge(33, 1);
    result.add_edge(34, 3);
    result.add_edge(35, 3);
    result.add_edge(35, 6);
    result.add_edge(36, 3);
    result.add_edge(36, 1);

    result
}

pub fn a() -> Digraph {
    let mut result = Digraph::unconnected([
        "GOAL_2",
        "SNode_3",
        "SNode_4",
        "SNode_5",
        "SNode_6",
        "SNode_7",
        "DISPLACEM0",
        "RApp1",
        "GIVEN_1",
        "RApp2",
        "SNode_8",
        "SNode_9",
        "SNode_10",
        "SNode_11",
        "SNode_12",
        "SNode_13",
        "SNode_14",
        "SNode_15",
        "SNode_16",
        "SNode_17",
        "SNode_18",
        "SNode_19",
        "NEED1",
        "SNode_20",
        "GRAV2",
        "SNode_21",
        "VALUE3",
        "SNode_24",
        "SLIDING4",
        "SNode_25",
        "CONSTANT5",
        "SNode_26",
        "KNOWN6",
        "VELOCITY7",
        "SNode_47",
        "RApp3",
        "KNOWN8",
        "RApp4",
        "SNode_27",
        "COMPO16",
        "GOAL_48",
        "TRY12",
        "TRY11",
        "GOAL_49",
        "CHOOSE19",
        "GOAL_50",
        "SYSTEM18",
        "SNode_51",
        "KINEMATI17",
        "SNode_52",
        "IDENTIFY10",
        "GOAL_53",
        "IDENTIFY9",
        "SNode_28",
        "TRY13",
        "TRY14",
        "TRY15",
        "VAR20",
        "SNode_29",
        "SNode_31",
        "GIVEN21",
        "SNode_33",
        "SNode_34",
        "VECTOR27",
        "APPLY32",
        "GOAL_56",
        "CHOOSE35",
        "GOAL_57",
        "MAXIMIZE34",
        "SNode_59",
        "AXIS33",
        "SNode_60",
        "WRITE31",
        "GOAL_61",
        "WRITE30",
        "GOAL_62",
        "RESOLVE37",
        "GOAL_63",
        "NEED36",
        "SNode_64",
        "SNode_41",
        "SNode_42",
        "IDENTIFY39",
        "SNode_43",
        "RESOLVE38",
        "GOAL_66",
        "SNode_67",
        "IDENTIFY41",
        "SNode_54",
        "RESOLVE40",
        "GOAL_69",
        "SNode_70",
        "IDENTIFY43",
        "SNode_55",
        "RESOLVE42",
        "GOAL_72",
        "SNode_73",
        "KINE29",
        "SNode_74",
        "VECTOR44",
        "SNode_75",
        "EQUATION28",
        "GOAL_79",
        "RApp5",
        "GOAL_80",
        "RApp6",
        "GOAL_81",
        "TRY25",
        "TRY24",
        "GOAL_83",
        "CHOOSE47",
        "GOAL_84",
        "SYSTEM46",
        "SNode_86",
        "NEWTONS45",
        "SNode_156",
        "DEFINE23",
        "GOAL_98",
        "IDENTIFY22",
        "SNode_37",
        "TRY26",
        "SNode_38",
        "SNode_40",
        "SNode_44",
        "SNode_46",
        "NULL48",
        "SNode_65",
        "SNode_68",
        "SNode_71",
        "FIND49",
        "GOAL_87",
        "NORMAL50",
        "SNode_88",
        "STRAT_90",
        "NORMAL52",
        "INCLINE51",
        "SNode_91",
        "HORIZ53",
        "BUGGY54",
        "SNode_92",
        "IDENTIFY55",
        "SNode_93",
        "WEIGHT56",
        "SNode_94",
        "WEIGHT57",
        "SNode_95",
        "SNode_97",
        "FIND58",
        "GOAL_99",
        "IDENTIFY59",
        "SNode_100",
        "FORCE60",
        "SNode_102",
        "APPLY61",
        "GOAL_103",
        "CHOOSE62",
        "GOAL_104",
        "SNode_106",
        "SNode_152",
        "WRITE63",
        "GOAL_107",
        "WRITE64",
        "GOAL_108",
        "GOAL_109",
        "GOAL65",
        "GOAL_110",
        "GOAL66",
        "GOAL_111",
        "NEED67",
        "RApp7",
        "RApp8",
        "SNode_112",
        "GOAL68",
        "GOAL_113",
        "GOAL_114",
        "SNode_115",
        "VECTOR69",
        "SNode_116",
        "SNode_117",
        "VECTOR70",
        "SNode_118",
        "EQUAL71",
        "SNode_119",
        "SNode_120",
        "GOAL72",
        "GOAL_121",
        "SNode_122",
        "VECTOR73",
        "SNode_123",
        "NEWTONS74",
        "SNode_124",
        "SUM75",
        "SNode_125",
        "GOAL_126",
        "GOAL_127",
        "RApp9",
        "RApp10",
        "SNode_128",
        "GOAL_129",
        "GOAL_130",
        "SNode_131",
        "SNode_132",
        "SNode_133",
        "SNode_134",
        "SNode_135",
        "SNode_154",
        "SNode_136",
        "SNode_137",
        "GOAL_142",
        "GOAL_143",
        "GOAL_146",
        "RApp11",
        "RApp12",
        "RApp13",
        "GOAL_147",
        "TRY76",
        "GOAL_149",
        "APPLY77",
        "GOAL_150",
        "GRAV78",
        "SNode_151",
        "GOAL_153",
        "SNode_155"
    ].iter().map(|x| x.to_string()).collect());

    result.add_edge(7, 6);
    result.add_edge(7, 1);
    result.add_edge(9, 8);
    result.add_edge(10, 7);
    result.add_edge(10, 9);
    result.add_edge(23, 18);
    result.add_edge(23, 22);
    result.add_edge(25, 23);
    result.add_edge(25, 24);
    result.add_edge(27, 25);
    result.add_edge(27, 26);
    result.add_edge(29, 17);
    result.add_edge(29, 28);
    result.add_edge(31, 13);
    result.add_edge(31, 30);
    result.add_edge(34, 1);
    result.add_edge(34, 33);
    result.add_edge(35, 32);
    result.add_edge(35, 31);
    result.add_edge(35, 34);
    result.add_edge(37, 36);
    result.add_edge(37, 13);
    result.add_edge(38, 35);
    result.add_edge(38, 37);
    result.add_edge(40, 0);
    result.add_edge(40, 39);
    result.add_edge(42, 41);
    result.add_edge(43, 3);
    result.add_edge(43, 4);
    result.add_edge(43, 40);
    result.add_edge(43, 42);
    result.add_edge(45, 43);
    result.add_edge(45, 44);
    result.add_edge(47, 19);
    result.add_edge(47, 45);
    result.add_edge(47, 46);
    result.add_edge(49, 47);
    result.add_edge(49, 48);
    result.add_edge(51, 43);
    result.add_edge(51, 49);
    result.add_edge(51, 50);
    result.add_edge(53, 38);
    result.add_edge(53, 51);
    result.add_edge(53, 52);
    result.add_edge(54, 41);
    result.add_edge(55, 41);
    result.add_edge(56, 41);
    result.add_edge(58, 53);
    result.add_edge(58, 57);
    result.add_edge(59, 58);
    result.add_edge(59, 26);
    result.add_edge(61, 12);
    result.add_edge(61, 60);
    result.add_edge(62, 12);
    result.add_edge(62, 30);
    result.add_edge(65, 43);
    result.add_edge(65, 49);
    result.add_edge(65, 64);
    result.add_edge(67, 65);
    result.add_edge(67, 66);
    result.add_edge(69, 5);
    result.add_edge(69, 67);
    result.add_edge(69, 68);
    result.add_edge(71, 69);
    result.add_edge(71, 70);
    result.add_edge(73, 65);
    result.add_edge(73, 71);
    result.add_edge(73, 72);
    result.add_edge(75, 73);
    result.add_edge(75, 74);
    result.add_edge(77, 53);
    result.add_edge(77, 75);
    result.add_edge(77, 76);
    result.add_edge(79, 77);
    result.add_edge(79, 78);
    result.add_edge(80, 11);
    result.add_edge(80, 30);
    result.add_edge(81, 10);
    result.add_edge(81, 80);
    result.add_edge(81, 32);
    result.add_edge(83, 81);
    result.add_edge(83, 51);
    result.add_edge(83, 82);
    result.add_edge(85, 83);
    result.add_edge(85, 75);
    result.add_edge(85, 84);
    result.add_edge(86, 85);
    result.add_edge(86, 78);
    result.add_edge(88, 51);
    result.add_edge(88, 87);
    result.add_edge(90, 88);
    result.add_edge(90, 75);
    result.add_edge(90, 89);
    result.add_edge(91, 90);
    result.add_edge(91, 78);
    result.add_edge(93, 62);
    result.add_edge(93, 51);
    result.add_edge(93, 92);
    result.add_edge(95, 93);
    result.add_edge(95, 75);
    result.add_edge(95, 94);
    result.add_edge(96, 95);
    result.add_edge(96, 78);
    result.add_edge(98, 75);
    result.add_edge(98, 79);
    result.add_edge(98, 86);
    result.add_edge(98, 91);
    result.add_edge(98, 96);
    result.add_edge(98, 97);
    result.add_edge(100, 2);
    result.add_edge(100, 95);
    result.add_edge(100, 96);
    result.add_edge(100, 99);
    result.add_edge(102, 98);
    result.add_edge(102, 100);
    result.add_edge(102, 101);
    result.add_edge(103, 63);
    result.add_edge(103, 102);
    result.add_edge(104, 100);
    result.add_edge(104, 101);
    result.add_edge(105, 39);
    result.add_edge(105, 104);
    result.add_edge(106, 103);
    result.add_edge(106, 105);
    result.add_edge(108, 107);
    result.add_edge(109, 106);
    result.add_edge(109, 108);
    result.add_edge(111, 109);
    result.add_edge(111, 110);
    result.add_edge(113, 111);
    result.add_edge(113, 112);
    result.add_edge(115, 113);
    result.add_edge(115, 114);
    result.add_edge(117, 109);
    result.add_edge(117, 115);
    result.add_edge(117, 116);
    result.add_edge(119, 117);
    result.add_edge(119, 118);
    result.add_edge(120, 107);
    result.add_edge(121, 119);
    result.add_edge(121, 57);
    result.add_edge(122, 121);
    result.add_edge(122, 26);
    result.add_edge(123, 83);
    result.add_edge(123, 57);
    result.add_edge(124, 123);
    result.add_edge(124, 26);
    result.add_edge(126, 58);
    result.add_edge(126, 77);
    result.add_edge(126, 79);
    result.add_edge(126, 125);
    result.add_edge(127, 85);
    result.add_edge(127, 86);
    result.add_edge(127, 99);
    result.add_edge(128, 90);
    result.add_edge(128, 91);
    result.add_edge(128, 99);
    result.add_edge(130, 109);
    result.add_edge(130, 115);
    result.add_edge(130, 129);
    result.add_edge(132, 29);
    result.add_edge(132, 130);
    result.add_edge(132, 131);
    result.add_edge(135, 134);
    result.add_edge(136, 132);
    result.add_edge(136, 14);
    result.add_edge(136, 15);
    result.add_edge(136, 133);
    result.add_edge(136, 135);
    result.add_edge(137, 134);
    result.add_edge(138, 134);
    result.add_edge(139, 14);
    result.add_edge(139, 133);
    result.add_edge(139, 138);
    result.add_edge(141, 130);
    result.add_edge(141, 132);
    result.add_edge(141, 140);
    result.add_edge(143, 18);
    result.add_edge(143, 61);
    result.add_edge(143, 130);
    result.add_edge(143, 142);
    result.add_edge(145, 143);
    result.add_edge(145, 144);
    result.add_edge(146, 130);
    result.add_edge(146, 143);
    result.add_edge(146, 140);
    result.add_edge(148, 117);
    result.add_edge(148, 147);
    result.add_edge(150, 117);
    result.add_edge(150, 149);
    result.add_edge(152, 130);
    result.add_edge(152, 132);
    result.add_edge(152, 143);
    result.add_edge(152, 151);
    result.add_edge(154, 109);
    result.add_edge(154, 152);
    result.add_edge(154, 153);
    result.add_edge(156, 154);
    result.add_edge(156, 155);
    result.add_edge(157, 156);
    result.add_edge(157, 68);
    result.add_edge(158, 157);
    result.add_edge(158, 70);
    result.add_edge(160, 154);
    result.add_edge(160, 158);
    result.add_edge(160, 159);
    result.add_edge(162, 160);
    result.add_edge(162, 161);
    result.add_edge(163, 160);
    result.add_edge(163, 161);
    result.add_edge(165, 163);
    result.add_edge(165, 164);
    result.add_edge(167, 163);
    result.add_edge(167, 166);
    result.add_edge(169, 168);
    result.add_edge(169, 163);
    result.add_edge(170, 78);
    result.add_edge(170, 167);
    result.add_edge(171, 169);
    result.add_edge(171, 170);
    result.add_edge(173, 165);
    result.add_edge(173, 172);
    result.add_edge(174, 165);
    result.add_edge(174, 172);
    result.add_edge(175, 174);
    result.add_edge(175, 78);
    result.add_edge(177, 145);
    result.add_edge(177, 146);
    result.add_edge(177, 174);
    result.add_edge(177, 175);
    result.add_edge(177, 176);
    result.add_edge(178, 173);
    result.add_edge(178, 78);
    result.add_edge(180, 136);
    result.add_edge(180, 173);
    result.add_edge(180, 179);
    result.add_edge(182, 141);
    result.add_edge(182, 178);
    result.add_edge(182, 180);
    result.add_edge(182, 181);
    result.add_edge(183, 139);
    result.add_edge(183, 141);
    result.add_edge(183, 173);
    result.add_edge(183, 178);
    result.add_edge(183, 176);
    result.add_edge(185, 163);
    result.add_edge(185, 184);
    result.add_edge(186, 185);
    result.add_edge(186, 78);
    result.add_edge(188, 2);
    result.add_edge(188, 150);
    result.add_edge(188, 185);
    result.add_edge(188, 186);
    result.add_edge(188, 187);
    result.add_edge(190, 119);
    result.add_edge(190, 163);
    result.add_edge(190, 171);
    result.add_edge(190, 186);
    result.add_edge(190, 189);
    result.add_edge(192, 163);
    result.add_edge(192, 171);
    result.add_edge(192, 191);
    result.add_edge(193, 162);
    result.add_edge(193, 164);
    result.add_edge(194, 162);
    result.add_edge(194, 166);
    result.add_edge(195, 168);
    result.add_edge(195, 162);
    result.add_edge(196, 78);
    result.add_edge(196, 194);
    result.add_edge(197, 195);
    result.add_edge(197, 196);
    result.add_edge(198, 193);
    result.add_edge(198, 172);
    result.add_edge(199, 193);
    result.add_edge(199, 172);
    result.add_edge(200, 199);
    result.add_edge(200, 78);
    result.add_edge(201, 145);
    result.add_edge(201, 146);
    result.add_edge(201, 199);
    result.add_edge(201, 200);
    result.add_edge(201, 176);
    result.add_edge(202, 198);
    result.add_edge(202, 78);
    result.add_edge(203, 136);
    result.add_edge(203, 141);
    result.add_edge(203, 198);
    result.add_edge(203, 202);
    result.add_edge(203, 187);
    result.add_edge(204, 139);
    result.add_edge(204, 141);
    result.add_edge(204, 198);
    result.add_edge(204, 202);
    result.add_edge(204, 176);
    result.add_edge(205, 185);
    result.add_edge(205, 78);
    result.add_edge(206, 119);
    result.add_edge(206, 162);
    result.add_edge(206, 197);
    result.add_edge(206, 205);
    result.add_edge(206, 189);
    result.add_edge(207, 162);
    result.add_edge(207, 197);
    result.add_edge(207, 191);
    result.add_edge(208, 177);
    result.add_edge(208, 192);
    result.add_edge(208, 101);
    result.add_edge(209, 177);
    result.add_edge(209, 201);
    result.add_edge(209, 101);
    result.add_edge(210, 201);
    result.add_edge(210, 207);
    result.add_edge(210, 101);
    result.add_edge(211, 63);
    result.add_edge(211, 208);
    result.add_edge(212, 39);
    result.add_edge(212, 209);
    result.add_edge(213, 63);
    result.add_edge(213, 210);
    result.add_edge(214, 211);
    result.add_edge(214, 212);
    result.add_edge(214, 213);
    result.add_edge(216, 214);
    result.add_edge(216, 215);
    result.add_edge(218, 23);
    result.add_edge(218, 119);
    result.add_edge(218, 216);
    result.add_edge(218, 217);
    result.add_edge(220, 218);
    result.add_edge(220, 219);
    result.add_edge(221, 162);
    result.add_edge(221, 184);
    result.add_edge(222, 2);
    result.add_edge(222, 150);
    result.add_edge(222, 221);
    result.add_edge(222, 205);
    result.add_edge(222, 99);

    result
}

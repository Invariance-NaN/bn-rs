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
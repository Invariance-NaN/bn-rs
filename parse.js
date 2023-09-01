const fs = require("fs");

const name = "munin";

const handle_bif = function(bif) {
	const variables = Array.from(new Set(
		Array.from(bif.matchAll(/variable ([^ ]*)/g)).map(e => e[1])
	));

	const deps = Array.from(bif.matchAll(/probability \((.*?)\)/g))
		.map(e => e[1])
		.filter(e => e.includes("|"))
		.map(e => e.split(" | ").map(f => f.trim()))
		.map(e => [variables.indexOf(e[0]), e[1].split(",").map(f => variables.indexOf(f.trim()))])
		.flatMap(e => e[1].map(f => [e[0], f]));

	const result =
`
pub fn ${name}() -> Digraph {
    let mut result = Digraph::unconnected([
        ${variables.map(e => `        "${e}"`).join(",\n")}
    ].iter().map(|x| x.to_string()).collect());

	${deps.map(([from, to]) => `    result.add_edge(${from}, ${to});`).join("\n")}

    result
}
`;

	fs.writeFileSync(`${name}.rs`, result);
}

const bif = fs.readFileSync(`${name}.bif`, { encoding: "utf-8" });
handle_bif(bif);

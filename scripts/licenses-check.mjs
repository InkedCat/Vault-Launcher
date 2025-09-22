const stdin = process.openStdin();

const cliArguments = [];
process.argv.forEach((val, index) => {
	if (index > 1) {
		cliArguments.push(val);
	}
});

const pattern = /\b(?!OR\b)[^()\s]+\b/g;

let data = '';

const openSourceDependencyLicenses = [
	'0BSD',
	'Apache-2.0',
	'BlueOak-1.0.0',
	'BSD-1-Clause',
	'BSD-2-Clause',
	'BSD-3-Clause',
	'CC-BY-4.0',
	'MIT',
	'MIT-0',
	'ISC',
	'MPL-2.0',
	'Python-2.0',
	'Unlicense',
	'UPL-1.0',
	'Zlib'
];

stdin.on('data', function (chunk) {
	data += chunk;
});

function getPacketDependenciesLicense(dependencies, licenseMap) {
	for (const [key, value] of Object.entries(dependencies)) {
		if (!value.license) {
			licenseMap.get('Unknown').add(`${key} - ${value.version}`);
		} else {
			const licenses = value.license.match(pattern) || ['Unknown'];
			licenses.forEach((license) => {
				if (!licenseMap.has(license)) {
					licenseMap.set(license, new Set());
				}
				licenseMap.get(license).add(`${key} - ${value.version}`);
			});
		}

		if (value.dependencies) {
			getPacketDependenciesLicense(value.dependencies, licenseMap);
		}
	}
}

function coloredLog(message, color) {
	const colors = {
		red: '\x1b[31m',
		green: '\x1b[32m',
		blue: '\x1b[34m',
		orange: '\x1b[33m',
		reset: '\x1b[0m'
	};

	console.log(`${colors[color] || colors.reset}${message}${colors.reset}`);
}

function listDependencies(licenseMap) {
	for (const [key, value] of licenseMap) {
		const message = `${key}: ${value.size} |`;
		coloredLog(message, 'blue');
		coloredLog('-'.repeat(message.length), 'blue');

		for (const item of value) {
			const color = openSourceDependencyLicenses.includes(key) ? 'green' : 'red';
			coloredLog(`    ${item}`, color);
		}

		console.log('\n');
	}
}

function listIncompatibleDependencies(licenseMap) {
	let incompatibleDependencies = 0;
	for (const [key, value] of licenseMap) {
		if (openSourceDependencyLicenses.includes(key)) {
			continue;
		}

		const message = `${key}: ${value.size} |`;
		coloredLog(message, 'red');
		coloredLog('-'.repeat(message.length), 'red');

		for (const item of value) {
			coloredLog(`    ${item}`, 'orange');
		}

		console.log('\n');
		incompatibleDependencies += value.size;
	}

	return incompatibleDependencies === 0;
}

function listUnknownDependencies(dependencies) {
	const message = `Unknown: ${dependencies.size} |`;
	coloredLog(message, 'red');
	coloredLog('-'.repeat(message.length), 'red');

	for (const dependency of dependencies) {
		coloredLog(`    ${dependency}`, 'orange');
	}
	console.log('\n');
}

stdin.on('end', function () {
	const dependencies = JSON.parse(data)[0].dependencies;
	const devDependencies = JSON.parse(data)[0].devDependencies;

	const prodLicenseMap = new Map();
	prodLicenseMap.set('Unknown', new Set());
	getPacketDependenciesLicense(dependencies, prodLicenseMap);

	if (cliArguments.includes('--unknown') && prodLicenseMap.get('Unknown').size > 0) {
		coloredLog('Unknown licenses in Production Dependencies:', 'red');
		listUnknownDependencies(prodLicenseMap.get('Unknown'));
	}
	prodLicenseMap.delete('Unknown');

	if (!cliArguments.includes('check')) {
		coloredLog('Production Dependencies licenses:', 'blue');
		listDependencies(prodLicenseMap);
	} else {
		coloredLog('Incompatible Production Dependencies licenses:', 'red');
		if (listIncompatibleDependencies(prodLicenseMap)) {
			coloredLog('    No incompatible Production Dependencies licenses found.\n', 'green');
		}
	}

	console.log('\n');

	const devLicenseMap = new Map();
	devLicenseMap.set('Unknown', new Set());
	getPacketDependenciesLicense(devDependencies, devLicenseMap);

	if (cliArguments.includes('--unknown') && devLicenseMap.get('Unknown').size > 0) {
		coloredLog('Unknown licenses in Development Dependencies:', 'red');
		listUnknownDependencies(devLicenseMap.get('Unknown'));
	}

	devLicenseMap.delete('Unknown');

	if (!cliArguments.includes('check')) {
		coloredLog('Development Dependencies licenses:', 'blue');
		listDependencies(devLicenseMap);
	} else {
		coloredLog('Incompatible Development Dependencies licenses:', 'red');
		if (listIncompatibleDependencies(devLicenseMap)) {
			coloredLog('    No incompatible Development Dependencies licenses found.\n', 'green');
		}
	}
});
